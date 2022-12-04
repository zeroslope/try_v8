use v8::HandleScope;

pub struct JsExtension {}

impl JsExtension {
    pub fn install(handle_scope: &mut HandleScope) {
        let global = handle_scope.get_current_context().global(handle_scope);

        let name = v8::String::new(handle_scope, "print").unwrap();
        let func = v8::FunctionTemplate::new(handle_scope, print);
        let func = func.get_function(handle_scope).unwrap();
        global.set(handle_scope, name.into(), func.into());

        let name = v8::String::new(handle_scope, "fetch").unwrap();
        let func = v8::FunctionTemplate::new(handle_scope, fetch);
        let func = func.get_function(handle_scope).unwrap();
        global.set(handle_scope, name.into(), func.into());
    }
}

fn print(
    scope: &mut HandleScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    let res: serde_json::Value = serde_v8::from_v8(scope, args.get(0)).unwrap();
    println!("print args = {res:#?}");
    retval.set(serde_v8::to_v8(scope, res).unwrap());
}

fn fetch(
    scope: &mut HandleScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    let url: String = serde_v8::from_v8(scope, args.get(0)).unwrap();
    println!("fetch url = {url}");
    let body = reqwest::blocking::get(url)
        .and_then(|resp| resp.text())
        .unwrap();
    retval.set(serde_v8::to_v8(scope, body).unwrap());
}

use serde::{Deserialize, Serialize};
use v8::Script;

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub status: u32,
    pub message: String,
}

fn main() {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let params = v8::CreateParams::default();
    let isolate = &mut v8::Isolate::new(params);

    let handle_scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(handle_scope);
    let context_scope = &mut v8::ContextScope::new(handle_scope, context);

    let source = r#"
        function hello() {
            return {
                status: 200,
                message: "Hello World!",
            }
        }
        hello();
    "#;

    let code = v8::String::new(context_scope, source).unwrap();
    let script = Script::compile(context_scope, code, None).unwrap();
    let result = script.run(context_scope).unwrap();
    // let value: Data = serde_v8::from_v8(context_scope, result).unwrap();
    let value: serde_json::Value = serde_v8::from_v8(context_scope, result).unwrap();
    println!("Result is: {:?}", value);
}

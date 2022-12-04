use try_v8::runtime::{JsRuntime, JsRuntimeParams};

fn main() {
    JsRuntime::init();
    JsRuntime::init();

    let mut runtime = JsRuntime::new(JsRuntimeParams::default());
    let code = r#"
    let res = print({ a: 1, b: 2 });
    print(res);
    let resp = fetch("http://httpbin.org/get");
    JSON.parse(resp);
    "#;
    let result = runtime.execute_sciprt(code);
    println!("Result is: {:#?}", result);
}

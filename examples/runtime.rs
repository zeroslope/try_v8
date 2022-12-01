use try_v8::runtime::{JsRuntime, JsRuntimeParams};

fn main() {
    JsRuntime::init();

    let mut runtime = JsRuntime::new(JsRuntimeParams::default());
    let code = r#"
    function hello() {
        return {
            status: 200,
            message: "Hello World!",
        }
    }
    hello();
    let a = b.c;
    "#;
    let result = runtime.execute_sciprt(code);
    println!("Result is: {:#?}", result);
}

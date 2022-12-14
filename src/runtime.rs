// init v8
// new runtime(isolate) with config
// run code

use once_cell::sync::OnceCell;
use v8::{CreateParams, HandleScope, Isolate, OwnedIsolate, Script};

use crate::{extension::JsExtension, state::JsRuntimeState};

type LocalValue<'s> = v8::Local<'s, v8::Value>;

pub struct JsRuntime {
    isolate: OwnedIsolate,
}

#[derive(Default)]
pub struct JsRuntimeParams(CreateParams);

impl JsRuntimeParams {
    pub fn new(_snapshot: Option<Vec<u8>>) -> Self {
        JsRuntimeParams(CreateParams::default())
    }

    pub fn into_inner(self) -> CreateParams {
        self.0
    }
}

impl JsRuntime {
    pub fn init() {
        static INSTANCE: OnceCell<()> = OnceCell::new();
        INSTANCE.get_or_init(|| {
            let platform = v8::new_default_platform(0, false).make_shared();
            v8::V8::initialize_platform(platform);
            v8::V8::initialize();
        });
    }

    fn init_isolate(mut isolate: OwnedIsolate) -> Self {
        let state = JsRuntimeState::new(&mut isolate);
        isolate.set_slot(state);
        Self { isolate }
    }

    pub fn new(params: JsRuntimeParams) -> Self {
        let isolate = Isolate::new(params.into_inner());
        Self::init_isolate(isolate)
    }

    pub fn execute_sciprt(
        &mut self,
        code: impl AsRef<str>,
    ) -> Result<serde_json::Value, serde_json::Value> {
        let state = JsRuntimeState::get_context(&mut self.isolate);
        let handle_scope = &mut HandleScope::with_context(&mut self.isolate, state);
        JsExtension::install(handle_scope);
        match execute_sciprt(handle_scope, code) {
            Ok(v) => Ok(serde_v8::from_v8(handle_scope, v).unwrap()),
            Err(e) => Err(serde_v8::from_v8(handle_scope, e).unwrap()),
        }
    }

    pub fn create_snapshot() -> Vec<u8> {
        todo!()
    }
}

pub(crate) fn execute_sciprt<'s>(
    handle_scope: &mut HandleScope<'s>,
    code: impl AsRef<str>,
) -> Result<LocalValue<'s>, LocalValue<'s>> {
    let scope = &mut v8::TryCatch::new(handle_scope);
    let source = v8::String::new(scope, code.as_ref()).unwrap();
    Script::compile(scope, source, None)
        .and_then(|script| script.run(scope))
        .map_or_else(|| Err(scope.stack_trace().unwrap()), Ok)
}

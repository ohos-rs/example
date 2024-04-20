use std::thread;

use napi_derive_ohos::{js_function, module_exports};
use napi_ohos::{
    threadsafe_function::{ThreadSafeCallContext, ThreadsafeFunctionCallMode},
    CallContext, JsFunction, JsNumber, JsObject, JsUndefined, Result,
};

#[js_function(1)]
pub fn test_threadsafe_function(ctx: CallContext) -> Result<JsUndefined> {
    let func = ctx.get::<JsFunction>(0)?;

    let tsfn =
        ctx.env
            .create_threadsafe_function(&func, 0, |ctx: ThreadSafeCallContext<Vec<u32>>| {
                ctx.value
                    .iter()
                    .map(|v| ctx.env.create_uint32(*v))
                    .collect::<Result<Vec<JsNumber>>>()
            })?;

    let tsfn_cloned = tsfn.clone();

    thread::spawn(move || {
        let output: Vec<u32> = vec![0, 1, 2, 3];
        // It's okay to call a threadsafe function multiple times.
        tsfn.call(Ok(output), ThreadsafeFunctionCallMode::Blocking);
    });

    thread::spawn(move || {
        let output: Vec<u32> = vec![3, 2, 1, 0];
        // It's okay to call a threadsafe function multiple times.
        tsfn_cloned.call(Ok(output), ThreadsafeFunctionCallMode::NonBlocking);
    });

    ctx.env.get_undefined()
}

#[module_exports]
pub fn register_js(mut exports: JsObject) -> Result<()> {
    exports.create_named_method("testTsFn", test_threadsafe_function)?;
    Ok(())
}

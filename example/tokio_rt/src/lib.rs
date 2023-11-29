use napi_derive_ohos::{js_function, module_exports};
use napi_ohos::bindgen_prelude::pre_init;
use napi_ohos::{module_init, CallContext, Error, JsObject, JsString, Result, Status};
use futures::prelude::*;

#[js_function(1)]
pub fn execute_tokio_readfile(ctx: CallContext) -> Result<JsObject> {
    let js_filepath = ctx.get::<JsString>(0)?;
    let path_str = js_filepath.into_utf8()?.into_owned()?;
    ctx.env.execute_tokio_future(
        tokio::fs::read(path_str).map(|v| {
            v.map_err(|e| {
                Error::new(
                    Status::GenericFailure,
                    format!("failed to read file, {}", e),
                )
            })
        }),
        |&mut env, data| env.create_buffer_with_data(data).map(|v| v.into_raw()),
    )
}

#[js_function(1)]
pub fn error_from_tokio_future(ctx: CallContext) -> Result<JsObject> {
    let js_filepath = ctx.get::<JsString>(0)?;
    let path_str = js_filepath.into_utf8()?.into_owned()?;
    ctx.env.execute_tokio_future(
        tokio::fs::read(path_str)
            .map_err(Error::from)
            .and_then(|_| async move {
                Err::<Vec<u8>, Error>(Error::new(
                    Status::GenericFailure,
                    "Error from tokio future".to_owned(),
                ))
            }),
        |&mut env, data| env.create_buffer_with_data(data).map(|v| v.into_raw()),
    )
}


#[module_exports]
pub fn register_js(mut exports: JsObject) -> Result<()> {
    exports.create_named_method("executeTokioReadFile", execute_tokio_readfile)?;
    exports.create_named_method("executeTokioReadFileFailed", error_from_tokio_future)?;
    Ok(())
}

#[module_init]
fn init() {
    pre_init();
}

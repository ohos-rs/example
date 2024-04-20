use napi_derive_ohos::{js_function, module_exports};

use napi_ohos::{CallContext, JsFunction, JsNumber, JsObject, JsTimeout, JsUndefined, Result};

#[js_function(2)]
pub fn set_timeout(ctx: CallContext) -> Result<JsTimeout> {
    let handler: JsFunction = ctx.get(0)?;
    let timeout: JsNumber = ctx.get(1)?;
    ctx.env
        .get_global()?
        .set_timeout(handler, timeout.try_into()?)
}

#[js_function(1)]
pub fn clear_timeout(ctx: CallContext) -> Result<JsUndefined> {
    let timer: JsTimeout = ctx.get(0)?;
    ctx.env.get_global()?.clear_timeout(timer)
}

#[module_exports]
pub fn register_js(mut exports: JsObject) -> Result<()> {
    exports.create_named_method("setTimeout", set_timeout)?;
    exports.create_named_method("clearTimeout", clear_timeout)?;
    Ok(())
}

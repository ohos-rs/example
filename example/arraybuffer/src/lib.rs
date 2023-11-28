use napi_derive_ohos::{js_function, module_exports};
use napi_ohos::bindgen_prelude::pre_init;
use napi_ohos::{module_init, JsObject};
use std::f64::consts::PI;

use napi_ohos::{
    bindgen_prelude::Uint8Array, CallContext, JsArrayBuffer, JsNumber, JsTypedArray, JsUndefined,
    Result,
};
#[js_function(1)]
pub fn get_arraybuffer_length(ctx: CallContext) -> Result<JsNumber> {
    let buffer = ctx.get::<JsArrayBuffer>(0)?.into_value()?;
    ctx.env.create_uint32(buffer.len() as u32)
}

#[js_function(1)]
pub fn mutate_uint8_array(ctx: CallContext) -> Result<JsUndefined> {
    let mut buffer = ctx.get::<Uint8Array>(0)?;
    let buffer_mut_ref: &mut [u8] = buffer.as_mut();
    buffer_mut_ref[0] = 42;
    ctx.env.get_undefined()
}

#[js_function(1)]
pub fn mutate_uint16_array(ctx: CallContext) -> Result<JsUndefined> {
    let mut buffer = ctx.get::<JsTypedArray>(0)?.into_value()?;
    let buffer_mut_ref: &mut [u16] = buffer.as_mut();
    buffer_mut_ref[0] = 65535;
    ctx.env.get_undefined()
}

#[js_function(1)]
pub fn mutate_int16_array(ctx: CallContext) -> Result<JsUndefined> {
    let mut buffer = ctx.get::<JsTypedArray>(0)?.into_value()?;
    let buffer_mut_ref: &mut [i16] = buffer.as_mut();
    buffer_mut_ref[0] = 32767;
    ctx.env.get_undefined()
}

#[js_function(1)]
pub fn mutate_float32_array(ctx: CallContext) -> Result<JsUndefined> {
    let mut buffer = ctx.get::<JsTypedArray>(0)?.into_value()?;
    let buffer_mut_ref: &mut [f32] = buffer.as_mut();
    buffer_mut_ref[0] = 3.33;
    ctx.env.get_undefined()
}

#[js_function(1)]
pub fn mutate_float64_array(ctx: CallContext) -> Result<JsUndefined> {
    let mut buffer = ctx.get::<JsTypedArray>(0)?.into_value()?;
    let buffer_mut_ref: &mut [f64] = buffer.as_mut();
    buffer_mut_ref[0] = PI;
    ctx.env.get_undefined()
}

#[js_function(1)]
pub fn mutate_i64_array(ctx: CallContext) -> Result<JsUndefined> {
    let mut buffer = ctx.get::<JsTypedArray>(0)?.into_value()?;
    let buffer_mut_ref: &mut [i64] = buffer.as_mut();
    buffer_mut_ref[0] = 9223372036854775807;
    ctx.env.get_undefined()
}

#[module_exports]
pub fn register_js(mut exports: JsObject) -> Result<()> {
    exports.create_named_method("getArraybufferLength", get_arraybuffer_length)?;
    exports.create_named_method("mutateUint8Array", mutate_uint8_array)?;
    exports.create_named_method("mutateUint16Array", mutate_uint16_array)?;
    exports.create_named_method("mutateInt16Array", mutate_int16_array)?;
    exports.create_named_method("mutateFloat32Array", mutate_float32_array)?;
    exports.create_named_method("mutateFloat64Array", mutate_float64_array)?;
    exports.create_named_method("mutateI64Array", mutate_i64_array)?;
    Ok(())
}

#[module_init]
fn init() {
    pre_init();
}

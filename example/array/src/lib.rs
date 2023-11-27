use std::convert::TryInto;

use napi_derive_ohos::{contextless_function,js_function, module_exports};
use napi_ohos::{CallContext, ContextlessResult, Env, JsBoolean, JsNumber, JsObject, JsUndefined, JsUnknown, module_init, Result};
use napi_ohos::bindgen_prelude::pre_init;

#[contextless_function]
fn test_create_array(env: Env) -> ContextlessResult<JsObject> {
  env.create_empty_array().map(Some)
}

#[js_function(1)]
fn test_create_array_with_length(ctx: CallContext) -> Result<JsObject> {
  let length: u32 = ctx.get::<JsNumber>(0)?.try_into()?;
  ctx.env.create_array_with_length(length as usize)
}

#[js_function(3)]
fn test_set_element(ctx: CallContext) -> Result<JsObject> {
  let mut arr = ctx.get::<JsObject>(0).unwrap();
  let index = ctx.get::<JsNumber>(1).unwrap();
  let ele = ctx.get::<JsUnknown>(2).unwrap();
  arr.set_element(index.try_into().unwrap(), ele).unwrap();
  Ok(arr)
}

#[js_function(2)]
fn test_has_element(ctx: CallContext) -> Result<JsBoolean> {
  let arr = ctx.get::<JsObject>(0)?;
  let index = ctx.get::<JsNumber>(1)?;

  ctx.env.get_boolean(arr.has_element(index.try_into()?)?)
}

#[js_function(2)]
fn test_delete_element(ctx: CallContext) -> Result<JsObject> {
  let mut arr = ctx.get::<JsObject>(0).unwrap();
  let index = ctx.get::<JsNumber>(1).unwrap();

  arr.delete_element(index.try_into().unwrap()).unwrap();
  Ok(arr)
}

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("createArray", test_create_array)?;
  exports.create_named_method("createArrayWithLength", test_create_array_with_length)?;
  exports.create_named_method("setElement", test_set_element)?;
  exports.create_named_method("hasElement", test_has_element)?;
  exports.create_named_method("deleteElement", test_delete_element)?;
  Ok(())
}


#[module_init]
fn module_pre_init() {
  pre_init();
}





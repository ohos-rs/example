use napi_derive_ohos::napi;
use napi_ohos::bindgen_prelude::pre_init;
use napi_ohos::module_init;

#[napi]
pub fn hello() -> String {
  String::from("hello world")
}

#[module_init]
fn init() {
  pre_init();
}

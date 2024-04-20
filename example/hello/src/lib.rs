use napi_derive_ohos::napi;

#[napi]
pub fn hello() -> String {
    String::from("hello world")
}

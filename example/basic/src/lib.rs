use napi_derive_ohos::napi;
use napi_ohos::bindgen_prelude::{BigInt, Buffer, Null, Object, Undefined};
use napi_ohos::Env;

#[napi]
pub fn get_bool() -> bool {
    false
}

#[napi]
pub fn get_number() -> i64 {
    100 as i64
}

#[napi]
pub fn get_undefined() -> Undefined {
    ()
}

#[napi]
pub fn get_null() -> Null {
    Null
}

#[napi]
pub fn get_array() -> Vec<u32> {
    vec![1, 2, 3]
}

#[napi]
pub fn get_object(env: Env) -> Object {
    let mut obj = env.create_object().unwrap();
    obj.set("test", 1).unwrap();
    obj
}

#[napi]
pub fn get_array_buffer() -> Buffer {
    vec![1, 2, 3].into()
}

#[napi]
pub fn bigint_add(a: BigInt, b: BigInt) -> u128 {
    a.get_u128().1 + b.get_u128().1
}

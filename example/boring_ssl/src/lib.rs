use boring::sha;
use napi_derive_ohos::napi;

#[napi]
pub fn h() -> String {
    let mut hasher = sha::Sha256::new();

    hasher.update(b"Hello, ");
    hasher.update(b"world");

    let hash = hasher.finish();
    hex::encode(hash)
}

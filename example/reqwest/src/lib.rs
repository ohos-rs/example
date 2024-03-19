use napi_derive_ohos::napi;
use napi_ohos::{Error, Result, Status};

#[napi]
pub fn req_ip() -> Result<String> {
    let client = reqwest::blocking::get("http://httpbin.org/ip")
        .map_err(|e| Error::new(Status::GenericFailure, format!("request failed:{:?}", e)));
    match client {
        Ok(c) => {
            let a = c.status();
            Ok(a.as_str().to_string())
        }
        Err(e) => Err(e),
    }
}

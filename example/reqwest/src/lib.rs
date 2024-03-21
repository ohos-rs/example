use napi_derive_ohos::napi;
use napi_ohos::{Error, Result, Status};

#[napi]
pub fn req_ip() -> Result<String> {
    let client = reqwest::blocking::get("https://httpbin.org/ip")
        .map_err(|e| Error::new(Status::GenericFailure, format!("request failed:{:?}", e)));
    match client {
        Ok(c) => c
            .text()
            .map_err(|e| Error::new(Status::GenericFailure, format!("Text failed:{:?}", e))),
        Err(e) => Err(e),
    }
}

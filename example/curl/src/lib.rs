use curl::easy::Easy;
use napi_derive_ohos::napi;

#[napi]
pub fn request_hello() -> String {
    let mut easy = Easy::new();
    easy.ssl_verify_peer(false).unwrap();
    easy.ssl_verify_host(false).unwrap();
    easy.url("https://httpbin.org/ip").unwrap();
    easy.write_function(|data: &[u8]| {
        Ok(data.len())
    })
        .unwrap();
    let a = easy.perform();

    match a {
        Ok(()) => {
            // get the content-type then print it
            let con_type: &str = easy.content_type().unwrap().unwrap();
            String::from(con_type)
        }
        Err(e) => {
            let res = format!("{:?}",e);
            res
        }
    }
}

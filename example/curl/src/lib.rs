use curl::easy::Easy;
use napi_derive_ohos::napi;
use std::fs::File;
use std::io::Read;

#[napi]
pub fn request_hello() -> String {
    let mut easy = Easy::new();
    easy.ssl_verify_peer(false).unwrap();
    easy.ssl_verify_host(false).unwrap();
    easy.url("https://httpbin.org/ip").unwrap();
    easy.write_function(|data: &[u8]| Ok(data.len())).unwrap();
    let a = easy.perform();

    match a {
        Ok(()) => {
            // get the content-type then print it
            let con_type: &str = easy.content_type().unwrap().unwrap();
            String::from(con_type)
        }
        Err(e) => {
            let res = format!("{:?}", e);
            res
        }
    }
}

#[napi]
pub fn request_hello_with_ca() -> String {
    let mut easy = Easy::new();

    // should use root ca
    let mut f = File::open("/etc/ssl/certs/cacert.pem").unwrap();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).unwrap();

    easy.ssl_cainfo_blob(&buf).unwrap();

    // some url will fail which is self-sign CA
    easy.url("https://www.qq.com").unwrap();
    easy.write_function(|data: &[u8]| Ok(data.len())).unwrap();
    let a = easy.perform();

    match a {
        Ok(()) => {
            // get the content-type then print it
            let con_type: &str = easy.content_type().unwrap().unwrap();
            String::from(con_type)
        }
        Err(e) => {
            let res = format!("{:?}", e);
            res
        }
    }
}

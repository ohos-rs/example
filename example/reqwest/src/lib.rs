use napi_derive_ohos::napi;
use std::{fs::File, io::Read};

#[napi]
pub fn req_ip() -> napi_ohos::anyhow::Result<String> {
    let mut buf = Vec::new();
    File::open("/etc/ssl/certs/cacert.pem")?.read_to_end(&mut buf)?;
    let cert = reqwest::Certificate::from_pem(&buf)?;

    let client = reqwest::blocking::Client::builder()
        .add_root_certificate(cert)
        .build()?;

    let res = client.get("https://www.baidu.com").send()?;
    Ok(res.text()?)
}

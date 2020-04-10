use crate::currency::CurrencyResponse;
use crate::Result;
use hyper::{body::HttpBody, Client};
use hyper_tls::HttpsConnector;
use std::str;

pub async fn do_get(url: &str) -> Result<CurrencyResponse> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let uri = url.parse::<hyper::Uri>().unwrap();
    let mut res = client.get(uri).await?;

    let mut body = String::new();
    while let Some(next) = res.data().await {
        let chunk = next?;
        body.push_str(str::from_utf8(&chunk[..]).unwrap_or(""));
    }
    Ok(CurrencyResponse(body))
}

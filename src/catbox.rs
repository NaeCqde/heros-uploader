use crate::data::{headers, MyError};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::{
    header::{self, HeaderValue},
    Client,
};

pub async fn catbox(url: &str) -> Result<String, MyError> {
    let client = Client::builder()
        .default_headers(headers())
        .build()
        .unwrap();

    let encoded_url = utf8_percent_encode(&url, NON_ALPHANUMERIC).to_string();

    let resp = client
        .post("https://catbox.moe/user/api.php")
        .header(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded; charset=UTF-8"),
        )
        .body(format!("reqtype=urlupload&userhash=&url={}", encoded_url))
        .send()
        .await?;

    if resp.status().is_success() {
        return Ok(resp.text().await.unwrap());
    } else {
        return Err(MyError::ResourceNotAccessible);
    }
}

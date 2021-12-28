use chrono::Utc;
use data_encoding::BASE64;
use hmac::{Hmac, Mac};
use reqwest;
use serde::Deserialize;
use sha2::{Digest, Sha256, Sha512};
use std::io::Write;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Credential {
    key: String,
    secret: Vec<u8>,
}

impl Credential {
    pub fn new(key: &str, secret: &[u8]) -> Self {
        Self {
            key: key.to_string(),
            secret: secret.to_vec(),
        }
    }
}

fn build_url(path: &str, query: &[(&str, &str)]) -> String {
    let mut url = String::from("https://api.kraken.com");
    url.push_str(path);
    if query.len() > 0 {
        url.push_str("?");
        url.push_str(&serde_urlencoded::to_string(query).unwrap());
    }
    return url;
}

/// Sign the content of a given payload.
/// This function will panic if args doesn't contain "nonce".
/// See https://docs.kraken.com/rest/#section/Authentication/Headers-and-Signature
pub fn sign(path: &str, args: &[(&str, &str)], secret: &[u8]) -> String {
    // extract nonce value
    let nonce = args.into_iter().find(|&item| item.0.eq("nonce")).unwrap().1;
    // url encode payload
    let postdata = serde_urlencoded::to_string(args).unwrap();
    let encoded: String = nonce.to_string().to_owned() + &postdata;
    let mut hasher = Sha256::new();
    hasher.update(encoded.as_bytes());
    let mut message: Vec<u8> = vec![];
    message.write(&path.as_bytes()).unwrap();
    message.write(&hasher.finalize().as_slice()).unwrap();
    let mut mac = Hmac::<Sha512>::new_from_slice(secret).unwrap();
    mac.update(&message);
    return BASE64.encode(&mac.finalize().into_bytes());
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("request error")]
    Request(#[from] reqwest::Error),
    #[error("json error")]
    JSON(#[from] serde_json::Error),
    #[error("client error")]
    Client(u16, String),
    #[error("server error")]
    Server(u16, String),
    #[error("api error")]
    API(Vec<String>),
}

pub async fn public_request(path: &str, query: &[(&str, &str)]) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let builder = client.get(build_url(path, query));
    return Ok(builder.send().await?.text().await?);
}

pub async fn private_request(
    cred: &Credential,
    path: &str,
    params: &[(&str, &str)],
) -> Result<String, Error> {
    let mut params_secure: Vec<(&str, &str)> = Vec::new();
    let nonce = Utc::now().timestamp_millis().to_string();
    params_secure.push(("nonce", &nonce));
    params
        .into_iter()
        .for_each(|item| params_secure.push(*item));

    let signature = sign(path, &params_secure, &cred.secret);
    let headers: Vec<(&str, &str)> = vec![
        ("API-Key", &cred.key),
        ("API-Sign", &signature),
        (
            "Content-Type",
            "application/x-www-form-urlencoded; charset=utf-8",
        ),
    ];
    let client = reqwest::Client::new();
    let mut builder = client.post(build_url(path, &[]));
    for item in headers {
        builder = builder.header(item.0, item.1);
    }
    let body = serde_urlencoded::to_string(params_secure).unwrap();
    builder = builder.body(body);
    return Ok(builder.send().await?.text().await?);
}

#[derive(Debug, Deserialize)]
struct Response<T> {
    error: Vec<String>,
    result: Option<T>,
}

pub fn load_response<T>(payload: &str) -> Result<T, Error>
where
    for<'a> T: Deserialize<'a>,
{
    let response: Response<T> = serde_json::from_str(payload)?;
    if response.error.len() > 0 {
        return Err(Error::API(response.error));
    }
    return Ok(response.result.unwrap());
}

#[cfg(test)]
mod tests {
    use super::sign;
    use data_encoding::BASE64;

    #[test]
    fn sign_test() {
        let args = vec![
            ("nonce", "1616492376594"),
            ("ordertype", "limit"),
            ("pair", "XBTUSD"),
            ("price", "37500"),
            ("type", "buy"),
            ("volume", "1.25"),
        ];
        let secret = BASE64.decode(b"kQH5HW/8p1uGOVjbgWA7FunAmGO8lsSUXNsu3eow76sz84Q18fWxnyRzBHCd3pd5nE9qa99HAZtuZuj6F1huXg==").unwrap();
        let path = "/0/private/AddOrder";
        let signature = sign(path, &args, &secret);
        let expected_signature = "4/dpxb3iT4tp/ZCVEwSnEsLxx0bqyhLpdfOpc6fn7OR8+UClSV5n9E6aSS8MPtnRfp32bAb0nmbRn6H8ndwLUQ==";
        assert_eq!(&signature, expected_signature);
    }
}

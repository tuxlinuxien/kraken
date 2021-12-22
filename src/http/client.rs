use async_trait::async_trait;
use chrono::Utc;
use reqwest;
use std::io;
use thiserror::Error;

use super::libs::sign;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error")]
    Io(#[from] io::Error),
    #[error("request error")]
    Other(#[from] reqwest::Error),
    #[error("request timeout")]
    Timeout,
}

pub enum Method {
    GET,
    POST,
}

#[async_trait]
pub trait BaseClient {
    async fn request(
        &self,
        method: Method,
        path: &str,
        headers: &[(&str, &str)],
        query: &[(&str, &str)],
        params: &[(&str, &str)],
    ) -> Result<String, Error>;
    async fn public(
        &self,
        method: Method,
        path: &str,
        params: &[(&str, &str)],
    ) -> Result<String, Error>;
    async fn private(
        &self,
        method: Method,
        path: &str,
        params: &[(&str, &str)],
    ) -> Result<String, Error>;
}

pub struct Client {
    key: String,
    secret: Vec<u8>,
}

impl Client {
    pub fn new(key: &str, secret: &[u8]) -> Client {
        Client {
            key: key.to_string(),
            secret: secret.to_owned(),
        }
    }
    pub fn build_url(&self, path: &str, query: &[(&str, &str)]) -> String {
        let mut url = String::from("https://api.kraken.com");
        url.push_str(path);
        if query.len() > 0 {
            url.push_str("?");
            url.push_str(&serde_urlencoded::to_string(query).unwrap());
        }
        return url;
    }
}

#[async_trait]
impl BaseClient for Client {
    async fn request(
        &self,
        method: Method,
        path: &str,
        headers: &[(&str, &str)],
        query: &[(&str, &str)],
        params: &[(&str, &str)],
    ) -> Result<String, Error> {
        let client = reqwest::Client::new();
        let mut builder = match method {
            Method::GET => client.get(self.build_url(path, query)),
            Method::POST => client.post(self.build_url(path, &[])),
        };
        for item in headers {
            builder = builder.header(item.0, item.1);
        }
        let body = serde_urlencoded::to_string(params).unwrap();
        if body.len() > 0 {
            builder = builder.body(body);
        }
        return Ok(builder.send().await?.text().await?);
    }
    async fn public(
        &self,
        method: Method,
        path: &str,
        params: &[(&str, &str)],
    ) -> Result<String, Error> {
        let empty = vec![];
        return self.request(method, path, &empty, &params, &empty).await;
    }
    async fn private(
        &self,
        method: Method,
        path: &str,
        params: &[(&str, &str)],
    ) -> Result<String, Error> {
        let mut params_secure: Vec<(&str, &str)> = Vec::new();
        let nonce = Utc::now().timestamp_millis().to_string();
        params_secure.push(("nonce", &nonce));
        params
            .into_iter()
            .for_each(|item| params_secure.push(*item));

        let signature = sign(path, &params_secure, &self.secret);
        let headers: Vec<(&str, &str)> = vec![
            ("API-Key", &self.key),
            ("API-Sign", &signature),
            (
                "Content-Type",
                "application/x-www-form-urlencoded; charset=utf-8",
            ),
        ];
        let empty = vec![];
        return self
            .request(method, path, &headers, &empty, &params_secure)
            .await;
    }
}

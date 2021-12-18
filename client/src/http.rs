use async_trait::async_trait;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("request timeout")]
    Timeout,
}

#[async_trait]
pub trait BaseClientMethods {
    async fn get(&self, url: &str, header: HashMap<String, String>)
        -> Result<Vec<u8>, ClientError>;
    async fn post(
        &self,
        url: &str,
        header: HashMap<String, String>,
        body: &str,
    ) -> Result<Vec<u8>, ClientError>;
    async fn put(
        &self,
        url: &str,
        header: HashMap<String, String>,
        body: &str,
    ) -> Result<Vec<u8>, ClientError>;
    async fn patch(
        &self,
        url: &str,
        header: HashMap<String, String>,
        body: &str,
    ) -> Result<Vec<u8>, ClientError>;
    async fn delete(
        &self,
        url: &str,
        header: HashMap<String, String>,
    ) -> Result<Vec<u8>, ClientError>;
}

pub struct Client {
    key: String,
    secret: String,
}

impl Client {
    fn api_sign(&self) {
        panic!("not implements");
    }
}

#[async_trait]
impl BaseClientMethods for Client {
    async fn get(
        &self,
        url: &str,
        header: HashMap<String, String>,
    ) -> Result<Vec<u8>, ClientError> {
        panic!("not implements");
    }

    async fn delete(
        &self,
        url: &str,
        header: HashMap<String, String>,
    ) -> Result<Vec<u8>, ClientError> {
        panic!("not implements");
    }

    async fn post(
        &self,
        url: &str,
        header: HashMap<String, String>,
        body: &str,
    ) -> Result<Vec<u8>, ClientError> {
        panic!("not implements");
    }

    async fn put(
        &self,
        url: &str,
        header: HashMap<String, String>,
        body: &str,
    ) -> Result<Vec<u8>, ClientError> {
        panic!("not implements");
    }

    async fn patch(
        &self,
        url: &str,
        header: HashMap<String, String>,
        body: &str,
    ) -> Result<Vec<u8>, ClientError> {
        panic!("not implements");
    }
}

pub fn new(key: String, secret: String) -> impl BaseClientMethods {
    let c = Client { key, secret };
    return c;
}

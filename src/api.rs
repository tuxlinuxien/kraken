use crate::request::private_request;
use crate::request::public_request;
use crate::request::Credential;
use crate::request::Error;

pub mod public {
    use super::*;
    pub async fn time() -> Result<String, Error> {
        return public_request("/0/public/Time", &[]).await;
    }

    pub async fn system_status() -> Result<String, Error> {
        return public_request("/0/public/SystemStatus", &[]).await;
    }

    pub async fn assets() -> Result<String, Error> {
        return public_request("/0/public/Assets", &[]).await;
    }

    pub async fn asset_pair(pairs: &[&str]) -> Result<String, Error> {
        let pairs = pairs.join(",");
        let pairs: Vec<(&str, &str)> = vec![("pair", &pairs)];
        return public_request("/0/public/AssetPairs", &pairs).await;
    }

    pub async fn ohcl(pairs: &[&str]) -> Result<String, Error> {
        let pairs = pairs.join(",");
        let pairs: Vec<(&str, &str)> = vec![("pair", &pairs)];
        return public_request("/0/public/OHLC", &pairs).await;
    }

    pub async fn depth(pairs: &[&str]) -> Result<String, Error> {
        let pairs = pairs.join(",");
        let pairs: Vec<(&str, &str)> = vec![("pair", &pairs)];
        return public_request("/0/public/Depth", &pairs).await;
    }

    pub async fn trades(pairs: &[&str]) -> Result<String, Error> {
        let pairs = pairs.join(",");
        let pairs: Vec<(&str, &str)> = vec![("pair", &pairs)];
        return public_request("/0/public/Trades", &pairs).await;
    }

    pub async fn spread(pairs: &[&str]) -> Result<String, Error> {
        let pairs = pairs.join(",");
        let pairs: Vec<(&str, &str)> = vec![("pair", &pairs)];
        return public_request("/0/public/Spread", &pairs).await;
    }
}

pub mod private {
    use super::*;

    pub async fn balance(cred: &Credential) -> Result<String, Error> {
        return private_request(&cred, "/0/private/Balance", &[]).await;
    }

    pub async fn balance_ex(cred: &Credential) -> Result<String, Error> {
        return private_request(&cred, "/0/private/BalanceEx", &[]).await;
    }
}

use super::client::BaseClient;
use super::client::Error;
use super::client::Method;

pub async fn time<T: BaseClient>(clt: &T) -> Result<String, Error> {
    let any = vec![];
    return clt.public(Method::GET, "/0/public/Time", &any).await;
}

pub async fn system_status<T: BaseClient>(clt: &T) -> Result<String, Error> {
    let any = vec![];
    return clt
        .public(Method::GET, "/0/public/SystemStatus", &any)
        .await;
}

pub async fn assets<T: BaseClient>(clt: &T) -> Result<String, Error> {
    let any = vec![];
    return clt.public(Method::GET, "/0/public/Assets", &any).await;
}

pub async fn asset_pair<T: BaseClient>(clt: &T, pairs: &[&str]) -> Result<String, Error> {
    let pairs = pairs.join(",");
    let any: Vec<(&str, &str)> = vec![("pair", &pairs)];
    return clt.public(Method::GET, "/0/public/AssetPairs", &any).await;
}

pub async fn ohcl<T: BaseClient>(clt: &T, pairs: &[&str]) -> Result<String, Error> {
    let pairs = pairs.join(",");
    let any: Vec<(&str, &str)> = vec![("pair", &pairs)];
    return clt.public(Method::GET, "/0/public/OHLC", &any).await;
}

pub async fn depth<T: BaseClient>(clt: &T, pairs: &[&str]) -> Result<String, Error> {
    let pairs = pairs.join(",");
    let any: Vec<(&str, &str)> = vec![("pair", &pairs)];
    return clt.public(Method::GET, "/0/public/Depth", &any).await;
}

pub async fn trades<T: BaseClient>(clt: &T, pairs: &[&str]) -> Result<String, Error> {
    let pairs = pairs.join(",");
    let any: Vec<(&str, &str)> = vec![("pair", &pairs)];
    return clt.public(Method::GET, "/0/public/Trades", &any).await;
}

pub async fn spread<T: BaseClient>(clt: &T, pairs: &[&str]) -> Result<String, Error> {
    let pairs = pairs.join(",");
    let any: Vec<(&str, &str)> = vec![("pair", &pairs)];
    return clt.public(Method::GET, "/0/public/Spread", &any).await;
}

use super::client::BaseClient;
use super::client::Error;
use super::client::Method;

pub async fn balance<T: BaseClient>(clt: &T) -> Result<String, Error> {
    return clt.private(Method::POST, "/0/private/Balance", &[]).await;
}

pub async fn balance_ex<T: BaseClient>(clt: &T) -> Result<String, Error> {
    return clt.private(Method::POST, "/0/private/BalanceEx", &[]).await;
}

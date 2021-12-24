// use client;
// use data_encoding::BASE64;

use kraken::api;
use kraken::request;

#[tokio::main]
async fn main() {
    let key = dotenv::var("KRAKEN_KEY").unwrap();
    let secret = dotenv::var("KRAKEN_SECRET").unwrap();
    println!("{} {}", key, secret);
    let secret = data_encoding::BASE64.decode(&secret.as_bytes()).unwrap();
    let cred = request::Credential::new(&key, &secret);
    if let Ok(content) = api::public::time().await {
        println!("{}", content);
    }
    if let Ok(content) = api::public::assets().await {
        println!("{}", content);
    }
    if let Ok(content) = api::private::balance(&cred).await {
        println!("{}", content);
    }
    if let Ok(content) = api::private::balance_ex(&cred).await {
        println!("{}", content);
    }
}

// use client;
// use data_encoding::BASE64;

use kraken::api;

#[tokio::main]
async fn main() -> Result<(), api::Error> {
    let key = dotenv::var("KRAKEN_KEY").unwrap();
    let secret = dotenv::var("KRAKEN_SECRET").unwrap();
    println!("{} {}", key, secret);
    let secret = data_encoding::BASE64.decode(&secret.as_bytes()).unwrap();
    let cred = api::Credential::new(&key, &secret);

    let content = api::private::balance(&cred).await?;
    println!("{}", content);
    return Ok(());
}

use client;
use data_encoding::BASE64;

#[tokio::main]
async fn main() {
    let key = "";
    let secret = BASE64.decode(b"").unwrap();
    let clt = client::Client::new(&key, &secret);
    if let Ok(res) = client::public::time(&clt).await {
        println!("{}", res);
    }
    if let Ok(res) = client::public::system_status(&clt).await {
        println!("{}", res);
    }
    if let Ok(res) = client::private::balance(&clt).await {
        println!("{}", res);
    }
}

// use client;
// use data_encoding::BASE64;

use kraken::api;

#[tokio::main]
async fn main() -> Result<(), api::Error> {
    let content = api::public::time().await?;
    println!("{:?}", content);
    let content = api::public::system_status().await?;
    println!("{:?}", content);
    let content = api::public::assets(None, None).await?;
    println!("{:?}", content);
    let content = api::public::asset_pair(&["XXBTZUSD", "XETHXXBT"], None).await?;
    println!("{:?}", content);
    let content = api::public::ticker("XBTUSD").await?;
    println!("{:?}", content);
    let content = api::public::ohcl("XBTUSD", None, None).await?;
    println!("{:?}", content);
    let content = api::public::depth("XBTUSD", None).await?;
    println!("{:?}", content);
    let content = api::public::trades("XBTUSD", None).await?;
    println!("{:?}", content);
    let content = api::public::spread("XBTUSD", None).await?;
    println!("{:?}", content);

    // TODO:
    // - add cli
    //
    // into a .env file, add the following key:
    // KRAKEN_KEY=<your key>
    // KRAKEN_SECRET=<your secret>
    //
    // let key = dotenv::var("KRAKEN_KEY").unwrap();
    // let secret = dotenv::var("KRAKEN_SECRET").unwrap();
    // println!("{} {}", key, secret);
    // let secret = data_encoding::BASE64.decode(&secret.as_bytes()).unwrap();
    // let cred = api::Credential::new(&key, &secret);
    // let content = api::private::balance(&cred).await?;
    // println!("{:?}", content);
    // let content = api::private::balance_ex(&cred).await?;
    // println!("{:?}", content);
    // let content = api::private::trade_volume(&cred, Some(&["XETCXETH"]), None).await?;
    // println!("{:?}", content);

    return Ok(());
}

use anyhow::{anyhow, Context, Result};
use clap::{App, Arg, SubCommand};
use data_encoding::BASE64;
use kraken::api::{self, Credential};
use serde::Serialize;
use serde_json::to_string_pretty;

fn display<T>(output: T)
where
    T: Serialize,
{
    if let Ok(out) = to_string_pretty(&output) {
        println!("{}", out);
    }
}

fn build_credentials(key: Option<&str>, secret: Option<&str>) -> Result<Credential> {
    if key.is_none() {
        return Err(anyhow!("missing credentials"));
    }
    if secret.is_none() {
        return Err(anyhow!("missing credentials"));
    }
    let key = key.unwrap();
    if key == "" {
        return Err(anyhow!("missing key"));
    }
    let secret = secret.unwrap();
    if secret == "" {
        return Err(anyhow!("missing secret"));
    }
    let secret = secret.as_bytes();
    let secret = BASE64.decode(secret).context("cannot decode secret")?;
    return Ok(Credential::new(key, &secret));
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let app = App::new("kraken-cli")
        .version("0.9")
        .author("Yoann Cerda <tuxlinuxien@gmail.com>")
        .arg(
            Arg::with_name("key")
                .long("key")
                .env("KRAKEN_KEY")
                .takes_value(true)
                .global(true),
        )
        .arg(
            Arg::with_name("secret")
                .long("secret")
                .env("KRAKEN_SECRET")
                .takes_value(true)
                .global(true),
        )
        .subcommand(SubCommand::with_name("time"))
        .subcommand(SubCommand::with_name("system-status"))
        .subcommand(
            SubCommand::with_name("assets")
                .arg(
                    Arg::with_name("asset")
                        .long("asset")
                        .multiple(true)
                        .takes_value(true),
                )
                .arg(Arg::with_name("aclass").long("aclass").takes_value(true)),
        )
        .subcommand(
            SubCommand::with_name("asset-pair")
                .arg(
                    Arg::with_name("pair")
                        .long("pair")
                        .multiple(true)
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("info")
                        .long("info")
                        .takes_value(true)
                        .possible_values(&["info", "leverage", "fees", "margin"])
                        .default_value("info"),
                ),
        )
        .subcommand(
            SubCommand::with_name("ticker").arg(
                Arg::with_name("pair")
                    .long("pair")
                    .takes_value(true)
                    .required(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("ohlc")
                .arg(
                    Arg::with_name("pair")
                        .long("pair")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("interval")
                        .long("interval")
                        .takes_value(true)
                        .required(true)
                        .possible_values(&[
                            "1", "5", "15", "30", "60", "240", "1440", "10080", "21600",
                        ])
                        .default_value("1"),
                )
                .arg(Arg::with_name("since").long("since").takes_value(true)),
        )
        .subcommand(
            SubCommand::with_name("depth")
                .arg(
                    Arg::with_name("pair")
                        .long("pair")
                        .takes_value(true)
                        .required(true),
                )
                .arg(Arg::with_name("count").long("count").takes_value(true)),
        )
        .subcommand(
            SubCommand::with_name("trades")
                .arg(
                    Arg::with_name("pair")
                        .long("pair")
                        .takes_value(true)
                        .required(true),
                )
                .arg(Arg::with_name("count").long("count").takes_value(true)),
        )
        .subcommand(
            SubCommand::with_name("spread")
                .arg(
                    Arg::with_name("pair")
                        .long("pair")
                        .takes_value(true)
                        .required(true),
                )
                .arg(Arg::with_name("count").long("count").takes_value(true)),
        )
        .subcommand(SubCommand::with_name("balance"))
        .subcommand(SubCommand::with_name("balance-ex"))
        .subcommand(
            SubCommand::with_name("trade-balance")
                .arg(Arg::with_name("asset").long("asset").takes_value(true)),
        );
    let mut help = app.clone();
    let matches = &app.get_matches();
    match matches.subcommand_name() {
        Some("time") => display(api::public::time().await?),
        Some("system-status") => display(api::public::time().await?),
        Some("assets") => {
            let cmd = matches.subcommand_matches("assets").unwrap();
            let asset: Option<Vec<&str>> = if let Some(items) = cmd.values_of("asset") {
                Some(items.map(|v| v).collect())
            } else {
                None
            };

            let aclass = cmd.value_of("aclass");
            display(api::public::assets(asset.as_deref(), aclass).await?)
        }
        Some("asset-pair") => {
            let cmd = matches.subcommand_matches("asset-pair").unwrap();
            display(
                api::public::asset_pair(
                    &cmd.values_of("pair").unwrap().collect::<Vec<&str>>(),
                    cmd.value_of("info"),
                )
                .await?,
            )
        }
        Some("ticker") => {
            let cmd = matches.subcommand_matches("ticker").unwrap();
            display(api::public::ticker(cmd.value_of("pair").unwrap()).await?)
        }
        Some("ohlc") => {
            let cmd = matches.subcommand_matches("ohlc").unwrap();
            let pair = cmd.value_of("pair").unwrap();
            let interval: Option<u64> = if let Some(val) = cmd.value_of("interval") {
                Some(val.parse::<u64>().unwrap())
            } else {
                None
            };
            let since: Option<u64> = if let Some(val) = cmd.value_of("since") {
                Some(
                    val.parse::<u64>()
                        .context("since is not an valid integer")?,
                )
            } else {
                None
            };
            display(api::public::ohcl(pair, interval, since).await?)
        }
        Some("depth") => {
            let cmd = matches.subcommand_matches("depth").unwrap();
            let pair = cmd.value_of("pair").unwrap();
            let count: Option<i64> = if let Some(val) = cmd.value_of("count") {
                let val = val
                    .parse::<i64>()
                    .context("count is not an valid integer")?;
                if val < 0 || val > 500 {
                    return Err(anyhow!("count must be between 0 to 500"));
                }
                Some(val)
            } else {
                None
            };
            display(api::public::depth(pair, count).await?)
        }
        Some("trades") => {
            let cmd = matches.subcommand_matches("trades").unwrap();
            let pair = cmd.value_of("pair").unwrap();
            let count: Option<i64> = if let Some(val) = cmd.value_of("count") {
                let val = val
                    .parse::<i64>()
                    .context("count is not an valid integer")?;
                if val < 0 || val > 500 {
                    return Err(anyhow!("count must be between 0 to 500"));
                }
                Some(val)
            } else {
                None
            };
            display(api::public::trades(pair, count).await?)
        }
        Some("spread") => {
            let cmd = matches.subcommand_matches("spread").unwrap();
            let pair = cmd.value_of("pair").unwrap();
            let count: Option<i64> = if let Some(val) = cmd.value_of("count") {
                let val = val
                    .parse::<i64>()
                    .context("count is not an valid integer")?;
                if val < 0 || val > 500 {
                    return Err(anyhow!("count must be between 0 to 500"));
                }
                Some(val)
            } else {
                None
            };
            display(api::public::spread(pair, count).await?)
        }
        Some("balance") => {
            let cmd = matches.subcommand_matches("balance").unwrap();
            let cred = build_credentials(cmd.value_of("key"), cmd.value_of("secret"))?;
            display(api::private::balance(&cred).await?);
        }
        Some("balance-ex") => {
            let cmd = matches.subcommand_matches("balance-ex").unwrap();
            let cred = build_credentials(cmd.value_of("key"), cmd.value_of("secret"))?;
            display(api::private::balance_ex(&cred).await?);
        }
        Some("trade-balance") => {
            let cmd = matches.subcommand_matches("trade-balance").unwrap();
            let cred = build_credentials(cmd.value_of("key"), cmd.value_of("secret"))?;
            display(api::private::trade_balance(&cred, cmd.value_of("asset")).await?);
        }
        Some(&_) => {
            help.print_long_help()?;
        }
        None => {
            help.print_long_help()?;
        }
    }

    return Ok(());
}

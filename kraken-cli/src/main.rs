use std::str::FromStr;

use anyhow::{anyhow, Context, Result};
use clap::{App, Arg, SubCommand};
use data_encoding::BASE64;
use kraken;
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

fn load_credentials_from_args(key: Option<&str>, secret: Option<&str>) -> Option<(String, String)> {
    let key = key.unwrap_or("");
    let secret = secret.unwrap_or("");
    if key == "" || secret == "" {
        return None;
    }
    Some((key.to_string(), secret.to_string()))
}

async fn load_credentials_from_file(credentials: Option<&str>) -> Result<Option<(String, String)>> {
    let credentials = credentials.unwrap_or("");
    if credentials == "" {
        return Ok(None);
    }
    let content = tokio::fs::read_to_string(credentials)
        .await
        .map_err(|e| anyhow!("cannot open {} ({})", credentials, e))?;
    let lines: Vec<&str> = content.lines().map(|l| l).collect();
    // ensure extra lines at the end of the file won't cause any error.
    // the credential file must be generated as:
    // <API_KEY>\n
    // <API_SECRET>\n
    if lines.len() < 2 {
        return Err(anyhow!("invalid credential file"));
    }
    let key = *lines.get(0).unwrap();
    let secret = *lines.get(1).unwrap();
    Ok(Some((key.to_string(), secret.to_string())))
}

async fn build_credentials(
    key: Option<&str>,
    secret: Option<&str>,
    cred_file: Option<&str>,
) -> Result<Option<kraken::Credential>> {
    let key_pair_args = load_credentials_from_args(key, secret);
    let key_pair_file = load_credentials_from_file(cred_file).await?;
    // pick one of them.
    let key_pair = key_pair_args.or(key_pair_file);
    if key_pair.is_none() {
        return Ok(None);
    }
    let (key, secret) = key_pair.unwrap();
    let secret = secret.as_bytes();
    let secret = BASE64.decode(secret).context("cannot decode secret")?;
    return Ok(Some(kraken::Credential::new(&key, &secret)));
}

fn parse_number_option<T>(val: Option<&str>) -> Result<Option<T>, anyhow::Error>
where
    T: FromStr,
{
    let val = match val {
        Some(val) => match val.parse::<T>() {
            Ok(val) => Some(val),
            Err(_) => return Err(anyhow!("invalid input")),
        },
        None => None,
    };
    return Ok(val);
}

fn pretty_error(e: kraken::Error) -> anyhow::Error {
    match e {
        kraken::Error::API(e) => anyhow!("[API] {}", e),
        kraken::Error::JSON(e) => anyhow!("[JSON DECODE] {}", e),
        kraken::Error::Request(e) => anyhow!("[CLIENT] {}", e),
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let app = App::new("kraken-cli")
        .version("1.0.1")
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
        .arg(
            Arg::with_name("credentials")
                .long("credentials")
                .env("CREDENTIALS")
                .takes_value(true)
                .global(true)
                .help("path of file that contains your key and secret"),
        )
        .subcommand(SubCommand::with_name("time").about("Get the server's time.").display_order(1))
        .subcommand(SubCommand::with_name("system-status").about("Get the current system status or trading mode.").display_order(1))
        .subcommand(
            SubCommand::with_name("assets")
                .arg(
                    Arg::with_name("asset")
                        .long("asset")
                        .multiple(true)
                        .takes_value(true),
                )
                .arg(Arg::with_name("aclass").long("aclass").takes_value(true))
                .about("Get information about the assets that are available for deposit, withdrawal, trading and staking.")
                .display_order(1),
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
                )
                .about("Get tradable asset pairs.")
                .display_order(1),
        )
        .subcommand(
            SubCommand::with_name("ticker").arg(
                Arg::with_name("pair")
                    .long("pair")
                    .takes_value(true)
                    .required(true),
            )
            .about("Today's prices start at midnight UTC.")
            .display_order(1),
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
                .arg(Arg::with_name("since").long("since").takes_value(true))
                .about("Get OHLC data.")
                .display_order(1),
        )
        .subcommand(
            SubCommand::with_name("depth")
                .arg(
                    Arg::with_name("pair")
                        .long("pair")
                        .takes_value(true)
                        .required(true),
                )
                .arg(Arg::with_name("count").long("count").takes_value(true))
                .about("Get Order book.")
                .display_order(1),
        )
        .subcommand(
            SubCommand::with_name("trades")
                .arg(
                    Arg::with_name("pair")
                        .long("pair")
                        .takes_value(true)
                        .required(true),
                )
                .arg(Arg::with_name("count").long("count").takes_value(true))
                .about("Get recent trades.")
                .display_order(1),
        )
        .subcommand(
            SubCommand::with_name("spread")
                .arg(
                    Arg::with_name("pair")
                        .long("pair")
                        .takes_value(true)
                        .required(true),
                )
                .arg(Arg::with_name("count").long("count").takes_value(true))
                .about("Get recent spreads.")
                .display_order(1),
        )
        .subcommand(SubCommand::with_name("balance").about("(private) Retrieve all cash balances, net of pending withdrawals."))
        .subcommand(SubCommand::with_name("balance-ex").about("(private) Retrieve all cash balances, net of pending withdrawals and hold trades."))
        .subcommand(
            SubCommand::with_name("trade-balance")
                .arg(Arg::with_name("asset").long("asset").takes_value(true))
                .about("(private) Retrieve a summary of collateral balances, margin position valuations, equity and margin level."),
        )
        .subcommand(
            SubCommand::with_name("open-orders")
                .arg(Arg::with_name("trades").long("trades").hidden(false))
                .arg(Arg::with_name("userref").long("userref").takes_value(true))
                .about("(private) Retrieve information about currently open orders."),
        )
        .subcommand(
            SubCommand::with_name("closed-orders")
                .arg(Arg::with_name("trades").long("trades").hidden(false))
                .arg(Arg::with_name("userref").long("userref").takes_value(true))
                .arg(Arg::with_name("start").long("start").takes_value(true))
                .arg(Arg::with_name("end").long("end").takes_value(true))
                .arg(Arg::with_name("ofs").long("ofs").takes_value(true))
                .arg(
                    Arg::with_name("closetime")
                        .long("closetime")
                        .takes_value(true)
                        .default_value("both")
                        .possible_values(&["open", "close", "both"]),
                ).about("(private) Retrieve information about orders that have been closed (filled or cancelled)."),
        )
        .subcommand(
            SubCommand::with_name("query-orders")
                .arg(Arg::with_name("trades").long("trades").hidden(false))
                .arg(Arg::with_name("userref").long("userref").takes_value(true))
                .arg(
                    Arg::with_name("txid")
                        .long("txid")
                        .takes_value(true)
                        .multiple(true)
                        .required(true)
                ).about("(private) Retrieve information about specific orders."),
        )
        .subcommand(
            SubCommand::with_name("trades-history")
                .arg(
                    Arg::with_name("type")
                        .long("type")
                        .takes_value(true)
                        .default_value("all")
                        .possible_values(&[
                            "all",
                            "any position",
                            "closed position",
                            "closing position",
                            "no position",
                        ])
                )
                .arg(
                    Arg::with_name("trades")
                        .long("trades")
                        .hidden(false)
                        .hidden(false),
                )
                .arg(Arg::with_name("userref").long("userref").takes_value(true))
                .arg(Arg::with_name("start").long("start").takes_value(true))
                .arg(Arg::with_name("end").long("end").takes_value(true))
                .arg(Arg::with_name("ofs").long("ofs").takes_value(true))
                .about("(private) Retrieve information about trades/fills."),
        )
        .subcommand(
            SubCommand::with_name("query-trades")
                .arg(Arg::with_name("trades").long("trades").hidden(false))
                .arg(
                    Arg::with_name("txid")
                        .long("txid")
                        .takes_value(true)
                        .multiple(true)
                        .required(true),
                ).about("(private) Retrieve information about specific trades/fills."),
        )
        .subcommand(
            SubCommand::with_name("open-positions")
                .arg(
                    Arg::with_name("txid")
                        .long("txid")
                        .takes_value(true)
                        .multiple(true)
                        .required(true),
                )
                .arg(Arg::with_name("docalcs").long("docalcs").hidden(false))
                .arg(
                    Arg::with_name("consolidation")
                        .long("consolidation")
                        .takes_value(true)
                        .default_value("market"),
                ).about("(private) Get information about open margin positions."),
        )
        .subcommand(
            SubCommand::with_name("ledgers")
                .arg(
                    Arg::with_name("asset")
                        .long("asset")
                        .takes_value(true)
                        .multiple(true)
                        .default_value("all"),
                )
                .arg(
                    Arg::with_name("aclass")
                        .long("aclass")
                        .takes_value(true)
                        .default_value("currency"),
                )
                .arg(
                    Arg::with_name("type")
                        .long("type")
                        .takes_value(true)
                        .default_value("all")
                        .possible_values(&["all", "deposit", "withdrawal", "trade", "margin"]),
                )
                .arg(Arg::with_name("start").long("start").takes_value(true))
                .arg(Arg::with_name("end").long("end").takes_value(true))
                .arg(Arg::with_name("ofs").long("ofs").takes_value(true))
                .about("(private) Retrieve information about ledger entries."),
        )
        .subcommand(
            SubCommand::with_name("query-ledgers")
                .arg(
                    Arg::with_name("id")
                        .long("id")
                        .takes_value(true)
                        .multiple(true)
                        .required(true),
                )
                .arg(Arg::with_name("trades").long("trades"))
                .about("(private) Retrieve information about specific ledger entries. "),
        )
        .subcommand(
            SubCommand::with_name("trade-volume")
                .arg(
                    Arg::with_name("pair")
                        .long("pair")
                        .takes_value(true)
                        .multiple(true),
                )
                .arg(Arg::with_name("fee-info").long("fee-info").hidden(false))
                .about("(private)"),
        );

    let mut help = app.clone();
    let matches = &app.get_matches();
    let cred = build_credentials(
        matches.value_of("key"),
        matches.value_of("secret"),
        matches.value_of("credentials"),
    )
    .await?;
    match matches.subcommand_name() {
        Some("time") => display(kraken::public::time().await.map_err(pretty_error)?),
        Some("system-status") => display(kraken::public::time().await.map_err(pretty_error)?),
        Some("assets") => {
            let cmd = matches.subcommand_matches("assets").unwrap();
            let asset: Option<Vec<&str>> = if let Some(items) = cmd.values_of("asset") {
                Some(items.map(|v| v).collect())
            } else {
                None
            };

            let aclass = cmd.value_of("aclass");
            display(
                kraken::public::assets(asset.as_deref(), aclass)
                    .await
                    .map_err(pretty_error)?,
            )
        }
        Some("asset-pair") => {
            let cmd = matches.subcommand_matches("asset-pair").unwrap();
            let pair = cmd.values_of("pair").unwrap().collect::<Vec<&str>>();
            let info = cmd.value_of("info");
            display(
                kraken::public::asset_pair(&pair, info)
                    .await
                    .map_err(pretty_error)?,
            )
        }
        Some("ticker") => {
            let cmd = matches.subcommand_matches("ticker").unwrap();
            display(
                kraken::public::ticker(cmd.value_of("pair").unwrap())
                    .await
                    .map_err(pretty_error)?,
            )
        }
        Some("ohlc") => {
            let cmd = matches.subcommand_matches("ohlc").unwrap();
            let pair = cmd.value_of("pair").unwrap();
            let interval = parse_number_option(cmd.value_of("interval"))?;
            let since = parse_number_option(cmd.value_of("since"))?;
            display(
                kraken::public::ohcl(pair, interval, since)
                    .await
                    .map_err(pretty_error)?,
            )
        }
        Some("depth") => {
            let cmd = matches.subcommand_matches("depth").unwrap();
            let pair = cmd.value_of("pair").unwrap();
            let count = parse_number_option(cmd.value_of("count"))?;
            display(
                kraken::public::depth(pair, count)
                    .await
                    .map_err(pretty_error)?,
            )
        }
        Some("trades") => {
            let cmd = matches.subcommand_matches("trades").unwrap();
            let pair = cmd.value_of("pair").unwrap();
            let count = parse_number_option(cmd.value_of("count"))?;
            display(
                kraken::public::trades(pair, count)
                    .await
                    .map_err(pretty_error)?,
            )
        }
        Some("spread") => {
            let cmd = matches.subcommand_matches("spread").unwrap();
            let pair = cmd.value_of("pair").unwrap();
            let count = parse_number_option(cmd.value_of("count"))?;
            display(
                kraken::public::spread(pair, count)
                    .await
                    .map_err(pretty_error)?,
            )
        }
        // private endpoints
        Some("balance") => {
            let cred = cred.ok_or(anyhow!("missing credentials"))?;
            display(
                kraken::private::balance(&cred)
                    .await
                    .map_err(pretty_error)?,
            );
        }
        Some("balance-ex") => {
            let cred = cred.ok_or(anyhow!("missing credentials"))?;
            display(
                kraken::private::balance_ex(&cred)
                    .await
                    .map_err(pretty_error)?,
            );
        }
        Some("trade-balance") => {
            let cmd = matches.subcommand_matches("trade-balance").unwrap();
            let cred = cred.ok_or(anyhow!("missing credentials"))?;
            display(
                kraken::private::trade_balance(&cred, cmd.value_of("asset"))
                    .await
                    .map_err(pretty_error)?,
            );
        }
        Some("open-orders") => {
            let cmd = matches.subcommand_matches("open-orders").unwrap();
            let cred = cred.ok_or(anyhow!("missing credentials"))?;
            let trades = Some(cmd.is_present("trades"));
            let userref = parse_number_option(cmd.value_of("userref"))?;
            display(
                kraken::private::open_orders(&cred, trades, userref)
                    .await
                    .map_err(pretty_error)?,
            );
        }
        Some("closed-orders") => {
            let cmd = matches.subcommand_matches("closed-orders").unwrap();
            let cred = cred.ok_or(anyhow!("missing credentials"))?;
            let trades = Some(cmd.is_present("trades"));
            let userref = parse_number_option(cmd.value_of("userref"))?;
            let start = parse_number_option(cmd.value_of("start"))?;
            let end = parse_number_option(cmd.value_of("end"))?;
            let ofs = parse_number_option(cmd.value_of("ofs"))?;
            let closetime = cmd.value_of("closetime");
            display(
                kraken::private::closed_orders(&cred, trades, userref, start, end, ofs, closetime)
                    .await
                    .map_err(pretty_error)?,
            );
        }
        Some("query-orders") => {
            let cmd = matches.subcommand_matches("query-orders").unwrap();
            let cred = cred.ok_or(anyhow!("missing credentials"))?;
            let trades = Some(cmd.is_present("trades"));
            let userref = parse_number_option(cmd.value_of("userref"))?;
            let txid: Vec<&str> = cmd.values_of("txid").unwrap().map(|f| f).collect();
            display(
                kraken::private::query_orders(&cred, trades, userref, &txid)
                    .await
                    .map_err(pretty_error)?,
            );
        }
        Some("trades-history") => {
            let cmd = matches.subcommand_matches("trades-history").unwrap();
            let cred = cred.ok_or(anyhow!("missing credentials"))?;
            let trades = Some(cmd.is_present("trades"));
            let type_ = cmd.value_of("type");
            let start = parse_number_option(cmd.value_of("start"))?;
            let end = parse_number_option(cmd.value_of("end"))?;
            let ofs = parse_number_option(cmd.value_of("ofs"))?;
            display(
                kraken::private::trades_history(&cred, type_, trades, start, end, ofs)
                    .await
                    .map_err(pretty_error)?,
            );
        }
        Some("query-trades") => {
            let cmd = matches.subcommand_matches("query-trades").unwrap();
            let cred = cred.ok_or(anyhow!("missing credentials"))?;
            let trades = Some(cmd.is_present("trades"));
            let txid: Vec<&str> = cmd.values_of("txid").unwrap().map(|f| f).collect();
            display(
                kraken::private::query_trades(&cred, &txid, trades)
                    .await
                    .map_err(pretty_error)?,
            );
        }
        Some("open-positions") => {
            let cmd = matches.subcommand_matches("open-positions").unwrap();
            let cred = cred.ok_or(anyhow!("missing credentials"))?;
            let txid: Vec<&str> = cmd.values_of("txid").unwrap().map(|f| f).collect();
            let docalcs = Some(cmd.is_present("docalcs"));
            let consolidation = cmd.value_of("consolidation").unwrap();
            display(
                kraken::private::open_positions(&cred, &txid, docalcs, consolidation)
                    .await
                    .map_err(pretty_error)?,
            );
        }
        Some("ledgers") => {
            let cmd = matches.subcommand_matches("ledgers").unwrap();
            let cred = cred.ok_or(anyhow!("missing credentials"))?;
            let asset: Option<Vec<&str>> = cmd
                .values_of("asset")
                .map(|f| f.into_iter().map(|v| v).collect());
            let aclass = cmd.value_of("aclass");
            let type_ = cmd.value_of("type");
            let start = parse_number_option(cmd.value_of("start"))?;
            let end = parse_number_option(cmd.value_of("end"))?;
            let ofs = parse_number_option(cmd.value_of("ofs"))?;
            display(
                kraken::private::ledgers(&cred, asset.as_deref(), aclass, type_, start, end, ofs)
                    .await
                    .map_err(pretty_error)?,
            );
        }
        Some("query-ledgers") => {
            let cmd = matches.subcommand_matches("query-ledgers").unwrap();
            let cred = cred.ok_or(anyhow!("missing credentials"))?;
            let id: Vec<&str> = cmd.values_of("id").unwrap().map(|f| f).collect();
            let trades = Some(cmd.is_present("trades"));
            display(
                kraken::private::query_ledgers(&cred, &id, trades)
                    .await
                    .map_err(pretty_error)?,
            );
        }
        Some("trade-volume") => {
            let cmd = matches.subcommand_matches("trade-volume").unwrap();
            let cred = cred.ok_or(anyhow!("missing credentials"))?;
            let pair: Option<Vec<&str>> = cmd
                .values_of("pair")
                .map(|f| f.into_iter().map(|v| v).collect());
            let fee_info = Some(cmd.is_present("fee-info"));
            display(
                kraken::private::trade_volume(&cred, pair.as_deref(), fee_info)
                    .await
                    .map_err(pretty_error)?,
            );
        }
        Some(&_) => {
            help.print_long_help()?;
            println!("");
        }
        None => {
            help.print_long_help()?;
            println!("");
        }
    }

    return Ok(());
}

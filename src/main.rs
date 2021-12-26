// use client;
// use data_encoding::BASE64;

use clap::{App, Arg, SubCommand};
use kraken::api;
use std::fmt::Debug;

fn display<T>(output: T)
where
    T: Debug,
{
    println!("{:?}", output);
}

#[tokio::main]
async fn main() -> Result<(), api::Error> {
    let matches = App::new("kraken-cli")
        .version("0.9")
        .author("Yoann Cerda <tuxlinuxien@gmail.com>")
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
        .get_matches();
    match matches.subcommand_name() {
        Some("time") => display(api::public::time().await?),
        Some("system-status") => display(api::public::time().await?),
        Some("assets") => {
            let cmd = matches.subcommand_matches("assets").unwrap();
            let asset = cmd.values_of("asset");
            let aclass = cmd.values_of("aclass");
            println!("{:?}, {:?}", asset, aclass);
        }
        Some(&_) => {}
        None => {}
    }

    return Ok(());
}

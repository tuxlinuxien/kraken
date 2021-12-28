# kraken

Library and cli bindings of the [kraken REST API](https://docs.kraken.com/rest/)

## Cli

### Build

    make build-cli

### Usage

    kraken-cli 0.9
    Yoann Cerda <tuxlinuxien@gmail.com>

    USAGE:
        kraken-cli [SUBCOMMAND]

    FLAGS:
        -h, --help
                Prints help information

        -V, --version
                Prints version information


    SUBCOMMANDS:
        asset-pair
        assets
        depth
        help             Prints this message or the help of the given subcommand(s)
        ohlc
        spread
        system-status
        ticker
        time
        trades

### Implementation

Public

-   [x] asset-pair
-   [x] assets
-   [x] depth
-   [x] ohlc
-   [x] spread
-   [x] system-status
-   [x] ticker
-   [x] time
-   [x] trades

Private

-   [ ] balance
-   [ ] balance_ex
-   [ ] trade_balance
-   [ ] open_orders
-   [ ] closed_orders
-   [ ] query_orders
-   [ ] trades_history
-   [ ] query_trades
-   [ ] open_positions
-   [ ] ledgers
-   [ ] query_ledgers
-   [ ] trade_volume

## Lib

### Implementation

Public

-   [x] asset-pair
-   [x] assets
-   [x] depth
-   [x] ohlc
-   [x] spread
-   [x] system-status
-   [x] ticker
-   [x] time
-   [x] trades

Private

-   [x] balance
-   [x] balance_ex
-   [x] closed_orders
-   [x] ledgers
-   [x] open_orders
-   [x] open_positions
-   [x] query_ledgers
-   [x] query_orders
-   [x] query_trades
-   [x] trade_balance
-   [x] trades_history
-   [x] trade_volume

## Test

    make test

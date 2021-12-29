# kraken

Library and cli bindings of the [kraken REST API](https://docs.kraken.com/rest/)

## Cli

### Build

    make build-cli

### Usage

    kraken-cli 1.0.1
    Yoann Cerda <tuxlinuxien@gmail.com>

    USAGE:
        kraken-cli [OPTIONS] [SUBCOMMAND]

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
            --credentials <credentials>    path of file that contains your key and secret [env: CREDENTIALS=]
            --key <key>                     [env: KRAKEN_KEY=]
            --secret <secret>               [env: KRAKEN_SECRET=]

    SUBCOMMANDS:
        asset-pair        Get tradable asset pairs.
        assets            Get information about the assets that are available for deposit, withdrawal, trading and staking.
        depth             Get Order book.
        ohlc              Get OHLC data.
        spread            Get recent spreads.
        system-status     Get the current system status or trading mode.
        ticker            Today's prices start at midnight UTC.
        time              Get the server's time.
        trades            Get recent trades.
        balance           (private) Retrieve all cash balances, net of pending withdrawals.
        balance-ex        (private) Retrieve all cash balances, net of pending withdrawals and hold trades.
        closed-orders     (private) Retrieve information about orders that have been closed (filled or cancelled).
        help              Prints this message or the help of the given subcommand(s)
        ledgers           (private) Retrieve information about ledger entries.
        open-orders       (private) Retrieve information about currently open orders.
        open-positions    (private) Get information about open margin positions.
        query-ledgers     (private) Retrieve information about specific ledger entries.
        query-orders      (private) Retrieve information about specific orders.
        query-trades      (private) Retrieve information about specific trades/fills.
        trade-balance     (private) Retrieve a summary of collateral balances, margin position valuations, equity and margin level.
        trade-volume      (private)
        trades-history    (private) Retrieve information about trades/fills.

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
-   [x] balance-ex
-   [x] trade-balance
-   [x] open-orders
-   [x] closed-orders
-   [x] query-orders
-   [x] trades-history
-   [x] query-trades
-   [x] open-positions
-   [x] ledgers
-   [x] query-ledgers
-   [x] trade-volume

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

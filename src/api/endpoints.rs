pub mod public {
    use super::super::request::*;
    pub async fn time() -> Result<String, Error> {
        return public_request("/0/public/Time", &[]).await;
    }

    pub async fn system_status() -> Result<String, Error> {
        return public_request("/0/public/SystemStatus", &[]).await;
    }

    pub async fn assets() -> Result<String, Error> {
        return public_request("/0/public/Assets", &[]).await;
    }

    pub async fn asset_pair(pairs: &[&str]) -> Result<String, Error> {
        let pairs = pairs.join(",");
        let pairs: Vec<(&str, &str)> = vec![("pair", &pairs)];
        return public_request("/0/public/AssetPairs", &pairs).await;
    }

    pub async fn ohcl(pairs: &[&str]) -> Result<String, Error> {
        let pairs = pairs.join(",");
        let pairs: Vec<(&str, &str)> = vec![("pair", &pairs)];
        return public_request("/0/public/OHLC", &pairs).await;
    }

    pub async fn depth(pairs: &[&str]) -> Result<String, Error> {
        let pairs = pairs.join(",");
        let pairs: Vec<(&str, &str)> = vec![("pair", &pairs)];
        return public_request("/0/public/Depth", &pairs).await;
    }

    pub async fn trades(pairs: &[&str]) -> Result<String, Error> {
        let pairs = pairs.join(",");
        let pairs: Vec<(&str, &str)> = vec![("pair", &pairs)];
        return public_request("/0/public/Trades", &pairs).await;
    }

    pub async fn spread(pairs: &[&str]) -> Result<String, Error> {
        let pairs = pairs.join(",");
        let pairs: Vec<(&str, &str)> = vec![("pair", &pairs)];
        return public_request("/0/public/Spread", &pairs).await;
    }
}

pub mod private {
    use super::super::request::*;

    pub async fn balance(cred: &Credential) -> Result<String, Error> {
        return private_request(&cred, "/0/private/Balance", &[]).await;
    }

    pub async fn balance_ex(cred: &Credential) -> Result<String, Error> {
        return private_request(&cred, "/0/private/BalanceEx", &[]).await;
    }

    pub async fn trade_balance(cred: &Credential, asset: &str) -> Result<String, Error> {
        return private_request(&cred, "/0/private/TradeBalance", &[("asset", asset)]).await;
    }

    pub async fn open_orders(cred: &Credential, trades: bool) -> Result<String, Error> {
        return private_request(
            &cred,
            "/0/private/OpenOrders",
            &[("trades", &trades.to_string())],
        )
        .await;
    }

    pub async fn closed_orders(cred: &Credential, trades: bool) -> Result<String, Error> {
        return private_request(
            &cred,
            "/0/private/ClosedOrders",
            &[("trades", &trades.to_string())],
        )
        .await;
    }

    pub async fn query_orders(
        cred: &Credential,
        trades: bool,
        userref: u64,
        txid: &[&str],
    ) -> Result<String, Error> {
        let trades = trades.to_string();
        let userref = userref.to_string();
        let txid = txid.join(",");
        let params: Vec<(&str, &str)> =
            vec![("trades", &trades), ("userref", &userref), ("txid", &txid)];
        return private_request(&cred, "/0/private/QueryOrders", &params).await;
    }

    pub async fn trades_history(cred: &Credential, trades: bool) -> Result<String, Error> {
        return private_request(
            &cred,
            "/0/private/TradesHistory",
            &[("trades", &trades.to_string())],
        )
        .await;
    }

    pub async fn query_trades(
        cred: &Credential,
        trades: bool,
        txid: &[&str],
    ) -> Result<String, Error> {
        let trades = trades.to_string();
        let txid = txid.join(",");
        let params: Vec<(&str, &str)> = vec![("trades", &trades), ("txid", &txid)];
        return private_request(&cred, "/0/private/QueryTrades", &params).await;
    }

    pub async fn open_positions(
        cred: &Credential,
        txid: &[&str],
        docalcs: bool,
        consolidation: &str,
    ) -> Result<String, Error> {
        let txid = txid.join(",");
        let docalcs = docalcs.to_string();
        let mut params: Vec<(&str, &str)> =
            vec![("docalcs", &docalcs), ("consolidation", consolidation)];
        if txid != "" {
            params.push(("txid", &txid))
        }
        return private_request(&cred, "/0/private/OpenPositions", &params).await;
    }

    // TODO:
    // /0/private/ txid=[] docalcs=bool consolidation=market
    // /0/private/Ledgers asset=[] aclass="currency" type=""all"|"deposit"|"withdrawal"|"trade"|"margin"" start=int end=int ofs=int
    // /0/private/QueryLedgers id=[] trades=false
    // /0/private/TradeVolume pair=string
}

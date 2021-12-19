#[cfg(test)]
mod tests {
    use crate::http::libs::sign;
    use data_encoding::BASE64;

    #[test]
    fn sign_test() {
        let args = vec![
            ("nonce", "1616492376594"),
            ("ordertype", "limit"),
            ("pair", "XBTUSD"),
            ("price", "37500"),
            ("type", "buy"),
            ("volume", "1.25"),
        ];
        let secret = BASE64.decode(b"kQH5HW/8p1uGOVjbgWA7FunAmGO8lsSUXNsu3eow76sz84Q18fWxnyRzBHCd3pd5nE9qa99HAZtuZuj6F1huXg==").unwrap();
        let path = "/0/private/AddOrder";
        let signature = sign(path, &args, &secret);
        let expected_signature = "4/dpxb3iT4tp/ZCVEwSnEsLxx0bqyhLpdfOpc6fn7OR8+UClSV5n9E6aSS8MPtnRfp32bAb0nmbRn6H8ndwLUQ==";
        assert_eq!(&signature, expected_signature);
    }
}

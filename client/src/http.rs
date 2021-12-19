use data_encoding::BASE64;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256, Sha512};
use std::io::Write;

/// Sign the content of a given payload.
/// This function will panic if args doesn't contain "nonce".
fn sign(path: &str, args: &[(&str, &str)], secret: &[u8]) -> String {
    // extract nonce value
    let nonce = args.into_iter().find(|&item| item.0.eq("nonce")).unwrap().1;
    // url encode payload
    let postdata = serde_urlencoded::to_string(args).unwrap();
    let encoded: String = nonce.to_string().to_owned() + &postdata;
    let mut hasher = Sha256::new();
    hasher.update(encoded.as_bytes());
    let mut message: Vec<u8> = vec![];
    message.write(&path.as_bytes()).unwrap();
    message.write(&hasher.finalize().as_slice()).unwrap();
    let mut mac = Hmac::<Sha512>::new_from_slice(secret).unwrap();
    mac.update(&message);
    return BASE64.encode(&mac.finalize().into_bytes());
}

#[cfg(test)]
mod tests {
    use super::sign;
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

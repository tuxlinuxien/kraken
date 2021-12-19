use data_encoding::BASE64;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256, Sha512};
use std::io::Write;

/// Sign the content of a given payload.
/// This function will panic if args doesn't contain "nonce".
/// See https://docs.kraken.com/rest/#section/Authentication/Headers-and-Signature
pub fn sign(path: &str, args: &[(&str, &str)], secret: &[u8]) -> String {
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

use anyhow::Result;
use chrono::Utc;
use hmac_sha512::HMAC;
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose};

fn make_hmac(private_key: &str, message: &[u8]) -> String {
    let key = general_purpose::STANDARD.decode(private_key.as_bytes()).unwrap();
    let result = HMAC::mac(message, key);
    general_purpose::STANDARD.encode(result)
}

fn sha256(message: &str, output: &mut [u8]) {
    let mut hasher = Sha256::new();
    hasher.update(message.as_bytes());
    output.copy_from_slice(&hasher.finalize())
}

fn make_sign(private_key: &str, nonce: &str, body: &str, path: &str) -> String {
    let message = format!("{}{}", nonce, body);
    let mut result = [0u8; 32];
    sha256(&message, &mut result);

    make_hmac(private_key, &[path.as_bytes(), &result].concat())
}

pub fn get_balance() -> Result<()> {
    let pub_key = "no";
    let private_key = "no";

    let proxy = ureq::Proxy::new("socks5://127.0.0.1:1086")?;
    let agent = ureq::AgentBuilder::new()
        .proxy(proxy)
        .build();

    let path = "/0/private/TradesHistory";
    let nonce = Utc::now().timestamp().to_string();
    let body = format!("nonce={}&trades=true", nonce);
    let sign = make_sign(private_key, &nonce, &body, path);
    println!("sign = {}", sign);

    let res = agent.post(format!("https://api.kraken.com{}", path).as_str())
                             .set("API-Key", pub_key)
                             .set("API-Sign", &sign)
                             .set("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
                             .send_string(&body)?;
    println!("response: {:#?}", res.into_string()?);

    // let path = "/0/private/AddOrder";
    // let nonce = "1616492376594";
    // let private_key = "kQH5HW/8p1uGOVjbgWA7FunAmGO8lsSUXNsu3eow76sz84Q18fWxnyRzBHCd3pd5nE9qa99HAZtuZuj6F1huXg==";
    // let body = "nonce=1616492376594&ordertype=limit&pair=XBTUSD&price=37500&type=buy&volume=1.25";
    // let sign = make_sign(private_key, &nonce, &body, path);
    // println!("sign = {}", sign);

    Ok(())
}
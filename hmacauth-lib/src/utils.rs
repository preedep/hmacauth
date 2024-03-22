use std::str::Split;
use std::time::SystemTime;

use base64::{Engine as _, engine::general_purpose};
use hmac::{Hmac, Mac};
use httpdate::fmt_http_date;
use log::debug;
use reqwest::header::HeaderMap;
use sha2::{Digest, Sha256};
use url::Url;

type HmacSha256 = Hmac<Sha256>;

pub fn compute_content_sha256(content: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let result = hasher.finalize();
    return general_purpose::STANDARD.encode(&result);
}

pub fn compute_signature(string_to_signed: &String, secret: &String) -> String {
    let mut mac = HmacSha256::new_from_slice(
        /*&base64::decode(secret).expect("HMAC compute decode secret failed")*/
        &general_purpose::STANDARD
            .decode(secret)
            .expect("HMAC compute decode secret failed"),
    )
        .expect("HMAC compute_signature can take key of any size");

    mac.update(string_to_signed.as_bytes());

    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    return general_purpose::STANDARD.encode(code_bytes);
}

pub fn get_request_header(
    url_endpoint: &Url,
    http_method: &str,
    request_id: &String,
    json_payload: &String,
    access_key: &String,
) -> Result<HeaderMap, String> {
    let mut header = HeaderMap::new();

    let now = SystemTime::now();
    let http_date = fmt_http_date(now);
    let (compute_hash,string_to_sign) = generate_signature(url_endpoint,
                                                                       http_method,
                                                                       &http_date,
                                                                       json_payload);
    debug!("{}\r\n", string_to_sign);

    header.insert("Content-Type", "application/json".parse().unwrap());
    header.insert("repeatability-request-id", request_id.parse().unwrap());
    header.insert("repeatability-first-sent", http_date.parse().unwrap());
    header.insert("x-ms-date", http_date.clone().parse().unwrap());
    header.insert("x-ms-content-sha256", compute_hash.parse().unwrap());

    let authorization = format!(
        "HMAC-SHA256 SignedHeaders=x-ms-date;host;x-ms-content-sha256&Signature={}",
        compute_signature(&string_to_sign.to_string(), &access_key.to_string())
    );
    header.insert("Authorization", authorization.parse().unwrap());

    debug!("{:#?}", header);

    Ok(header)
}

pub fn generate_signature(url_endpoint: &Url,
                      http_method: &str,
                      http_date: &String,
                      json_payload: &String) -> (String, String) {
    let compute_hash = compute_content_sha256(json_payload);

    //let now = SystemTime::now();
    //let http_date = fmt_http_date(now);

    let host_authority = format!("{}", url_endpoint.host().unwrap(), );
    let path_and_query = format!("{}?{}", url_endpoint.path(), url_endpoint.query().unwrap());
    let string_to_sign = format!(
        "{}\n{}\n{};{};{}",
        http_method,
        path_and_query,
        http_date.clone(),
        host_authority,
        compute_hash.clone(),
    );
    (compute_hash, string_to_sign)
}

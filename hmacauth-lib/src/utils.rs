use std::collections::HashMap;
use std::str::Split;
use std::time::SystemTime;

use base64::{Engine as _, engine::general_purpose};
use hmac::{Hmac, Mac};
use httpdate::fmt_http_date;
use log::debug;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use url::Url;

#[derive(Debug,Serialize,Deserialize)]
pub struct HMACAuthSignedHeader {
    pub params : Vec<String>,
    pub signature : String,
}

pub type HMACAuthSignedHeaderResult = Result<HMACAuthSignedHeader, String>;

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
) -> Result<HashMap<String,String>, String> {
    let mut header = HashMap::new();

    let now = SystemTime::now();
    let http_date = fmt_http_date(now);
    let (compute_hash,string_to_sign) = generate_string_to_sign(url_endpoint,
                                                                       http_method,
                                                                       &http_date,
                                                                       json_payload);
    debug!("{}\r\n", string_to_sign);

    header.insert("Content-Type".to_string(), "application/json".parse().unwrap());
    header.insert("repeatability-request-id".to_string(), request_id.parse().unwrap());
    header.insert("repeatability-first-sent".to_string(), http_date.parse().unwrap());
    header.insert("x-ms-date".to_string(), http_date.clone().parse().unwrap());
    header.insert("x-ms-content-sha256".to_string(), compute_hash.parse().unwrap());

    let authorization = format!(
        "HMAC-SHA256 SignedHeaders=x-ms-date;host;x-ms-content-sha256&Signature={}",
        compute_signature(&string_to_sign.to_string(), &access_key.to_string())
    );
    header.insert("Authorization".to_string(), authorization.parse().unwrap());

    debug!("{:#?}", header);

    Ok(header)
}

pub fn generate_string_to_sign(url_endpoint: &Url,
                      http_method: &str,
                      http_date: &String,
                      json_payload: &String) -> (String, String) {
    let compute_hash = compute_content_sha256(json_payload);

    //let now = SystemTime::now();
    //let http_date = fmt_http_date(now);

    let host_authority = format!("{}", url_endpoint.host().unwrap(), );
    let path_and_query = match url_endpoint.query() {
        Some(query) => format!("{}?{}", url_endpoint.path(), query),
        None => format!("{}", url_endpoint.path()),
    };
    //let path_and_query = format!("{}?{}", url_endpoint.path(), url_endpoint.query().unwrap());
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

pub fn get_signed_header(header: &Vec<(String,String)>) -> HMACAuthSignedHeaderResult {
    for (key, value) in header.iter() {
        if key.to_lowercase().eq("authorization") {
            let value = value;
            if !value.starts_with("HMAC-SHA256") {
                return Err("Authorization header is not HMAC-SHA256".to_string());
            }
            let mut signed_value = HMACAuthSignedHeader {
                params: Vec::new(),
                signature: "".to_string(),
            };
            let signed_header_value = value.replace("HMAC-SHA256 SignedHeaders=","");
            let mut iter = signed_header_value.split(";");
            while let Some(param) = iter.next() {
                if param.starts_with("x-ms-content-sha256") {
                    signed_value.params.push("x-ms-content-sha256".to_string());
                    let signature = param.replace("x-ms-content-sha256&Signature=","");
                    signed_value.signature = signature.to_string();
                }else{
                    signed_value.params.push(param.to_string());
                }
            }
            return Ok(signed_value);
        }
    }
    return Err("Authorization header is not found".to_string());
}
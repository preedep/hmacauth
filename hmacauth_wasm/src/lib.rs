use wasm_bindgen::prelude::*;
use web_sys::console;

extern crate hmacauth_lib;


use hmacauth_lib::models::Payload;
use hmacauth_lib::utils::get_request_header;

#[wasm_bindgen]
pub fn string_example(s: String) -> String {
    format!("Hello {}", s)
}

#[wasm_bindgen]
pub fn http_post_payload(url: String,
                         request_id: String,
                         message: String,
                         access_key: String,
                         f_callback: &js_sys::Function,
) -> String {
    let result = format!("Hello, {}", message);

    let payload = Payload {
        message: Some(message.clone()),
    };
    let payload_str = serde_json::to_string(&payload).unwrap();
    let headers = get_request_header(
        &url.parse().unwrap(),
        "POST",
        &request_id,
        &payload_str,
        &access_key,
    );
    match headers {
        Ok(header) => {
            console::log_1(&JsValue::from_str("Print headers:"));
            header.iter().for_each(|(key, value)| {
                //print to javascript console
                let result = format!("{}: {}", key, value);
                console::log_1(&JsValue::from_str(&result));

                //call back to javascript
                f_callback.call1(&JsValue::NULL, &JsValue::from_str(&result)).unwrap();
            });
        }
        Err(e) => {
            return e;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        //let result = add(2, 2);
        //assert_eq!(result, 4);
    }
}

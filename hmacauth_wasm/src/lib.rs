use wasm_bindgen::prelude::*;
use web_sys::console;

extern crate hmacauth_lib;

use hmacauth_lib::models::Payload;
use hmacauth_lib::utils::get_request_header;

#[wasm_bindgen]
pub fn http_post_payload(url: String,
                         request_id: String,
                         message: String,
                         access_key: String,
                         _f_callback: &js_sys::Function,
) -> String {
    let result = format!("Hello, {}", message);

    let payload = Payload {
        message: Some(message.clone()),
    };
    let payload_str = serde_json::to_string(&payload).unwrap();
    let method = "POST".to_string();

    console::log_1(&JsValue::from_str("call http_post_payload"));
    console::log_1(&JsValue::from_str(&format!("url: {}", url)));
    console::log_1(&JsValue::from_str(&format!("method: {}", method)));
    console::log_1(&JsValue::from_str(&format!("request_id: {}", request_id)));
    console::log_1(&JsValue::from_str(&format!("message: {}", message)));
    console::log_1(&JsValue::from_str(&format!("access key {}", access_key)));

    let _headers = get_request_header(
        &url.parse().unwrap(),
        &method,
        &request_id,
        &payload_str,
        &access_key,
    );
    /*
    let mut closure = Closure::wrap(Box::new(move |url: String,
                                                   method:String,
                                                   request_id:String,
                                                   message: String,
                                                   access_key: String,
                                                   payload_str: String| {

        console::log_1(&JsValue::from_str(&format!("url: {}", url)));
        console::log_1(&JsValue::from_str(&format!("method: {}", method)));
        console::log_1(&JsValue::from_str(&format!("request_id: {}", request_id)));
        console::log_1(&JsValue::from_str(&format!("message: {}", message)));
        console::log_1(&JsValue::from_str(&format!("access key {}", access_key)));


        let headers = get_request_header(
            &url.parse().unwrap(),
            &method,
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
                });
            }
            Err(e) => {
                console::log_1(&JsValue::from_str(&format!("Error: {}", e)));
            }
        }

    }) as Box<dyn FnMut(String,String,String,String,String,String)>);
    */

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

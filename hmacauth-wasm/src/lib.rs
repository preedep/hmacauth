
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn string_example(s: String) -> String {
    format!("Hello {}", s)
}
#[wasm_bindgen]
pub fn http_post_payload(url: String,
                         request_id: String,
                         message: String,
                         access_key: String,
                         f_callback: &js_sys::Function) -> String {

    let result = format!("Hello, {}", message);
    let payload = hmacauth_lib::models::Payload {
        message: Some(message.clone()),
    };
    let headers = hmacauth_lib::utils::get_request_header(
        &url.parse().unwrap(),
        "POST",
        &request_id,
        &message,
        &access_key,
    );
    match headers {
        Ok(header) => {
            console::log_1(&JsValue::from_str("Print headers:"));
            header.iter().for_each(|(key, value)| {
                let result = format!("{}: {}", key, value.to_str().unwrap());
                console::log_1(&JsValue::from_str(&result));
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
    use super::*;

    #[test]
    fn it_works() {
        //let result = add(2, 2);
        //assert_eq!(result, 4);
    }
}

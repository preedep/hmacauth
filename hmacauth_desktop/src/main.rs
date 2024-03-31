use log::{debug, info};
use reqwest::header::HeaderMap;
use slint::SharedString;

use hmacauth_lib::models::Payload;
use hmacauth_lib::utils::get_request_header;

slint::include_modules!();

pub fn show_info_dialog(message: SharedString) {
    slint::slint! {
        import { StandardButton, VerticalBox } from "std-widgets.slint";
        export component InfoDialog {
            in-out property <string> info_text: "Information";
            VerticalBox {
                alignment: start;
                Text {
                    text: info_text;
                }
                StandardButton { kind: close; }
            }
        }
    }
    let dialog = InfoDialog::new().unwrap();
    dialog.set_info_text(message);
    let _ = dialog.run();
}

fn main() -> Result<(), slint::PlatformError> {
    pretty_env_logger::init();

    let ui = AppWindow::new()?;

    /*
    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });
    */
    ui.on_request_api_call({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let url = ui.get_url().to_string();
            let request_id = ui.get_request_id().to_string();
            let message = ui.get_message().to_string();
            let shared_key = ui.get_shared_key().to_string();

            debug!("url: {}", url);
            debug!("request_id: {}", request_id);
            debug!("message: {}", message);

            let payload_str = serde_json::to_string(&Payload {
                message: Some(message.clone()),
            })
                .unwrap();

            let method = "POST".to_string();

            let result = get_request_header(
                &url.parse().unwrap(),
                &method,
                &request_id,
                &payload_str,
                &shared_key,
            );
            match result {
                Ok(header) => {
                    header.iter().for_each(|(key, value)| {
                        info!("{}: {}", key, value);
                        if key.to_lowercase().eq("authorization") {
                            ui.set_authorization(value.to_string().into());
                        }
                    });
                    let header_map: HeaderMap = (&header).try_into().expect("valid headers");

                    let client = reqwest::blocking::Client::new();
                    let result = client
                        .post(&url)
                        .headers(header_map)
                        .json(&Payload {
                            message: Some(message.clone()),
                        })
                        .send();
                    match result {
                        Ok(response) => {
                            let status = response.status();
                            let body = response.text().unwrap();
                            info!("status: {}", status);
                            info!("body: {}", body);
                            //ui.set_response_status(status.to_string().into());
                            //ui.set_response_body(body.into());
                            show_info_dialog(body.into());
                        }
                        Err(e) => {
                            //error!("{}", e);
                            let error_message = format!("Error : {}", e);
                            //ui.set_response_status(error_message.into());
                            ui.set_authorization(error_message.into());
                        }
                    }
                }
                Err(e) => {
                    //error!("{}", e);
                    let error_message = format!("Error : {}", e);
                    ui.set_authorization(error_message.into());
                }
            }
        }
    });

    ui.run()
}

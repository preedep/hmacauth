use std::time::Instant;
use reqwest::header::HeaderMap;
use hmacauth_lib::utils::get_request_header;
use log::{debug, info};
use hmacauth_lib::models::Payload;
slint::include_modules!();


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
            let message = ui.get_message();
            let shared_key = ui.get_shared_key();

            ui.set_authorization("Test".to_string().into());
        }
    });


    ui.run()
}

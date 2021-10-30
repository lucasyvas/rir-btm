#![forbid(unsafe_code)]
#![warn(clippy::all)]
#![cfg_attr(not(debug_assertions), deny(warnings))]

use cfg_if::cfg_if;
use chrono::Utc;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use wasm_bindgen::prelude::*;
    } else {
        use ctor::ctor;
        use dotenv::dotenv;
        use log::{log, Level};
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[ctor]
fn init() {
    dotenv().ok();
    pretty_env_logger::try_init().ok();
}

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
extern "C" {
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(js_namespace = console, js_name = info)]
    fn console_info(s: &str);

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(js_namespace = console, js_name = warn)]
    fn console_warn(s: &str);

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn console_error(s: &str);

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(js_namespace = console, js_name = debug)]
    fn console_debug(s: &str);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn info(message: &str) {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            console_info(&format("INFO", message));
        } else {
            log(Level::Info, message);
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn warn(message: &str) {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            console_warn(&format("WARN", message));
        } else {
            log(Level::Warn, message);
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn error(message: &str) {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            console_error(&format("ERROR", message));
        } else {
            log(Level::Error, message);
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn debug(message: &str) {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            console_debug(&format("DEBUG", message));
        } else {
            log(Level::Debug, message);
        }
    }
}

fn format(level: &str, message: &str) -> String {
    format!(
        "time={} level={} message={}",
        Utc::now().to_rfc3339(),
        level,
        message
    )
}

#[cfg(not(target_arch = "wasm32"))]
fn log(level: Level, message: &str) {
    log!(level, "{}", format(level.as_str(), message));
}

#![forbid(unsafe_code)]
#![warn(clippy::all)]

use cfg_if::cfg_if;
use instant::SystemTime;
use time::{format_description::well_known::Iso8601, OffsetDateTime};

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use wasm_bindgen::prelude::*;
    } else {
        use ctor::ctor;
        use dotenvy::dotenv;
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
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = info)]
    fn console_info(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = warn)]
    fn console_warn(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn console_error(s: &str);

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

#[cfg(not(target_arch = "wasm32"))]
fn log(level: Level, message: &str) {
    log!(level, "{}", format(level.as_str(), message));
}

fn format(level: &str, message: &str) -> String {
    format!(
        "time={} level={level} message={message}",
        now_utc().format(&Iso8601::DEFAULT).expect("Valid format")
    )
}

fn now_utc() -> OffsetDateTime {
    let unix_nanos: i128 = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Later than Unix Epoch")
        .as_nanos()
        .try_into()
        .expect("Convertible to signed");

    OffsetDateTime::from_unix_timestamp_nanos(unix_nanos).expect("In range")
}

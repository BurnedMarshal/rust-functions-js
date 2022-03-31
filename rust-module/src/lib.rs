mod types;
mod utils;

use types::*;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(js_name=getService)]
pub async fn get_service(config: Config, service_id: String) -> Result<JsValue, JsValue> {
    console_log!("Get_service called!");

    get_value("https://app-backend.io.italia.it/info")
        .await
        .map_err(|err| {
            console_log!("Errore dalla get_value {:?}", err);

            JsValue::from_serde(&(err.to_string()))
                .unwrap_or(JsValue::from_str("Error encoding reqwest error"))
        })
        // La parte di errore del parsing non è gestita
        .map(|val| {
            console_log!("valore get_value {:?}", val);
            JsValue::from_serde(&val).unwrap()
        })
}

async fn get_value(url: &str) -> Result<Info, reqwest::Error> {
    let res = reqwest::Client::new()
        .get(url)
        .header("Accept", "application/json")
        .send()
        .await?;

    let text: Info = res.json().await?;
    Ok(text)
}

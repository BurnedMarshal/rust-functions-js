mod utils;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Config {
    COSMOSDB_KEY: String,
    COSMOSDB_NAME: String,
    COSMOSDB_URI: String,
}

#[wasm_bindgen]
impl Config {
    #[wasm_bindgen(constructor)]
    pub fn new(key: &str, name: &str, uri: &str) -> Self {
        Self {
            COSMOSDB_KEY: String::from(key),
            COSMOSDB_NAME: String::from(name),
            COSMOSDB_URI: String::from(uri),
        }
    }
    #[wasm_bindgen(getter)]
    pub fn COSMOSDB_KEY(&self) -> String {
        self.COSMOSDB_KEY.to_owned()
    }

    #[wasm_bindgen(getter)]
    pub fn COSMOSDB_NAME(&self) -> String {
        self.COSMOSDB_NAME.to_owned()
    }

    #[wasm_bindgen(getter)]
    pub fn COSMOSDB_URI(&self) -> String {
        self.COSMOSDB_URI.to_owned()
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct ServicePublic {
    config: Config,
    service_id: String,
}

#[wasm_bindgen]
impl ServicePublic {
    #[wasm_bindgen(constructor)]
    pub fn new(config: Config, service_id: &str) -> Self {
        Self {
            config: config,
            service_id: String::from(service_id.clone()),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn config(&self) -> Config {
        Config::new(
            &self.config.COSMOSDB_KEY,
            &self.config.COSMOSDB_NAME,
            &self.config.COSMOSDB_URI,
        )
    }

    #[wasm_bindgen(getter)]
    pub fn service_id(&self) -> String {
        self.service_id.to_owned()
    }
}

#[wasm_bindgen(js_name=getService)]
pub fn get_service(config: Config, service_id: &str) -> JsValue {
    JsValue::from_serde(&ServicePublic::new(config, service_id)).unwrap()
}

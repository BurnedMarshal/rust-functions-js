use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmosQueryResponse<T> {
    pub Documents: Vec<T>,
    pub _count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    serviceName: String,
    organizationName: String,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Config {
    cosmos_db_key: String,
    cosmos_db_name: String,
    cosmos_db_uri: String,
}

#[wasm_bindgen]
impl Config {
    #[wasm_bindgen(constructor)]
    pub fn new(key: &str, name: &str, uri: &str) -> Self {
        Self {
            cosmos_db_key: String::from(key),
            cosmos_db_name: String::from(name),
            cosmos_db_uri: String::from(uri),
        }
    }
    #[wasm_bindgen(getter)]
    pub fn cosmos_db_key(&self) -> String {
        self.cosmos_db_key.to_owned()
    }

    #[wasm_bindgen(getter)]
    pub fn cosmos_db_name(&self) -> String {
        self.cosmos_db_name.to_owned()
    }

    #[wasm_bindgen(getter)]
    pub fn cosmos_db_uri(&self) -> String {
        self.cosmos_db_uri.to_owned()
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceVersion {
    ios: String,
    android: String,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    min_app_version: DeviceVersion,
    min_app_version_pagopa: DeviceVersion,
    version: String,
    version_2: Option<String>,
}

mod utils;
use azure_data_cosmos::{
    clients::CosmosClient,
    prelude::{AuthorizationToken, CosmosOptions, Query},
    Error,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

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

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    serviceName: String,
    organizationName: String,
}

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
pub async fn get_service(config: Config, service_id: String) -> Result<JsValue, JsValue> {
    console_log!("Get_service called!");
    /* query(config, &service_id)
        .await
        .map(|s| JsValue::from_serde(&s).unwrap())
        .map_err(|e| JsValue::from_serde(&e.to_string()).unwrap())
    */
    // Ok(JsValue::from_serde(&config).unwrap())
    Err(JsValue::from_serde("Errore interno di sistema").unwrap())
    // match query(config, &service_id).await {
    //     Err(e) => Err(JsValue::from_serde(&e.to_string()).unwrap()),
    //     Ok(service) => Ok(JsValue::from_serde(&service).unwrap()),
    // }
    // Ok(JsValue::from_serde(&query(config, &service_id).await).unwrap())
}

async fn query(config: Config, service_id: &str) -> Result<Service, Error> {
    console_log!("Query called!");
    let master_key = String::from(config.COSMOSDB_KEY);
    let database_name = String::from(config.COSMOSDB_NAME);
    let collection_name = String::from("service");
    let account: String = String::from("io-d-cosmos-free");

    let authorization_token = AuthorizationToken::primary_from_base64(&master_key).expect("NUOOOO");
    console_log!(">>>>>>>>>>>>>> Authorization");
    let options = CosmosOptions::default();
    console_log!(">>>>>>>>>>>>>> Cosmos!");
    let client = CosmosClient::new(account, authorization_token, options);
    console_log!(">>>>>>>>>>>>>> DB!!");
    let database_client = client.database_client(database_name);
    console_log!(">>>>>>>>>>>>>> Collection!!");
    let collection_client = database_client.into_collection_client(collection_name);

    /*
    let p = collection_client
        .query_documents(Query::new(format!(
            "SELECT TOP 1 * FROM c WHERE c.serviceId = '{}'",
            service_id
        )))
        .into_stream::<Service>()
        .map(|p| {
            console_log!(">>>>>>>>>>>>>> Result Map!!");
            p.into_raw()
                .results
                .get(0)
                .ok_or("NOT FOUNDDDDDD")
                .cloned()
                .unwrap()
        });
    console_log!(">>>>>>>>>>>>>> RESULT!!");
    */
    p
}

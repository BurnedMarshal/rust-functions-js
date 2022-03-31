mod cosmos;
mod types;
mod utils;

use chrono::Utc;
use cosmos::*;
use types::*;

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
    get_value(
        &format!(
            "{}dbs/{}/colls/{}/docs",
            config.cosmos_db_uri(),
            config.cosmos_db_name(),
            "services"
        ),
        &service_id,
    )
    .await
    .map(|p| JsValue::from_serde(&p).unwrap())
    .map_err(|err| JsValue::from_serde(&err.to_string()).unwrap())
}

async fn get_value(url: &str, service_id: &str) -> Result<String, String> {
    let auth_token = get_authorization_token_using_master_key(
        CosmosVerb::POST,
        CosmosResurceType::Documents,
        format!("dbs/{}/colls/{}/docs", "test", "services"),
        Utc::now(),
        "key".to_owned(),
    )?;
    let res = reqwest::Client::new()
        .post(url)
        .header("x-ms-documentdb-isquery", "true")
        .header("Content-Type", "application/query+json")
        .header("authorization", auth_token)
        .body(format!(
            "{{ \"query\": \"SELECT TOP 1 * FROM c WHERE c.serviceId = '{}'\"}}",
            service_id
        ))
        .send()
        .await
        .map_err(|err| err.to_string())?;
    let response = res.text().await.map_err(|err| err.to_string())?;
    console_log!("Response {}", response);

    //let text: Service = res.json().await.map_err(|err| err.to_string())?;
    Ok(response)
}

use chrono::Utc;
use wasm_bindgen::prelude::*;

use cosmos::*;
use types::*;

mod cosmos;
mod types;
mod utils;

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

#[wasm_bindgen(js_name = getService)]
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
        &config.cosmos_db_name().as_str(),
        &config.cosmos_db_key().as_str(),
    )
        .await
        .map(|p| JsValue::from_serde(&p).unwrap())
        .map_err(|err| JsValue::from_serde(&err.to_string()).unwrap())
}

async fn get_value(url: &str, service_id: &str, db_name: &str, db_key: &str) -> Result<ServicePublic, String> {
    let date = Utc::now();
    let auth_token = get_authorization_token_using_master_key(
        CosmosVerb::POST,
        CosmosResurceType::Documents,
        format!("dbs/{}/colls/{}", db_name, "services"),
        &date,
        db_key.to_owned(),
    )?;
    // console_log!("x-ms-date {}", format_date(&date));
    let req = reqwest::Client::new()
        .post(url)
        .header("x-ms-documentdb-isquery", "True")
        .header("x-ms-date", format_date(&date))
        .header("x-ms-version", "2018-12-31")
        .header("Content-Type", "application/query+json")
        .header("authorization", auth_token)
        .body(format!(
            "{{ \"query\": \"SELECT TOP 1 * FROM c WHERE c.serviceId = '{}' ORDER BY c.version DESC\"}}",
            service_id
        ));
    // console_log!("Request {:?}", req);
    let res = req
        .send()
        .await
        .map_err(|err| err.to_string())?;
    //let response = res.text().await.map_err(|err| err.to_string())?;
    //console_log!("Response {}", response);

    let cosmos_response: CosmosQueryResponse<Service> = res.json().await.map_err(|err| err.to_string())?;
    let retrievedService = &cosmos_response.Documents[0];
    let service: ServicePublic = ServicePublic {
        available_notification_channels: if retrievedService.requireSecureChannels { Some(vec![NotificationChannel::WEBHOOK]) } else { Some(vec![NotificationChannel::WEBHOOK, NotificationChannel::EMAIL]) },
        department_name: String::from(&retrievedService.departmentName),
        organization_fiscal_code: String::from(&retrievedService.organizationFiscalCode),
        organization_name: String::from(&retrievedService.organizationName),
        service_id: String::from(&retrievedService.serviceId),
        service_metadata: retrievedService.serviceMetadata.as_ref().map( |s| ServiceMetadataPublic{
            address: s.address.as_ref().map(|a| a.to_string()),
            app_android: s.appAndroid.as_ref().map(|a| a.to_string()),
            app_ios: s.appIos.as_ref().map(|a| a.to_string()),
            category: s.category,
            cta: s.cta.as_ref().map(|a| a.to_string()),
            custom_special_flow: s.customSpecialFlow.as_ref().map(|a| a.to_string()),
            description: s.description.as_ref().map(|a| a.to_string()),
            email: s.email.as_ref().map(|a| a.to_string()),
            pec: s.pec.as_ref().map(|a| a.to_string()),
            phone: s.phone.as_ref().map(|a| a.to_string()),
            privacy_url: s.privacyUrl.as_ref().map(|a| a.to_string()),
            scope: s.scope,
            support_url: s.supportUrl.as_ref().map(|a| a.to_string()),
            token_name: s.tokenName.as_ref().map(|a| a.to_string()),
            tos_url: s.tosUrl.as_ref().map(|a| a.to_string()),
            web_url: s.webUrl.as_ref().map(|a| a.to_string()),
        }),
        service_name: String::from(&retrievedService.serviceName),
        version: retrievedService.version,
    };
    Ok(service.to_owned())
}

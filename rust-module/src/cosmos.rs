use base64::encode as encode_base64;
use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use urlencoding::encode as encode_url;

pub enum CosmosVerb {
    POST,
    GET,
}
impl CosmosVerb {
    fn as_str(&self) -> &'static str {
        match self {
            CosmosVerb::POST => "post",
            CosmosVerb::GET => "get",
        }
    }
}
pub enum CosmosResurceType {
    Database,
    Container,
    StoredProcedures,
    UserDefinedFunctions,
    Triggers,
    Users,
    Permissions,
    Documents,
}
impl CosmosResurceType {
    fn as_str(&self) -> &'static str {
        match self {
            CosmosResurceType::Database => "dbs",
            CosmosResurceType::Container => "colls",
            CosmosResurceType::StoredProcedures => "sprocs",
            CosmosResurceType::UserDefinedFunctions => "udfs",
            CosmosResurceType::Triggers => "triggers",
            CosmosResurceType::Users => "users",
            CosmosResurceType::Permissions => "permissions",
            CosmosResurceType::Documents => "docs",
        }
    }
}

pub fn get_authorization_token_using_master_key(
    verb: CosmosVerb,
    resource_type: CosmosResurceType,
    resource_id: String,
    date: DateTime<Utc>,
    master_key: String,
) -> Result<String, String> {
    let text = format!(
        "{}\n{}\n{}\n{}\n{}\n",
        verb.as_str(),
        resource_type.as_str(),
        resource_id.as_str(),
        date.format(&"%a, %d %b %Y %T %Z").to_string().as_str(),
        ""
    );
    // Create alias for HMAC-SHA256
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(master_key.as_bytes()).map_err(|op| op.to_string())?;
    mac.update(text.as_bytes());
    let result = mac.finalize();
    let signature = encode_base64(result.into_bytes());

    Ok(encode_url(&format!("type=master&ver=1.0&sig={}", signature)).into_owned())
}

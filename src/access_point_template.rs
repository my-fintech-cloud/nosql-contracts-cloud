use serde::{Deserialize, Serialize};

service_sdk::macros::use_my_no_sql_entity!();
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessPointRuleNoSqlModel{
    pub service_id: String,
    pub third_lvl_domain: Option<String>,
    pub path: Option<String>,
}

#[my_no_sql_entity("access-points")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessPointNoSqlModel{
    pub name: String,
    pub product_id: String,
    pub rules: Vec<AccessPointRuleNoSqlModel>,
}

impl AccessPointNoSqlModel {
    pub fn generate_partition_key() -> &'static str {
        "access-points"
    }

    pub fn get_id(&self) -> &str {
        &self.row_key
    }
}
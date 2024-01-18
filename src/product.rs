use serde::{Serialize, Deserialize};

service_sdk::macros::use_my_no_sql_entity!();
#[my_no_sql_entity("products")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductNoSqlModel{
    pub name: String
}

impl ProductNoSqlModel {
    pub fn generate_partition_key() -> &'static str {
        "products"
    }

    pub fn get_id(&self) -> &str {
        &self.row_key
    }
}
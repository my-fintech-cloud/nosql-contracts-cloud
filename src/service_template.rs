use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};
service_sdk::macros::use_my_no_sql_entity!();

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceTemplatePortsCustom{
    pub name: String,
    pub inner_port: i32,
    pub external_port: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceTemplatePortsMapping{
    pub http_port: Option<i32>,
    pub grpc_port: Option<i32>,
    pub custom_ports: Option<Vec<ServiceTemplatePortsCustom>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceTemplateResourcesNoSqlModel{
    pub memory_request: String,
    pub memory_limit: String,
    pub cpu_request: String,
    pub cpu_limit: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceTemplateVolumesNoSqlModel{
    pub size_in_gb: i32,
    pub paths: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceTemplateSettingsNoSqlModel{
    pub path: String,
    pub config_name: String,
    pub settings: String,
}

#[my_no_sql_entity("service-templates")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceTemplateNoSqlModel{
    pub name: String,
    pub replicas: i32,
    pub image: String,
    pub version: String,
    pub deployment_strategy: String,
    pub pull_policy: String,
    pub product_id: String,
    pub labels: HashMap<String, String>,
    pub resources: Option<ServiceTemplateResourcesNoSqlModel>,
    pub volumes: Option<ServiceTemplateVolumesNoSqlModel>,
    pub settings: Option<ServiceTemplateSettingsNoSqlModel>,
    pub ports_mapping: Option<ServiceTemplatePortsMapping>
}

impl ServiceTemplateNoSqlModel {
    pub fn generate_partition_key() -> &'static str {
        "services"
    }

    pub fn get_id(&self) -> &str {
        &self.row_key
    }
}
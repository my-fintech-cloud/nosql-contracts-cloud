use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{ServiceTemplatePortsMapping, ServiceTemplateResourcesNoSqlModel, ServiceTemplateSettingsNoSqlModel, ServiceTemplateVolumesNoSqlModel};

service_sdk::macros::use_my_no_sql_entity!();

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InfrastructureServiceTypeSaas{
    pub params: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InfrastructureServiceTypeSelfHost{
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
    pub ports_mapping: Option<ServiceTemplatePortsMapping>,
    pub env_variables: HashMap<String, String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InfrastructureServiceType {
    SelfHost(InfrastructureServiceTypeSelfHost),
    Saas(InfrastructureServiceTypeSaas),
}

#[my_no_sql_entity("infrastructure-services")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InfrastructureServiceNoSqlModel{
    pub name: String,
    pub _type: InfrastructureServiceType,
}

impl InfrastructureServiceNoSqlModel {
    pub fn generate_partition_key() -> &'static str {
        "infrastructure-service"
    }

    pub fn get_id(&self) -> &str {
        &self.name
    }
}
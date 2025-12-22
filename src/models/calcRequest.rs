use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, Debug, ToSchema)]
pub struct CalcRequest {
    pub value_a: Option<f64>,
    pub value_b: Option<f64>,
    pub operation: String,
    // new fields for localCalc
    pub device_count: Option<u64>,
    pub device_price: Option<f64>,
    pub servers_count: Option<u64>,
    pub server_price: Option<f64>,
    // descriptors for mobile dev
    pub developer_count: Option<u64>,
    pub developer_salary: Option<f64>,
    pub qa_count: Option<u64>,
    pub qa_salary: Option<f64>,
}
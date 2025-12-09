use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CalcRequest {
    pub value_a: Option<f64>,
    pub value_b: Option<f64>,
    pub operation: String,
    // new fields for localCalc
    pub device_count: Option<u64>,
    pub device_price: Option<f64>,
    pub servers_count: Option<u64>,
    pub server_price: Option<f64>,
}
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CalcRequest {
    pub value_a: f64,
    pub value_b: f64,
    pub operation: String,
}
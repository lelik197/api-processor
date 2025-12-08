use serde::Serialize;

#[derive(Serialize)]
pub struct CalcResponse {
    pub result: Option<f64>,
    pub message: String,
    pub service_used: String,
}
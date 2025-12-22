use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct CalcResponse {
    pub result: Option<f64>,
    pub message: String,
    pub service_used: String,
}
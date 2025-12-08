use axum::{Json, response::IntoResponse};
use crate::models::{calcRequest::CalcRequest, calcResponse::CalcResponse};
use crate::services::{localCalc, compareService, sensitivityService};

pub async fn process_calculation(Json(payload): Json<CalcRequest>) -> impl IntoResponse {
    // request routing logic depending on the operation
    let (res, msg, srv_name) = match payload.operation.as_str() {
        "compare" => {
            let analysis = compareService::compare(payload.value_a, payload.value_b);
            (None, analysis, "CompareService")
        },
        "sensitivity" => {
            let val = sensitivityService::analyze(payload.value_a);
            (Some(val), "Sensitivity analysis complete".to_string(), "SensitivityService")
        },
        _ => {
            // local calc by default
            let val = localCalc::calculate(payload.value_a, payload.value_b, &payload.operation);
            (Some(val), "Calculation success".to_string(), "LocalCalc")
        }
    };

    let response = CalcResponse {
        result: res,
        message: msg,
        service_used: srv_name.to_string(),
    };

    Json(response)
}
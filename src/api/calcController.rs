use axum::{Json, response::IntoResponse};
use crate::models::{calcRequest::CalcRequest, calcResponse::CalcResponse};
use crate::services::{localCalc, compareService, sensitivityService};

pub async fn process_calculation(Json(payload): Json<CalcRequest>) -> impl IntoResponse {
    // request routing logic depending on the operation
    let (res, msg, srv_name) = match payload.operation.as_str() {
        "compare" => {
            if let (Some(a), Some(b)) = (payload.value_a, payload.value_b) {
                let analysis = compareService::compare(a, b);
                (None, analysis, "CompareService")
            } else {
                (None, "Missing value_a or value_b for compare".to_string(), "CompareService")
            }
        },
        "sensitivity" => {
            if let Some(a) = payload.value_a {
                let val = sensitivityService::analyze(a);
                (Some(val), "Sensitivity analysis complete".to_string(), "SensitivityService")
            } else {
                (None, "Missing value_a for sensitivity".to_string(), "SensitivityService")
            }
        },
        _ => {
            // local calc by default
            if let (Some(dc), Some(dp), Some(sc), Some(sp)) = (
                payload.device_count,
                payload.device_price,
                payload.servers_count,
                payload.server_price,
            ) {
                let val = localCalc::calculate(dc, dp, sc, sp, &payload.operation);
                (Some(val), "Calculation success".to_string(), "LocalCalc")
            } else {
                (None, "Missing required fields (device_count, device_price, servers_count, server_price)".to_string(), "LocalCalc")
            }
        }
    };

    let response = CalcResponse {
        result: res,
        message: msg,
        service_used: srv_name.to_string(),
    };

    Json(response)
}
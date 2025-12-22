use axum::{Json, response::IntoResponse};
use crate::models::{calcRequest::CalcRequest, calcResponse::CalcResponse};
use crate::services::{localCalc, compareService, sensitivityService};

#[utoipa::path(
    post,
    path = "/api/calc",
    request_body = CalcRequest,
    responses(
        (status = 200, description = "Calculation result", body = CalcResponse)
    )
)]
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
                let dev_count = payload.developer_count.unwrap_or(0);
                let dev_salary = payload.developer_salary.unwrap_or(0.0);
                let qa_count = payload.qa_count.unwrap_or(0);
                let qa_salary = payload.qa_salary.unwrap_or(0.0);

                let val = localCalc::calculate(
                    dc,
                    dp,
                    sc,
                    sp,
                    dev_count,
                    dev_salary,
                    qa_count,
                    qa_salary,
                    &payload.operation,
                );
                (Some(val), "Calculation success".to_string(), "LocalCalc")
            } else {
                (
                    None,
                    "Missing required fields (device_count, device_price, servers_count, server_price)"
                        .to_string(),
                    "LocalCalc",
                )
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

#[utoipa::path(
    get,
    path = "/api/calc",
    responses(
        (status = 200, description = "Get service info", body = String)
    )
)]
pub async fn get_calc_info() -> impl IntoResponse {
    "Service Ready: API Processor v0.1.0".to_string()
}

#[utoipa::path(
    put,
    path = "/api/calc",
    responses(
        (status = 200, description = "Update configuration", body = String)
    )
)]
pub async fn update_config() -> impl IntoResponse {
    "Config updated (dummy)".to_string()
}

#[utoipa::path(
    patch,
    path = "/api/calc",
    responses(
        (status = 200, description = "Patch configuration", body = String)
    )
)]
pub async fn patch_config() -> impl IntoResponse {
    "Config patched (dummy)".to_string()
}

#[utoipa::path(
    delete,
    path = "/api/calc",
    responses(
        (status = 200, description = "Delete configuration", body = String)
    )
)]
pub async fn delete_config() -> impl IntoResponse {
    "Config deleted (dummy)".to_string()
}
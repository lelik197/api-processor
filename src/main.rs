use axum::{
    routing::post,
    Router,
};
use std::net::SocketAddr;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[allow(non_snake_case)]
mod api {
    pub mod calcController;
}

#[allow(non_snake_case)]
mod models {
    pub mod calcRequest;
    pub mod calcResponse;
}

#[allow(non_snake_case)]
mod services {
    pub mod cloudCalc;
    pub mod compareService;
    pub mod localCalc;
    pub mod sensitivityService;
}

#[derive(OpenApi)]
#[openapi(
    paths(
        api::calcController::process_calculation,
        api::calcController::get_calc_info,
        api::calcController::update_config,
        api::calcController::patch_config,
        api::calcController::delete_config
    ),
    components(schemas(models::calcRequest::CalcRequest, models::calcResponse::CalcResponse))
)]
struct ApiDoc;
// ---------------------------------------------------------

#[tokio::main]
async fn main() {
    // init log
    tracing_subscriber::fmt::init();

    // route processing
    let app = Router::new()
        // POST processing
        .route(
            "/api/calc",
            post(api::calcController::process_calculation)
                .get(api::calcController::get_calc_info)
                .put(api::calcController::update_config)
                .patch(api::calcController::patch_config)
                .delete(api::calcController::delete_config),
        )
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    // address for docker
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on http://{}", addr);

    // run server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
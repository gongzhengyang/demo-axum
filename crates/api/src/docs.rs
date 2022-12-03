use std::sync::Arc;

use aide::transform::TransformOpenApi;
use aide::{
    axum::{
        routing::{get, get_with},
        ApiRouter, IntoApiResponse,
    },
    openapi::OpenApi,
    redoc::Redoc,
};
use axum::{
    body::Body,
    response::{IntoResponse, Response},
    Extension, Json,
};

pub fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("demo axum swagger")
}

pub fn api_docs_json() -> ApiRouter {
    aide::gen::infer_responses(true);

    let router = ApiRouter::new()
        .api_route_with(
            "/",
            get_with(
                Redoc::new("/docs/private/api.json")
                    .with_title("Aide Axum")
                    .axum_handler(),
                |op| op.description("This documentation page."),
            ),
            |p| p.security_requirement("ApiKey"),
        )
        .route("/private/api.json", get(serve_docs));

    // Afterwards we disable response inference because
    // it might be incorrect for other routes.
    aide::gen::infer_responses(false);

    router
}

async fn serve_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    let response = Json(api).into_response();
    // response
    let (parts, body) = response.into_parts();
    let bytes = hyper::body::to_bytes(body).await.unwrap();
    let str = std::str::from_utf8(&bytes)
        .unwrap()
        .replace("openapi\":\"3.1.0", "openapi\":\"3.0.1");
    Response::from_parts(parts, Body::from(str.into_bytes()))
}

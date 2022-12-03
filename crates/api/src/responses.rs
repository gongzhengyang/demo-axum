use std::fmt::Debug;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct ListResponse<T> {
    pub max_page: u64,
    pub results: T
}

impl <T> IntoResponse for ListResponse<T>
where
    T: Serialize + Send + Sync + Debug + 'static
{
    fn into_response(self) -> Response {
        let json_string = serde_json::to_string(&self).expect("serialize error");
        (StatusCode::OK, json_string).into_response()
    }

}
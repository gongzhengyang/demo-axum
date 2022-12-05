use axum::{
    body::{boxed, Full},
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use rust_embed::RustEmbed;

// We use static route matchers ("/" and "/index.html") to serve our home
// page.
pub async fn index_handler() -> impl IntoResponse {
    static_handler("/index.html".parse::<Uri>().unwrap()).await
}

// We use a wildcard matcher ("/dist/*file") to match against everything
// within our defined assets directory. This is the directory on our Asset
// struct below, where folder = "examples/public/".
pub async fn static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();

    if path.starts_with("statics/") {
        path = path.replace("statics/", "");
    }

    StaticFile(path)
}


#[derive(RustEmbed)]
#[folder = "statics/"]
struct Asset;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
    where
        T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        match Asset::get(path.as_str()) {
            Some(content) => {
                let body = boxed(Full::from(content.data));
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                Response::builder().header(header::CONTENT_TYPE, mime.as_ref()).body(body).unwrap()
            }
            None => Response::builder().status(StatusCode::NOT_FOUND).body(boxed(Full::from("404"))).unwrap(),
        }
    }
}
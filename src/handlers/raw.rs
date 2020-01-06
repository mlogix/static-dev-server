extern crate regex;

use std::fs;
use std::path::Path;
use std::convert::Infallible;

use regex::Regex;

use hyper::{Body, Request, Response};
use hyper::{Method, StatusCode};

use crate::tools::mime_type;

const ROOT: &str = "/";

pub async fn static_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());
    let headers = response.headers_mut();

    match (req.method(), req.uri().path()) {
        (&Method::GET, path) => {
            let asset_name = if path == ROOT { "/index.html" } else { path };
            let file_name = format!("public{}", asset_name);
            let mime_type = mime_type::find(&file_name.to_owned());

            // Test path contains file extension or not. Like: `/some_assets_name.html` or `/some_path`
            let matcher = Regex::new(r".*\\/.*\\.[a-zA-Z0-9]+").unwrap();

            headers.insert("Content-Type", format!("{}", mime_type).parse().unwrap());

            if Path::new(file_name.as_str()).exists() {
                *response.body_mut() = Body::from(fs::read(file_name.as_str()).unwrap());
            } else if matcher.is_match(file_name.as_str()) {
                *response.status_mut() = StatusCode::NOT_FOUND;
            } else {
                headers.insert("refresh", "1; url='/".parse().unwrap());
                *response.status_mut() = StatusCode::MOVED_PERMANENTLY;
            }

        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    Ok(response)
}

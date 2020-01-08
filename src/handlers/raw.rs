extern crate regex;

use std::env;
use std::fs;
use std::path::Path;
use std::convert::Infallible;

use regex::Regex;

use hyper::{Body, Request, Response};
use hyper::{Method, StatusCode};

use crate::tools::mime_type;

const ROOT: &str = "/";

pub fn static_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());
    let headers = response.headers_mut();

    let index_file = match env::var("AMOEBA_INDEX_FILE") {
        Ok(p) => p.parse::<String>().unwrap(),
        Err(..) => String::from("index.html"),
    };

    let public_dir = match env::var("AMOEBA_PUBLIC_DIR") {
        Ok(p) => p.parse::<String>().unwrap(),
        Err(..) => String::from("public"),
    };

    let index_page = format!("/{}", index_file);

    match (req.method(), req.uri().path()) {
        (&Method::GET, path) => {
            let asset_name = if path == ROOT { index_page.to_owned() } else { path.to_owned() };
            let file_name = format!("{}{}", public_dir, asset_name);
            let mime_type = mime_type::find(&file_name.to_owned());
            // Test path contains file extension or not. Like: `/some_assets_name.html` or `/some_path`
            let matcher = Regex::new(r".*\\/.*\\.[a-zA-Z0-9]+").unwrap();


            if Path::new(file_name.as_str()).exists() {
                headers.insert("Content-Type", format!("{}", mime_type).parse().unwrap());
                *response.body_mut() = Body::from(fs::read(file_name.as_str()).unwrap());
            } else if matcher.is_match(file_name.as_str()) {
                *response.status_mut() = StatusCode::NOT_FOUND;
            } else {
                let file_name = format!("{}{}", public_dir, index_page);
                let mime_type = mime_type::find(&file_name.to_owned());

                headers.insert("Content-Type", format!("{}", mime_type).parse().unwrap());
                *response.body_mut() = Body::from(fs::read(file_name.as_str()).unwrap());
            }

        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    Ok(response)
}

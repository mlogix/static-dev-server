extern crate mime;

use mime::Mime;

pub fn find(filename: &String) -> Mime {

    let parts : Vec<&str> = filename.split('.').collect();

    let res = match parts.last() {
        Some(v) =>
            match *v {
                "png" => mime::IMAGE_PNG,
                "jpg" => mime::IMAGE_JPEG,
                "json" => mime::APPLICATION_JSON,
                "svg" => mime::IMAGE_SVG,
                "js" => mime::APPLICATION_JAVASCRIPT,
                "ico" => mime::IMAGE_STAR,
                "woff" => mime::FONT_WOFF,
                "woff2" => mime::FONT_WOFF2,
                "css" => mime::TEXT_CSS,
                "htm" => mime::TEXT_HTML,
                "html" => mime::TEXT_HTML,
                "pdf" => mime::APPLICATION_PDF,

                &_ => mime::TEXT_PLAIN,
            },
        None => mime::TEXT_PLAIN,
    };
    return res;
}

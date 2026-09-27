// Sites BODY formatters and all requirements
use bytes::Bytes;
use crate::configs::Configs;



pub fn check_content_type(headers: &reqwest::header::HeaderMap) -> bool {
    // format string with built in rules
    if headers.get("content-type").is_none() {
        return false; // do not process if Content-type not found
    }
    
    // now only one rule for html
    let content_type = headers.get("content-type").unwrap().to_str().unwrap();

    if !content_type.contains("text/html") {
        return false;
    }

    true
}


pub fn formatter(
    headers: &reqwest::header::HeaderMap, body: &mut Bytes, configs: &Configs
) {
    // DEFAULT FORMATTER
    // format string with built in rules
    if !check_content_type(headers) {
        return;
    }

    // changing all original urls to the proxy one
    let body_str = str::from_utf8(body).unwrap_or_default()
        .replace(&configs.proxy_to_url, &configs.proxy_from_url);

    *body = Bytes::from(body_str);
}


pub fn js_injector(
    headers: &reqwest::header::HeaderMap, body: &mut Bytes, configs: &Configs
) {
    // checking for body size, do not process any body which less then config.
    if body.len() < configs.js_injector_min_size {
        return;
    }

    if !check_content_type(headers) {
        return;
    }

    let body_str = format!(
        "{}{}", 
        str::from_utf8(body).unwrap_or_default(), 
        &configs.js_injector.clone().unwrap()
    );
    *body = Bytes::from(body_str);
}

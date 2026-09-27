use std::collections::HashMap;

// STATIC SHIT

pub const PACKET_SIZE: usize = 1024 * 1024 ;
pub const EMPTY_HEADERS_COUNT: usize = 64;


pub const REQUEST_HEADER_BLOCKLIST: [&str; 4] = [
    "",
    "Host",
    "Referer",
    "Accept-Encoding"
];


pub const RESPONSE_HEADER_BLOCKLIST: [&str; 7] = [
    "content-security-policy",
    "content-security-policy-report-only",
    "x-content-type-options",
    "x-frame-options",
    "content-length",
    "transfer-encoding",
    "content-encoding",
];



// UNSTATIC SHIT

// https://stackoverflow.com/questions/76039999/idiomatic-way-to-create-a-constant-hashmap-with-values-in-rust
pub fn get_cors_hijacking_headers() -> HashMap<String, String> {
    let mut cors_hijacking_headers = HashMap::<String, String>::new();

    cors_hijacking_headers.insert("access-control-allow-origin".to_string(), "*".to_string());
    cors_hijacking_headers.insert("access-control-allow-methods".to_string(), "GET, POST, OPTIONS".to_string());
    cors_hijacking_headers.insert("access-control-allow-headers".to_string(), "*".to_string());

    cors_hijacking_headers
}

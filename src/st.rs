pub const PACKET_SIZE: usize = 1024000;
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


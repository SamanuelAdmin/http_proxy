use bytes::Bytes;
use std::path::Path;
use std::fs;
use std::str::FromStr;
use http::{Response};
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::prelude::*;
use http_wire::WireDecode;
use http_wire::WireEncode;
use http_wire::request::FullRequest;
use http_body_util::Full;
use log::{error, info};

mod configs;
use crate::configs::*;
mod st;
use crate::st::*;
mod body_form;
use crate::body_form::*;



fn load_javascript_injection(configs: &mut Configs) {
    // loading javascript from different sources 
    // (sources - link or file)
    // at the result ready-to-use injection will be in configs

    if !configs.js_injector.is_some() {
        return;
    }

    let injector_cloned = &configs.js_injector.clone().unwrap();

    // check if string is a valid file path
    if Path::new(&injector_cloned).exists() {
        configs.js_injector = Some(
            format!("<script type=\"text/javascript\">{}</script>", 
                fs::read_to_string(&injector_cloned)
                    .expect("Cannot read JS file for injection.")
                    .replace("\n", "\r\n")
            )
        );

        if configs.debug_enabled {
            info!("JS injector configured from file: {}", injector_cloned);
        }
    } else {
        // string is a link
        configs.js_injector = Some(
            format!("<script  type=\"text/javascript\" src=\"{}\"></script>", &injector_cloned)
        );

        if configs.debug_enabled {
            info!("JS injector configured with URL: {}", injector_cloned);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init color logger
    colog::init();

    let mut configs = init_configs();
    load_javascript_injection(&mut configs);


    let listener = TcpListener::bind(
        format!("{}:{}", configs.host, configs.port)
    ).await?;

    let mut incoming = listener.incoming();

    info!(
        "[ {} ] Proxy server for original resource {} has been started on {}:{}.", 
        get_current_pid(), configs.proxy_to_url, configs.host, configs.port
    );

    while let Some(stream) = incoming.next().await {
        let stream = stream?;

        tokio::spawn(
            handle_connection(stream, configs.clone())
        );
    }

    Ok(())
}


fn check_request_header(header_name: &str) -> bool {
    for rh in &REQUEST_HEADER_BLOCKLIST {
        if header_name.eq_ignore_ascii_case(rh) {
            return false;
        }
    }

    true
}


fn build_proxy_request(client: &reqwest::Client, user_request: &FullRequest, configs: &Configs) -> reqwest::RequestBuilder {
    let full_url = format!("{}{}", configs.proxy_to_url, user_request.head.path.unwrap());
    let method_str = user_request.head.method.unwrap_or("GET");
    let method = reqwest::Method::from_str(method_str).unwrap_or(reqwest::Method::GET);

    let mut proxy_request = client.request(method, full_url);

    for header in user_request.head.headers.iter() {
        if !check_request_header(&header.name) {
            continue;
        }
        
        proxy_request = proxy_request.header(header.name, header.value);
    }

    proxy_request
        .header("Host", configs.proxy_to_host.clone())
        .body(Bytes::copy_from_slice(user_request.body))
}


fn check_response_header(header_name: &str) -> bool {
    for rh in &RESPONSE_HEADER_BLOCKLIST {
        if header_name.eq_ignore_ascii_case(rh) {
            return false;
        }
    }

    true
}


fn build_proxy_response(
    status: reqwest::StatusCode, 
    headers: reqwest::header::HeaderMap, 
    mut body: Bytes, configs: &Configs
) -> Result<Response<Full<Bytes>>, http::Error> {
    let mut response = Response::builder().status(status.as_u16());
    let mut last_header_name = String::new();

    if configs.enable_formatter {
        formatter(&headers, &mut body, configs);
    } 

    if configs.js_injector.is_some() {
        js_injector(&headers, &mut body, configs);
    }


    for (name, value) in headers {
        let header_name = if let Some(name_unw) = name {
            last_header_name = name_unw.to_string();
            name_unw.to_string()
        } else {
            last_header_name.clone()
        };

        if !check_response_header(&header_name) {
            continue;
        }

        response = response.header(
            header_name.clone(), value.as_bytes().to_owned()
        );
    }

    // CORS hijacking
    // if enabled - sending "ALL enabled" CORS to the client
    // to except cross-site origin blocking
    if configs.cors_hijacking {
        for (name, value) in get_cors_hijacking_headers() {
            response = response.header(
                name, value
            )
        }    
    }

    Ok(response.body(Full::new(body)).unwrap())
}


async fn handle_connection(mut stream: TcpStream, configs: Configs) {
    let client = reqwest::Client::new();
    let connection_ip = stream.peer_addr().unwrap();

    let mut buffer = [0; PACKET_SIZE];
    let gotten_bytes_pack = stream.read(&mut buffer).await;

    if let Err(error) = gotten_bytes_pack {
        if configs.debug_enabled {
            error!(
                "[ {} ] Failed to read bytes from client! Error: {}", 
                connection_ip, error
            );
        }

        return;
    }

    let gotten_bytes = gotten_bytes_pack.unwrap();

    // parsing request
    let mut headers = [httparse::EMPTY_HEADER; EMPTY_HEADERS_COUNT];
    let (request, request_length) = match FullRequest::decode(&buffer[..gotten_bytes], &mut headers) {
        Ok(result) => result,
        Err(e) => {
            if configs.debug_enabled {
                error!(
                    "[ {} ] Failed to parse request. Error: {}", 
                    connection_ip, e
                );
            }

            return;
        }
    };


    // mapping request and build new one
    // static map for now
    let proxy_request = build_proxy_request(&client, &request, &configs);

        
    let source_response = proxy_request.send().await.unwrap();

    // clone response 
    let status_code = source_response.status();
    let response_headers = source_response.headers().clone();
    let body = source_response.bytes().await.unwrap();

    // build new response (proxy response)
    let client_response = build_proxy_response(
        status_code, response_headers, body, &configs
    ).unwrap();

    stream.write_all(&client_response.encode().unwrap()).await.unwrap();
    stream.flush().await.unwrap();

    if configs.debug_enabled {
        info!(
            "[ {} ] Got {} bytes. Connection closed without errors.",
            connection_ip, request_length
        );
    }
}

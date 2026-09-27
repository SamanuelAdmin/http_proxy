use clap::{Parser};


#[derive(Parser, Clone)]
#[command(
    name = "HTTP Proxy Util",
    version = "1.0.0",
    author = "Samanuel Admin",
    about = "Proxy any http/https site"
)]
pub struct Configs {
    #[arg(
        short = 'd',
        long = "debug",
        default_value_t = false,
        action = clap::ArgAction::Set
    )]
    pub debug_enabled: bool,

    #[arg(
        long = "proxy-from-url",
        value_name = "URL"
    )]
    pub proxy_from_url: String,

    #[arg(
        long = "proxy-to-url",
        value_name = "URL"
    )]
    pub proxy_to_url: String,

    #[arg(
        long = "proxy-to-host",
        value_name = "HOST"
    )]
    pub proxy_to_host: String,

    #[arg(
        short = 'H',
        long = "server-host",
        value_name = "IP",
        default_value = "127.0.0.1"
    )]
    pub host: String,

    #[arg(
        short = 'P',
        long = "server-port",
        value_name = "PORT",
        default_value_t = 7878
    )]
    pub port: u16,

    #[arg(
        short = 'f',
        long = "enable-formatter",
        default_value_t = true,
        action = clap::ArgAction::Set
    )]
    pub enable_formatter: bool,

    #[arg(
        long = "cors-hijacking",
        default_value_t = true,
        action = clap::ArgAction::Set
    )]
    pub cors_hijacking: bool,

    #[arg(
        short = 'j',
        long = "js-injector",
        value_name = "STR"
    )]
    pub js_injector: Option<String>,

    #[arg(
        long = "injector-min-size",
        default_value_t = 0
    )]
    pub js_injector_min_size: usize,
}


pub fn init_configs() -> Configs {
    Configs::parse()
}

pub fn get_current_pid() -> u32 {
    if let Ok(pid) = sysinfo::get_current_pid() {
        pid.as_u32()
    } else {
        0
    }
}

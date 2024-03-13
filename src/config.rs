use std::net::Ipv4Addr;

use clap::Parser;
use dotenv::dotenv;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Config {
    #[arg(short, long, env("SERVER_IP"), default_value = "127.0.0.1")]
    pub ipaddr: Ipv4Addr,
    #[arg(short, long, env("SERVER_PORT"), default_value_t = 8022)]
    pub port: u16,
}

impl Config {
    pub fn from_env_and_args() -> Self {
        dotenv().ok();
        Self::parse()
    }
}

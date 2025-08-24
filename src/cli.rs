use clap::Parser;
use std::net::IpAddr;

#[derive(Debug, Parser)]
#[command(name = "ip_sniffer")]
pub struct Cli {
    #[arg(short, long, default_value_t = 800)]
    pub threads: u16,

    #[arg(short, long, default_value_t = u16::MAX)]
    pub max_port: u16,

    pub ip_addr: IpAddr,
}

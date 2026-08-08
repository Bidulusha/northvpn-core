mod xray_configs;
mod arg_handler;
mod messages;

use xray_configs::default_tun_inbound;

use std::io::{self, BufRead};
use std::path::Path;
use std::process::Command;
use xray_configs::XrayConfig;

use arg_handler::ArgCommands;

const XRAY: &'static str = "./xray-core/xray";
const XRAY_KNIFE: &'static str = "./xray-core/xray-knife";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Automaticly handle arguments and start vpn
    ArgCommands::handle(std::env::args().collect()).await;
    Ok(())
}

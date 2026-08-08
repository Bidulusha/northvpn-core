use std::fs::File;
use std::io::{self, BufRead, BufReader, prelude::*};
use std::process::Command;
use std::process::Stdio;
use colored::Colorize;

use crate::XrayConfig;
use crate::{ XRAY, XRAY_KNIFE, info_message };
use crate::default_tun_inbound;

use crate::arg_handler::ArgCommandsError;
use crate::{ error_message, command_message, nr };

macro_rules! config_check {
    ($config:tt) => {
        if $config.is_none() {
            error_message!("Configuration unspecified!");
            return Err(Box::new(
                ArgCommandsError::new("Configuration unspecified!")
            ));
        }
    }
}

async fn start_xray(path_to_config: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut child = Command::new("sudo")
        .args([XRAY, "run", "-c", &path_to_config])
        .stdout(Stdio::piped())
        .spawn()?;

    let output = child.stdout.take().expect("Cannot get stout from xray!");

    tokio::spawn(async move {
        let reader = BufReader::new(output);
        for line in reader.lines() {
            match line {
                Ok(text) => {command_message!(XRAY, text);}
                Err(_err) => {error_message!(_err)}
            }
        }
    }).await?;                          // AWAITING HERE!

    Ok(())
}

async fn parse_link(link: String) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new(XRAY_KNIFE)
        .args(["parse", "-c", &format!("{}", link), "-j"])
        .output()?;


    let config = serde_json::from_str::<XrayConfig>(&String::from_utf8(output.stdout.clone()).unwrap());
    if config.is_ok() && !output.stdout.is_empty() {
        // Rewrite config to TUN
        let mut tun_config = config.unwrap(); 
        tun_config.inbounds = Some(vec![default_tun_inbound()]);
        
        // Write config to file
        let mut file = File::create("config.json")?;
        file.write_all(serde_json::to_string_pretty(&tun_config).unwrap().as_bytes())?;

        // Start xray
        start_xray("config.json".into()).await?;

        Ok(())
    } else {
        error_message!(&String::from_utf8(output.stderr).unwrap());
        Ok(())
    }
}

pub async fn run_handler(flag: Option<String>, config: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    match flag.as_deref() {
        Some("-c") | Some("--config") => { // Configuration file
            config_check!(config);

            let _ = start_xray(config.unwrap()).await;
        }
        Some("-l") | Some("--link") => { // Configuration link
            config_check!(config);
            
            let _ = parse_link(config.unwrap()).await;
        }
        Some(_) | None => { // Command error (no flag)
            error_message!("Command syntax error!");
        }
    }
    Ok(())
}
// Copyright (c) 2026, Eloise Nash
#![feature(addr_parse_ascii)]
mod methods;

use clap::Parser;
use resting::Method;
use std::error::Error;
use log::info;
use rustls::VecInput;

use crate::methods::conn;
use crate::methods::requests;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Network port to use, if not defined in address string
    #[arg(short, long, value_parser = clap::value_parser!(u16).range(1..))]
    port: Option<u16>,

    ///Specify a file path to write response to
    #[arg(short, long = "file", value_parser = clap::value_parser!(String))]
    file_path: Option<String>,

    /// Which HTTP Method to use
    #[arg(value_enum)]
    method: Option<Method>,

    ///IP Address to send the request to, optionally append the port after a `:`
    #[arg(value_parser = clap::value_parser!(String))]
    addr: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    use requests::make_request;

    // let tmp_fp: Option<String> = None;

    let cli = Cli::parse();
    let mut over_port = "";
    let mut port_override = false;

    // This seems inefficient, maybe refactor?
    let sv_addr = match cli.addr.split(':').next() {
        Some(v) => v.to_string(),
        None => panic!("Fatal Error: Could not parse server address.")
    };

    let rev_addr: String = cli.addr.chars().rev().collect();

    match rev_addr.split(':').next() {
        Some(v) => {
            if v != cli.addr {
                over_port = v;
                port_override = true;
            }
        }
        None => {
            panic!("Fatal Error: Invalid address entered!");
        }
    }

    let mut sv_name: String = String::new();

    if port_override {
        sv_name = cli.addr;
    } else {
        sv_name.push_str(cli.addr.as_str());
        over_port = match cli.port {
            Some(v) => format!("{}", v).as_str(),
            None => {
                info!("No port found, defaulting to 8080");
                "8080"
            },
        };
        sv_name.push(':');
        sv_name.push_str(over_port);
    }

    // let out_str = make_request(Method::HEAD, "8.8.8.8".to_string(), tmp_fp, Some(80)).await;

    // build out params for conn


    let mut tcp_stream = conn::create_tcp_conn(sv_name);
    let mut input = VecInput::default();
    let mut client_conn = conn::create_config(sv_addr);



    let out_str = make_request(
        cli.method.unwrap(),
        cli.addr.unwrap(),
        cli.file_path,
        cli.port,
    )
    .await;

    println!("{}", out_str);

    Ok(())
}

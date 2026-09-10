#![feature(addr_parse_ascii)]
mod methods;

use std::error::Error;
use resting::Method;
use clap::Parser;

use crate::methods::requests;
use crate::methods::conn;

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
    addr: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    use requests::make_request;

    let tmp_fp: Option<String> = None;

    let cli = Cli::parse();

    // let out_str = make_request(Method::HEAD, "8.8.8.8".to_string(), tmp_fp, Some(80)).await;

    let out_str = make_request(cli.method.unwrap(), cli.addr.unwrap(), cli.file_path, cli.port).await;    

    println!("{}", out_str);

    Ok(())
}

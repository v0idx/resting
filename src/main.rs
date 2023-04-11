use std::error::Error;
use resting::Method;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional network port to use
    #[arg(short, long, value_parser = clap::value_parser!(u16).range(1..))]
    port: Option<u16>,

    /// Use ipv6
    #[arg(short = '6' ,long = "v6")]
    v6: bool,

    /// What HTTP Method to use
    #[arg(value_enum)]
    method: Option<Method>,

    ///IP ADDR
    #[arg(value_parser = clap::value_parser!(String))]
    addr: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    use resting::make_request;

    let cli = Cli::parse();

    let out_str = make_request(cli.method.unwrap(), cli.addr.unwrap()).await;

    println!("{}", out_str);

    Ok(())
}

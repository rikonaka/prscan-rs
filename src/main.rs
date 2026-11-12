use anyhow::Result;
use clap::ArgAction;
use clap::Parser;
use clap::ValueEnum;
use pistol::Pistol;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ScanKind {
    /// TCP SYN SCAN
    Syn,
    /// TCP CONNECT SCAN
    Connect,
    /// UDP SCAN
    Udp,
}

/// Next generation probe scan software.
#[derive(Parser, Debug, Clone)]
#[command(author = "RikoNaka", version, about, long_about = None)]
struct CliArgs {
    /// Work in scan mode.
    #[arg(short = 's', long = "scan")]
    scan: Option<ScanKind>,
}

fn main() {
    let args = CliArgs::parse();

    match args.scan {
        Some(sk) => (),
        None => (),
    }
    println!("Hello, world!");
}

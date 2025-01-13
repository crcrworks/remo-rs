mod cli;

use clap::Parser as _;
use cli::{Cli, Cmds};
use eyre::Result;
use remo_sdk::{self, client::Client};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;
    let cli = Cli::parse();

    let token = env::var("TOKEN").expect("TOKEN Not found");
    let client = Client::new(None, Some(token), None, None);

    match cli.cmds {
        Cmds::Operate { device, power } => match device {
            cli::Device::Ac => todo!(),
            cli::Device::Light => match power {
                cli::Power::On => {
                    // on
                }
                cli::Power::Off => {
                    // off
                }
            },
        },
        Cmds::GetDevices => {
            let devices = client.get_devices().await?;
            println!("{devices:?}");
        }
        _ => {}
    }

    Ok(())
}

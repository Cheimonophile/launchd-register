
use std::{*};

use crate::*;

use clap::{Parser, Subcommand};


pub fn get() -> Command {
  Command::parse()
}


/// Installs and uninstalls rust programs as mac services
#[derive(Parser)]
#[clap(
  version = "1.0",
  author = "Benjamin Reagan McLemore V <mclemorebenjamin@gmail.com>",
  about = "Installs and uninstalls rust programs as Mac OS services"
)]
pub struct Command {

  #[command(subcommand)]
  pub action: Action

}

#[derive(Subcommand)]
#[clap(about = "Specifies whether or install or uninstall the mac service")]
pub enum Action {

  #[clap(about = "Installs the rust program as a mac service")]
  Install {

    /// The path to the bundle
    // #[arg]
    bundle: String,

    /// The length of time, in seconds, between each run of the service
    #[arg(long)]
    start_interval: Option<u64>,

  },
  
  #[clap(about = "Uninstalls the mac service")]
  Uninstall {

    /// The name of the service
    #[arg(long)]
    bundle: String,

  },
}


impl Command {
  pub fn run(self) -> Result<(), Box<dyn error::Error>>  {
    match self.action {
      Action::Install {bundle,start_interval} => {
        commands::install(bundle, start_interval)
      },
      Action::Uninstall { bundle } => {
        todo!("Uninstall");
      }
    }
  }
}




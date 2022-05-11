mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use anyhow::Result;
use args::{Commands, PngMeArgs};
use clap::StructOpt;
fn main() -> Result<()> {
    let args = PngMeArgs::parse();
    match &args.command {
        Commands::Encode(arg) => {
            println!("{:?}", &arg);
            Ok(())
        }
        Commands::Decode(arg) => {
            println!("{:?}", &arg);
            Ok(())
        }
        Commands::Remove(arg) => {
            println!("{:?}", &arg);
            Ok(())
        }
        Commands::Print(arg) => {
            println!("{:?}", &arg);
            Ok(())
        }
    }
}

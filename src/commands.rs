use std::{fs, str::FromStr};

use anyhow::Result;

use crate::{
    args::{Commands, DecodeArgs, EncodeArgs, PngMeArgs, PrintArgs, RemoveArgs},
    chunk::Chunk,
    chunk_type::ChunkType,
    png::Png,
};
use clap::StructOpt;
pub struct Command {}

impl Command {
    pub fn handle() -> Result<()> {
        let args = PngMeArgs::parse();
        match &args.command {
            Commands::Encode(arg) => Self::handle_encode(arg),
            Commands::Decode(arg) => Self::handle_decode(arg),
            Commands::Remove(arg) => Self::handle_remove(arg),
            Commands::Print(arg) => Self::handle_print(arg),
        }
    }

    fn handle_encode(args: &EncodeArgs) -> Result<()> {
        let EncodeArgs {
            file_path,
            chunk_type,
            message,
            output_file,
        } = args;
        let mut png = Png::from_file(file_path)?;
        let chunk_t = ChunkType::from_str(chunk_type)?;
        let data = message.bytes().collect();

        let chunk = Chunk::new(chunk_t, data);

        png.append_chunk(chunk);

        let output = match output_file {
            Some(output) => output,
            None => file_path,
        };
        fs::write(output, png.as_bytes())?;
        println!("Done!");
        Ok(())
    }

    fn handle_decode(args: &DecodeArgs) -> Result<()> {
        let DecodeArgs {
            file_path,
            chunk_type,
        } = args;
        let png = Png::from_file(file_path)?;
        let chunk = png.chunk_by_type(chunk_type);
        match chunk {
            Some(c) => println!("data: {}", c.data_as_string().unwrap()),
            None => println!("not exists"),
        }
        Ok(())
    }

    fn handle_remove(args: &RemoveArgs) -> Result<()> {
        let RemoveArgs {
            file_path,
            chunk_type,
        } = args;
        let mut png = Png::from_file(file_path)?;
        png.remove_chunk(chunk_type)?;

        fs::write(file_path, png.as_bytes())?;

        println!("Done!");
        Ok(())
    }

    fn handle_print(args: &PrintArgs) -> Result<()> {
        let PrintArgs { file_path } = args;
        let png = Png::from_file(file_path)?;
        println!("{}", png.to_string());
        Ok(())
    }
}

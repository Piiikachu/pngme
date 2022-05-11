use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(name = "pngme")]
#[clap(author, version, about, long_about = None)]
pub struct PngMeArgs {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(clap::Args, Debug)]
pub struct EncodeArgs {
    file_path: PathBuf,
    chunk_type: String,
    message: String,
    #[clap(short, long, parse(from_os_str), value_name = "OUTPUT")]
    output_file: Option<PathBuf>,
}

#[derive(clap::Args, Debug)]
pub struct DecodeArgs {
    file_path: PathBuf,
    chunk_type: String,
}

#[derive(clap::Args, Debug)]
pub struct RemoveArgs {
    file_path: PathBuf,
    chunk_type: String,
}

#[derive(clap::Args, Debug)]
pub struct PrintArgs {
    file_path: PathBuf,
}

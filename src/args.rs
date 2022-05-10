use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(name = "pngme")]
#[clap(bin_name = "pngme")]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(clap::Args)]
#[clap(author, version, about, long_about = None)]
pub struct EncodeArgs {
    file_path: PathBuf,
    chunk_type: String,
    message: String,
    output_file: Option<PathBuf>,
}

#[derive(clap::Args)]
#[clap(author, version, about, long_about = None)]
pub struct DecodeArgs {
    file_path: PathBuf,
    chunk_type: String,
}

#[derive(clap::Args)]
#[clap(author, version, about, long_about = None)]
pub struct RemoveArgs {
    file_path: PathBuf,
    chunk_type: String,
}

#[derive(clap::Args)]
#[clap(author, version, about, long_about = None)]
pub struct PrintArgs {
    file_path: PathBuf,
}

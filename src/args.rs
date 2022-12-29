use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command :PngMeArgs,
}

#[derive(Subcommand)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Args)]
pub struct EncodeArgs {
    #[clap(value_parser)]
    pub file_path: PathBuf,

    #[clap(value_parser)]
    pub chunk_type: String,

    #[clap(value_parser)]
    pub message: String,

    #[clap(value_parser)]
    pub output: Option<PathBuf>,
}

#[derive(Args)]
pub struct DecodeArgs {
    // Write me!
    #[clap(value_parser)]
    pub file_path: PathBuf,

    #[clap(value_parser)]
    pub chunk_type: String,
}

#[derive(Args)]
pub struct RemoveArgs {
    // Write me!
    #[clap(value_parser)]
    pub file_path: PathBuf,

    #[clap(value_parser)]
    pub chunk_type: String,
}

#[derive(Args)]
pub struct PrintArgs {
    #[clap(value_parser)]
    pub file_path: PathBuf,

}

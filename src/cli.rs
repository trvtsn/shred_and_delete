use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "shred_and_delete")]
pub struct Args {
    #[command(subcommand)]
    pub method: Method,

    #[arg(short, long)]
    pub path: String,
}

#[derive(Subcommand)]
pub enum Method {
    Trash,
    Delete,
}

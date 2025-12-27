use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = "MoonFS is a Distributed File System")]
pub struct CLI {

    // --p for short, --path for long 
    #[arg(short, long)]
    pub path: Option<PathBuf>, 

    #[arg(short, long)]
    server: String, 
}

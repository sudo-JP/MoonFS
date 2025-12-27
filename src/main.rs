use clap::Parser;
use moonfs::cli::{CLI};

fn main() {
    let cli = CLI::parse();

    // TODO: extract the path from cli, throw error if path doesn't exist
}

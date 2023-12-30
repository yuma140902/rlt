use clap::Parser;
use rlt::cli::{Args, Commands};

mod build;

fn main() {
    let args = Args::parse();

    match args.subcommand {
        Commands::Build(args) => build::build(args),
    }
}

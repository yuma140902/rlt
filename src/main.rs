use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    subcommand: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[clap(alias = "b", about = "Build the document. [alias: b]")]
    Build(BuildArgs),
}

#[derive(Debug, Parser)]
struct BuildArgs {}

impl Default for BuildArgs {
    fn default() -> Self {
        Self {}
    }
}

fn main() {
    let args = Args::parse();

    dbg!(&args);
}

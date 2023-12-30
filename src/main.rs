use std::process::Command;

use clap::{Parser, Subcommand, ValueEnum};

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
struct BuildArgs {
    #[clap(
        short = 'e',
        long = "latex-engine",
        help = "LaTeX engine",
        default_value = "uplatex"
    )]
    latex_engine: Option<LatexEngine>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum LatexEngine {
    #[clap(name = "uplatex")]
    upLaTeX,
}

fn main() {
    let args = Args::parse();

    match args.subcommand {
        Commands::Build(args) => build(args),
    }
}

fn build(_args: BuildArgs) {
    let mut command = Command::new("uplatex");
    command.args(&[
        "main.ltx",
        "-halt-on-error",
        "-interaction=nonstopmode",
        "-file-line-error",
    ]);
    let mut process = command.spawn().expect("Failed to run uplatex");
    process.wait().expect("Failed to wait uplatex");

    let mut command = Command::new("uplatex");
    command.args(&[
        "main.ltx",
        "-halt-on-error",
        "-interaction=nonstopmode",
        "-file-line-error",
    ]);
    let mut process = command.spawn().expect("Failed to run uplatex");
    process.wait().expect("Failed to wait uplatex");

    let mut command = Command::new("uplatex");
    command.args(&[
        "main.ltx",
        "-halt-on-error",
        "-interaction=nonstopmode",
        "-file-line-error",
    ]);
    let mut process = command.spawn().expect("Failed to run uplatex");
    process.wait().expect("Failed to wait uplatex");

    println!("done");
}

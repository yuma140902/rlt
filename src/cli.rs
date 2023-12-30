use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub subcommand: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(alias = "b", about = "Build the document. [alias: b]")]
    Build(BuildArgs),
}

#[derive(Debug, Parser)]
pub struct BuildArgs {
    #[clap(
        short = 'e',
        long = "latex-engine",
        help = "LaTeX engine",
        default_value = "uplatex"
    )]
    pub latex_engine: Option<LatexEngine>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum LatexEngine {
    #[clap(name = "uplatex")]
    upLaTeX,
}

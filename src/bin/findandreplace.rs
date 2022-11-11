use std::{path::PathBuf, str::FromStr};

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    directory: String,
    #[clap(short, long)]
    config_path: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    let config_path = match cli.config_path {
        Some(p) => p,
        None => PathBuf::from_str("./replace.yaml").unwrap(),
    };
    mk_findandreplace::findanreplace(&cli.directory, &config_path);
}

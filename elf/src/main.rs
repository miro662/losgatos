use std::{error::Error, fs, path::PathBuf};

use clap::Parser;
use elf::Elf;

#[derive(Parser, Debug)]
struct Cli {
    #[arg()]
    path: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let file = fs::read(args.path)?;
    let elf = Elf::from_bytes(&file)?;
    println!("{}", elf);
    Ok(())
}

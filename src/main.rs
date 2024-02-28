use std::{
    io::{self, Read},
    process,
};

use arboard::Clipboard;
use clap::{Parser, Subcommand};

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Parser)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Clone, Copy, Debug, Subcommand)]
enum Command {
    #[clap(alias = "c")]
    Copy {
        #[arg(short, long)]
        trim: bool,
    },
    #[clap(alias = "p", alias = "v")]
    Paste,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Clipboard(#[from] arboard::Error),
}

fn main() {
    if let Err(e) = run(&Args::parse()) {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run(args: &Args) -> Result<()> {
    match args.command {
        Command::Copy { trim } => set(trim),
        Command::Paste => get(),
    }
}

fn set(trim: bool) -> Result<()> {
    let mut buf = String::new();
    let text = io::stdin().lock().read_to_string(&mut buf).map(|_| buf)?;

    if trim {
        Clipboard::new()?.set_text(text.trim())?;
    } else {
        Clipboard::new()?.set_text(text)?;
    }

    Ok(())
}

fn get() -> Result<()> {
    let text = Clipboard::new()?.get_text()?;
    println!("{text}");
    Ok(())
}

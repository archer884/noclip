mod error;
mod opt;

use crate::{
    error::Result,
    opt::{Mode, Opts},
};
use clipboard::{ClipboardContext, ClipboardProvider};
use std::{
    io::{self, stdin, stdout, Read, Write},
    time::Duration,
};

fn main() {
    let opts = Opts::parse();
    if let Err(e) = run(&opts) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run(opts: &Opts) -> anyhow::Result<()> {
    match opts.mode() {
        Mode::Read => read(),
        Mode::Write { trim } => write(trim),
    }
}

fn read() -> anyhow::Result<()> {
    let clipboard = x11_clipboard::Clipboard::new()?;
    let content = clipboard.load(
        clipboard.setter.atoms.clipboard,
        clipboard.setter.atoms.utf8_string,
        clipboard.setter.atoms.property,
        Duration::from_secs(3),
    )?;

    let text = dbg!(String::from_utf8_lossy(&content));
    stdout().write_all(text.as_bytes())?;
    stdout().flush()?;
    Ok(())
}

fn write(trim: bool) -> anyhow::Result<()> {
    let mut content = String::new();
    stdin().read_to_string(&mut content)?;

    if trim {
        content.truncate(content.trim_end().len());
    }

    let clipboard = x11_clipboard::Clipboard::new()?;
    clipboard.store(
        clipboard.setter.atoms.clipboard,
        clipboard.setter.atoms.utf8_string,
        dbg!(content),
    )?;
    Ok(())
}

// fn read(ctx: &mut ClipboardContext) -> Result<()> {
//     let mut stdout = io::stdout();
//     stdout.write_all(ctx.get_contents()?.as_ref())?;
//     stdout.flush()?;
//     Ok(())
// }

// fn write(ctx: &mut ClipboardContext, trim: bool) -> Result<()> {
//     let mut content = String::new();
//     io::stdin().read_to_string(&mut content)?;

//     if trim {
//         content.truncate(content.trim_end().len());
//     }

//     dbg!(&content);
//     ctx.set_contents(content)?;
//     ctx
//     Ok(())
// }

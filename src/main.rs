mod error;
mod opt;

use crate::{
    error::Result,
    opt::{Mode, Opt},
};
use clipboard::{ClipboardContext, ClipboardProvider};
use std::io::{self, Read, Write};

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    match opt.mode() {
        Mode::Read => read(&mut ctx),
        Mode::Write => write(&mut ctx),
    }
}

fn read(ctx: &mut ClipboardContext) -> Result<()> {
    let mut stdout = io::stdout();
    stdout.write_all(ctx.get_contents()?.as_ref())?;
    stdout.flush()?;
    Ok(())
}

fn write(ctx: &mut ClipboardContext) -> Result<()> {
    let mut content = String::new();
    io::stdin().read_to_string(&mut content)?;
    ctx.set_contents(content)?;
    Ok(())
}

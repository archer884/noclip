use std::error::Error;
use std::io::{self, Write};
use clipboard::{ClipboardContext, ClipboardProvider};

fn main() -> Result<(), Box<Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;

    if let Ok(content) = ctx.get_contents() {
        io::stdout().write(content.as_ref())?;
        io::stdout().flush()?;
    }

    Ok(())
}

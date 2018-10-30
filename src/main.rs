use clipboard::{ClipboardContext, ClipboardProvider};
use std::env;
use std::error::Error;
use std::io::{self, Read, Write};

enum Mode {
    Read,
    Write,
}

impl Mode {
    fn from_args() -> Result<Mode, &'static str> {
        let mut has_input = false;
        let mut has_output = false;

        for item in env::args().skip(1) {
            match item.as_ref() {
                "c" | "copy" => has_input = true,
                "v" | "paste" => has_output = true,

                _ => return Err("Invalid mode setting"),
            }
        }

        match (has_input, has_output) {
            (true, false) => Ok(Mode::Write),
            (false, true) => Ok(Mode::Read),

            _ => Err("Invalid mode setting"),
        }
    }
}

fn main() -> Result<(), Box<Error>> {
    let mode = Mode::from_args()?;
    
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    match mode {
        Mode::Read => read(&mut ctx),
        Mode::Write => write(&mut ctx),
    }
}

fn read(ctx: &mut ClipboardContext) -> Result<(), Box<Error>> {
    let mut stdout = io::stdout();

    stdout.write_all(ctx.get_contents()?.as_ref())?;
    stdout.flush()?;

    Ok(())
}

fn write(ctx: &mut ClipboardContext) -> Result<(), Box<Error>> {
    let mut content = String::new();

    io::stdin().read_to_string(&mut content)?;
    ctx.set_contents(content)?;

    Ok(())    
}

use std::str::FromStr;

use structopt::StructOpt;

use crate::error::ParseModeError;

#[derive(Copy, Clone, Debug, StructOpt)]
pub struct Opts {
    mode: Mode,
}

impl Opts {
    pub fn parse() -> Self {
        StructOpt::from_args()
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Mode {
    Read,
    Write { trim: bool },
}

impl FromStr for Mode {
    type Err = ParseModeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "c" | "copy" => Ok(Mode::Write { trim: false }),
            "t" | "trim" => Ok(Mode::Write { trim: true }),
            "v" | "p" | "paste" => Ok(Mode::Read),

            _ => Err(ParseModeError),
        }
    }
}

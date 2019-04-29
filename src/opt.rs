use crate::error::ParseModeError;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Copy, Clone, Debug, StructOpt)]
pub struct Opt {
    mode: Mode,
}

impl Opt {
    pub fn from_args() -> Self {
        StructOpt::from_args()
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Mode {
    Read,
    Write,
}

impl FromStr for Mode {
    type Err = ParseModeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "c" | "copy" => Ok(Mode::Write),
            "v" | "p" | "paste" => Ok(Mode::Read),

            _ => Err(ParseModeError),
        }
    }
}

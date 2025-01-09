mod base64;
mod csv;
mod genpass;
mod http;
mod text;

use std::path::{Path, PathBuf};

pub use self::{base64::*, csv::*, genpass::*, http::*, text::*};
use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Parser, Debug)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum SubCommand {
    #[command(name = "csv", about = "show CSV, convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "generate random pass")]
    Genpass(GenpassOpts),
    #[command(subcommand, about = "base64 encode/decode")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "text sign/verify")]
    Text(TextSubCommand),
    #[command(subcommand, about = "http server")]
    Http(HttpSubCommand),
}

pub fn verify_file(filename: &str) -> Result<String, &'static str> {
    // if input is '-' or file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exists")
    }
}

pub fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exists or is not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File does not exists"));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exist"), Err("File does not exists"));
    }
}

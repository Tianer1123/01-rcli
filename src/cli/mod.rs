mod base64;
mod csv;
mod genpass;
mod http;
mod text;

use std::path::{Path, PathBuf};

use crate::CmdExecutor;

pub use self::{
    base64::{Base64Format, Base64SubCommand},
    csv::OutputFormat,
    http::HttpSubCommand,
    text::{TextSignFormat, TextSubCommand},
};
use self::{csv::CsvOpts, genpass::GenpassOpts};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
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

impl CmdExecutor for SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            SubCommand::Csv(opts) => opts.execute().await,
            SubCommand::Genpass(opts) => opts.execute().await,
            SubCommand::Base64(opts) => opts.execute().await,
            SubCommand::Text(opts) => opts.execute().await,
            SubCommand::Http(opts) => opts.execute().await,
        }
    }
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

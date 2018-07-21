use std::{fs, io, path};
use structopt;

#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
pub struct Cli {
  /// Path to a markdown file
  #[structopt(parse(from_os_str))]
  path: path::PathBuf,
}

impl Cli {
  /// Get the file path.
  #[inline]
  pub fn path(&self) -> &path::PathBuf {
    &self.path
  }

  /// Read the file at the file path to a string.
  #[inline]
  pub fn read_to_string(&self) -> io::Result<String> {
    fs::read_to_string(&self.path)
  }
}

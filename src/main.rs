#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

extern crate playground_markdown;
extern crate pulldown_cmark;
extern crate structopt;

use playground_markdown::cli::Cli;
use pulldown_cmark::{Event, Parser, Tag};
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Cli::from_args();
  let raw_md = args.read_to_string()?;

  let mut should_print = false;
  let mut header_level = 0;

  for event in Parser::new(&raw_md) {
    if should_print {
      should_print = false;
      if let Event::Text(txt) = &event {
        println!("Header {} {}", header_level, *txt);
      }
    }

    if let Event::Start(tag) = event {
      if let Tag::Header(level) = tag {
        header_level = level;
        should_print = true;
      }
    }
  }

  Ok(())
}

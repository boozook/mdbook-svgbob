#[macro_use]
extern crate log;
extern crate env_logger;
extern crate mdbook;
#[macro_use]
extern crate clap;

use std::process;
use std::error::Error;
use mdbook::preprocess::Preprocessor;

mod cli;
mod cfg;
mod preprocessor;
mod svgbob;


pub type Result<Ok = (), Err = Box<dyn Error>> = std::result::Result<Ok, Err>;


fn main() -> Result {
	let opts = cli::init()?;

	// handle supports or processing:
	if let Some(cli::Commands::Supports { renderer }) = opts.command {
		let bob = preprocessor::Bob::new();
		// Signal whether the renderer is supported by exiting with 1 or 0.
		if bob.supports_renderer(&renderer) {
			process::exit(0);
		} else {
			process::exit(1);
		}
	}

	Ok(())
}

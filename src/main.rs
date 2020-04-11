#[macro_use]
extern crate log;
extern crate env_logger;
extern crate mdbook;
extern crate clap;

use std::process;
use std::error::Error;

use clap::{App, Arg, ArgMatches, SubCommand};
use mdbook::preprocess::Preprocessor;

mod preprocessor;
mod svgbob;


pub type Result<Ok = (), Err = Box<dyn Error>> = std::result::Result<Ok, Err>;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const NAME: &str = env!("CARGO_PKG_NAME");
// const DESCRIPTION_SHORT: &str = env!("CARGO_PKG_DESCRIPTION");
const DESCRIPTION: &str = concat!(env!("CARGO_PKG_DESCRIPTION"), "\n", env!("CARGO_PKG_HOMEPAGE"));


pub fn init_cli() -> Result<ArgMatches<'static>> {
	env_logger::try_init()?;

	Ok(App::new(NAME).version(VERSION)
	.author(AUTHORS)
	    .about(DESCRIPTION)
	    .subcommand(
	       SubCommand::with_name("supports")
	                  .arg(Arg::with_name("renderer").required(true))
	                  .about("Check whether a renderer is supported by this preprocessor"),
	).get_matches())
}


fn main() -> Result {
	let opts = init_cli()?;
	let bob = preprocessor::Bob::new();

	// handle supports or processsing:
	if let Some(sub_args) = opts.subcommand_matches("supports") {
		let renderer = sub_args.value_of("renderer").expect("Required argument");
		let supported = bob.supports_renderer(&renderer);

		// Signal whether the renderer is supported by exiting with 1 or 0.
		if supported {
			process::exit(0);
		} else {
			process::exit(1);
		}
	} else if let Err(e) = bob.handle_preprocessing() {
		error!("{}", e);
		process::exit(1);
	}
	Ok(())
}

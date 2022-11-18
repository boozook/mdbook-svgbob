use clap::{Parser, Subcommand};
use crate::Result;


const DESCRIPTION: &str = concat!(crate_description!(), "\n", env!("CARGO_PKG_HOMEPAGE"));


#[derive(Parser)]
#[command(author, version, about=DESCRIPTION, long_about = None)]
pub struct Opts {
	#[command(subcommand)]
	pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
	/// Check whether a renderer is supported by this preprocessor
	Supports { renderer: String },
}


pub fn init() -> Result<Opts> {
	env_logger::try_init()?;
	Opts::try_parse().map_err(Into::into).map_err(|err| {
		                                     println!("{}", err);
		                                     err
	                                     })
}

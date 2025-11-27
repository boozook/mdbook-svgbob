use log::error;
use mdbook_preprocessor::Preprocessor;
use std::error::Error;
use std::process;

mod cfg;
mod cli;
mod preprocessor;
mod svgbob;

pub type Result<Ok = (), Err = Box<dyn Error>> = std::result::Result<Ok, Err>;

fn main() -> Result {
    let opts = cli::init()?;

    // handle supports or processing:
    let bob = preprocessor::Bob::new();
    if let Some(cli::Commands::Supports { renderer }) = opts.command {
        // Signal whether the renderer is supported by exiting with 1 or 0.
        if bob.supports_renderer(&renderer)? {
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

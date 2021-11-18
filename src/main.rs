use anyhow::{bail, Context, Result};
use mdbook::preprocess::Preprocessor;
use structopt::StructOpt;

mod preprocessor;
mod svgbob;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    cmd: Option<SubCommand>,
}

#[derive(Debug, StructOpt)]
enum SubCommand {
    #[structopt(help = "Check whether a renderer is supported by this preprocessor")]
    Supports { renderer: String },
}

fn main() -> Result<()> {
    env_logger::try_init().with_context(|| "failed to initialize logger")?;

    let bob = preprocessor::Bob::new();

    match Opt::from_args().cmd {
        // checks whether or not a given renderer is supported
        Some(SubCommand::Supports { renderer }) => {
            if bob.supports_renderer(&renderer) {
                Ok(())
            } else {
                bail!("svgbob does not support this renderer")
            }
        }
        // preprocesses the input
        None => bob
            .handle_preprocessing()
            .with_context(|| "failed to preprocess svgbob drawing"),
    }
}

use anyhow::Result;
use clap::{Parser, Subcommand};
use mdbook::errors::Error;
use mdbook::preprocess::CmdPreprocessor;
use mdbook::preprocess::Preprocessor;
use mdbook_github_authors::GithubAuthorsPreprocessor;
use std::io;
use std::process;

/// mdbook preprocessor to create an authors section for every Chapter
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Check whether a renderer is supported by this preprocessor
    Supports { renderer: String },
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let cli = Cli::parse();
    if let Err(error) = run(cli) {
        log::error!("Fatal error: {}", error);
        for error in error.chain() {
            log::error!("  - {}", error);
        }
        process::exit(1);
    }
}

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        None => handle_preprocessing(),
        Some(Commands::Supports { renderer }) => {
            handle_supports(renderer);
        }
    }
}

fn handle_supports(renderer: String) -> ! {
    let supported = GithubAuthorsPreprocessor.supports_renderer(&renderer);

    // Signal whether the renderer is supported by exiting with 1 or 0.
    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

fn handle_preprocessing() -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    if ctx.mdbook_version != mdbook::MDBOOK_VERSION {
        eprintln!(
            "Warning: The mdbook-github-authors preprocessor was built against version \
             {} of mdbook, but we're being called from version {}",
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = GithubAuthorsPreprocessor.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

mod commands;
mod templates;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "kit")]
#[command(about = "A CLI for scaffolding Kit web applications", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Kit project
    New {
        /// The name of the project to create
        name: Option<String>,

        /// Skip all prompts and use defaults
        #[arg(long)]
        no_interaction: bool,

        /// Skip git initialization
        #[arg(long)]
        no_git: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New {
            name,
            no_interaction,
            no_git,
        } => {
            commands::new::run(name, no_interaction, no_git);
        }
    }
}

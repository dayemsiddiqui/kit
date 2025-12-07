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
    /// Start the development servers (backend + frontend)
    Serve {
        /// Backend port (default: 8000)
        #[arg(long, short = 'p', default_value = "8000")]
        port: u16,

        /// Frontend port (default: 5173)
        #[arg(long, default_value = "5173")]
        frontend_port: u16,

        /// Only start backend server
        #[arg(long)]
        backend_only: bool,

        /// Only start frontend server
        #[arg(long)]
        frontend_only: bool,
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
        Commands::Serve {
            port,
            frontend_port,
            backend_only,
            frontend_only,
        } => {
            commands::serve::run(port, frontend_port, backend_only, frontend_only);
        }
    }
}

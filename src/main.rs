use clap::{Parser, Subcommand};

mod logic;
#[derive(Parser)]
#[command(name = "Feather CLI")]
#[command(version, author, about, long_about = None)]
#[command(about = "The CLI Tooling for the Feather Web Framework", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    #[command(name = "new", about = "Create a New Feather Project")]
    New { name: String },
    #[command(name = "dev", about = "Start the Hot-Reloading Dev Server")]
    /// Start the development server
    /// This command will start the development server for the Feather framework.
    /// It will watch for changes in the source code and automatically reload the server.
    Dev,
    #[command(name = "build", about = "Build the Feather Project")]
    Build,
    
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::New { name } => {
            println!("Creating a new Feather project: {}", name);
            logic::create_new_project(name.as_str());
        }
        Commands::Dev => {
            println!("Starting the Feather development server...");
            logic::start_dev_server().unwrap();
        }
        Commands::Build => {
            println!("Building the Feather project...");
            logic::build_project().unwrap();
        }
        
    }
}

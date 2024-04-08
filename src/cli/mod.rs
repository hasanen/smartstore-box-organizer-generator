use clap::{Parser, Subcommand};

pub mod generate;
use generate::RackGenerationArgs;

#[derive(Parser, Debug)]
#[structopt(name = "Container rack")]
#[command(arg_required_else_help(true))]
struct CommandLineCommands {
    /// Command to use: hours, integrations etc
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate SVG
    Generate(RackGenerationArgs),
    /// List supported containers
    Containers,
}

pub fn run() {
    let args = CommandLineCommands::parse();

    match &args.command {
        Commands::Generate(args) => {
            generate::svg(args);
        }
        Commands::Containers => {
            println!("list containers - TBD");
        }
    }
}

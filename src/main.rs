use clap::{Parser, Subcommand};
use ec::{scaffold_quest, solve_quest};

#[derive(Parser)]
#[command(name = "Everybody Codes")]
#[command(about = "Everybody Codes CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scaffold a quest (creates files and downloads input)
    Scaffold {
        /// Event/Story
        #[clap(short, long)]
        event: Option<String>,
        /// Quest number (1-20)
        quest: Option<u8>,
        /// Part number (1-3), defaults to next unsolved part
        part: Option<u8>,
    },
    /// Solve a quest (runs the solution)
    Solve {
        /// Event/Story
        #[clap(short, long)]
        event: Option<String>,
        /// Quest number (1-20)
        quest: Option<u8>,
        /// Part number (1-3), if provided will submit this part
        part: Option<u8>,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Scaffold { event, quest, part } => {
            if let Some(p) = part
                && !(1..=3).contains(&p)
            {
                eprintln!("Part must be between 1 and 3");
                std::process::exit(1);
            }
            scaffold_quest(event, quest, part)
        }
        Commands::Solve { event, quest, part } => {
            if let Some(p) = part
                && !(1..=3).contains(&p)
            {
                eprintln!("Part must be between 1 and 3");
                std::process::exit(1);
            }
            let submit = part.is_some();
            solve_quest(event, quest, part, submit)
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "hars", version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand, Debug)]
enum Resources {
    Lb,
}
#[derive(Subcommand, Debug)]
enum Commands {
    Create {
        #[command(subcommand)]
        resource: Resources,
    },
    Delete {
        #[command(subcommand)]
        resource: Resources,
    },
    Get {
        #[command(subcommand)]
        resource: Resources,
    },
    Update {
        #[command(subcommand)]
        resource: Resources,
    },
    Start {
        #[command(subcommand)]
        resource: Resources,
    },
    Stop {
        #[command(subcommand)]
        resource: Resources,
    },
}
fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Create { resource } => match resource {
            Resources::Lb => {
                println!("Creating load balancer");
            }
        },
        Commands::Delete { resource } => match resource {
            Resources::Lb => {
                println!("Deleting load balancer");
            }
        },
        Commands::Get { resource } => match resource {
            Resources::Lb => {
                println!("Getting load balancer");
            }
        },
        Commands::Update { resource } => match resource {
            Resources::Lb => {
                println!("Updating load balancer");
            }
        },
        Commands::Start { resource } => match resource {
            Resources::Lb => {
                println!("Starting load balancer");
            }
        },
        Commands::Stop { resource } => match resource {
            Resources::Lb => {
                println!("Stopping load balancer");
            }
        },
    }
}
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "hearth", about = "Hearth CLI — A shared hub for your memories and collections")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Initialize Hearth client
    Init {
        /// Server URL
        #[arg(long)]
        server: String,

        /// User email
        #[arg(long)]
        email: String,
    },

    /// Watch a directory and sync changes
    Watch {
        /// Directory to watch
        #[arg(value_name = "PATH")]
        path: String,

        /// Collection name
        #[arg(long)]
        collection: Option<String>,
    },

    /// List collections
    List,

    /// Show sync status
    Status,

    /// Invite a member to a collection
    Invite {
        /// Email address to invite
        email: String,

        /// Collection name or ID
        #[arg(long)]
        collection: String,

        /// Role: owner, editor, viewer
        #[arg(long, default_value = "viewer")]
        role: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Init { server, email } => {
            println!("Initializing Hearth client...");
            println!("  Server: {}", server);
            println!("  Email: {}", email);
            println!("TODO: Implement init command");
        }
        Command::Watch { path, collection } => {
            println!("Watching directory: {}", path);
            if let Some(col) = collection {
                println!("  Collection: {}", col);
            }
            println!("TODO: Implement watch command");
        }
        Command::List => {
            println!("TODO: List collections");
        }
        Command::Status => {
            println!("TODO: Show sync status");
        }
        Command::Invite {
            email,
            collection,
            role,
        } => {
            println!("Inviting {} to collection '{}' as {}", email, collection, role);
            println!("TODO: Implement invite command");
        }
    }
}

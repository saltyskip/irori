use clap::{Parser, Subcommand};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

use hearth::app::AppState;
use hearth::core::config::Config;
use hearth::core::db;
use hearth::core::storage;

#[derive(Parser)]
#[command(name = "hearth", about = "A shared hub for your memories and collections")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Run database migrations
    Migrate {
        /// Migration name (e.g. m001_init_schema)
        #[arg(long)]
        name: Option<String>,

        /// List available migrations
        #[arg(long)]
        list: bool,

        /// Actually apply the migration (default is dry run)
        #[arg(long)]
        apply: bool,
    },

    /// Start the server
    Serve {
        /// Bind address (default: 0.0.0.0:3000)
        #[arg(long)]
        host: Option<String>,

        /// Port (default: 3000)
        #[arg(long)]
        port: Option<u16>,
    },
}

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    let cfg = Config::from_env();

    // Initialize tracing
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new("hearth=info,tower_http=warn,hyper=warn,sqlx=warn")
    });
    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Some(Command::Migrate { list: true, .. }) => {
            println!("Available migrations:");
            println!("  m001_init_schema — Initialize database schema");
        }
        Some(Command::Migrate { name: Some(_name), .. }) => {
            eprintln!("Migration system not yet implemented");
            std::process::exit(1);
        }
        Some(Command::Migrate { .. }) => {
            eprintln!("Provide --name <migration> or --list");
            std::process::exit(1);
        }
        Some(Command::Serve { host, port }) => {
            let host = host.unwrap_or_else(|| "0.0.0.0".to_string());
            let port = port.unwrap_or(3000);
            run_server(cfg, &host, port).await;
        }
        None => {
            // Default: run server
            run_server(cfg, "0.0.0.0", 3000).await;
        }
    }
}

async fn run_server(cfg: Config, host: &str, port: u16) {
    tracing::info!("Starting Hearth server at {}:{}", host, port);

    // Connect to database
    let pool = match db::connect(&cfg.database_url).await {
        Ok(pool) => {
            tracing::info!("Connected to database");
            pool
        }
        Err(e) => {
            tracing::error!("Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };

    // Initialize storage backend
    let storage_backend = storage::create_backend(&cfg).await;

    // Create app state
    let app_state = Arc::new(AppState {
        db_pool: pool,
        config: cfg,
        storage: Arc::new(storage_backend),
    });

    // Build router
    let app = hearth::api::router(app_state.clone())
        .layer(CorsLayer::permissive());

    // Bind and serve
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();

    tracing::info!("Server listening on {}:{}", host, port);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install CTRL+C signal handler");
    };

    ctrl_c.await;
    tracing::info!("Shutdown signal received");
}

use clap::{Parser, Subcommand};

mod commands;
mod ollama;

#[derive(Parser)]
#[command(name = "shc-tool")]
#[command(about = "Support Help CLI Tool")]
#[command(version = "0.1.1")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show local IP addresses and gateway information
    Ip,
    /// Send ping to a target
    Ping {
        /// Target IP address or hostname
        target: String,
    },
    /// Collect and analyze system log files
    Logs {
        /// Create ZIP archive of log files
        #[arg(long)]
        zip: bool,
        /// Analyze logs with AI (Ollama)
        #[arg(long)]
        analyze: bool,
        /// Question for AI analysis
        #[arg(long)]
        query: Option<String>,
        /// Number of lines to analyze (default: 200)
        #[arg(long, default_value = "200")]
        lines: usize,
        /// Ollama model (default: gemma2:2b)
        #[arg(long)]
        model: Option<String>,
        /// Specific path to log file
        #[arg(long)]
        file: Option<String>,
    },
    /// Perform simple network scan
    Scan {
        /// Target network or host
        target: String,
    },
    /// Test tool functionality
    Test {
        /// Test all available commands
        #[arg(long)]
        all: bool,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ip => {
            commands::ip::handle_ip_command();
        }
        Commands::Ping { target } => {
            commands::ping::handle_ping_command(&target);
        }
        Commands::Logs { zip, analyze, query, lines, model, file } => {
            commands::logs::handle_logs_command(zip, analyze, query, lines, model, file).await;
        }
        Commands::Scan { target } => {
            commands::scan::handle_scan_command(&target);
        }
        Commands::Test { all } => {
            commands::test::handle_test_command(all);
        }
    }
}
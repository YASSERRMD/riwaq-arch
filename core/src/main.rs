//! Riwaq Arch CLI - Codebase Analysis and Documentation Generation
//!
//! A command-line tool for analyzing codebases and generating documentation
//! using LLM-powered insights.

use clap::{Parser, Subcommand};
use riwaq_core::logging;
use std::path::PathBuf;
use tracing::info;

/// Riwaq Arch - Intelligent Codebase Documentation Generator
#[derive(Parser, Debug)]
#[command(name = "riwaq-arch")]
#[command(author = "YASSERRMD <arafath.yasser@gmail.com>")]
#[command(version)]
#[command(about = "Analyze codebases and generate living documentation using LLM", long_about = None)]
struct Cli {
    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Output format (json, text)
    #[arg(short, long, global = true, default_value = "text")]
    format: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Analyze a codebase and output a snapshot
    Analyze {
        /// Path to the codebase to analyze
        #[arg(short, long, default_value = ".")]
        path: PathBuf,

        /// Output file for the analysis snapshot (JSON)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Maximum number of git commits to analyze
        #[arg(long, default_value = "500")]
        max_commits: usize,

        /// Include test files in analysis
        #[arg(long, default_value = "false")]
        include_tests: bool,
    },

    /// Generate documentation from an analyzed codebase
    Docgen {
        /// Path to the codebase to analyze
        #[arg(short, long, default_value = ".")]
        path: PathBuf,

        /// Output directory for generated documentation
        #[arg(short, long, default_value = "./generated-docs")]
        output: PathBuf,

        /// Skip LLM calls (generate structure only)
        #[arg(long)]
        skip_llm: bool,
    },

    /// Ask a question about the codebase
    Ask {
        /// Path to the codebase
        #[arg(short, long, default_value = ".")]
        path: PathBuf,

        /// Question to ask
        #[arg(short, long)]
        question: String,

        /// Enable streaming response
        #[arg(long)]
        stream: bool,
    },

    /// Start an HTTP server for the VS Code extension
    Serve {
        /// Port to listen on
        #[arg(short, long, default_value = "9527")]
        port: u16,

        /// Host to bind to
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from .env file if present
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    // Initialize logging
    logging::init_logging(cli.verbose)?;

    info!(version = riwaq_core::VERSION, "Starting Riwaq");

    match cli.command {
        Commands::Analyze {
            path,
            output,
            max_commits,
            include_tests,
        } => {
            info!(?path, "Analyzing codebase");
            run_analyze(path, output, max_commits, include_tests).await?;
        }
        Commands::Docgen {
            path,
            output,
            skip_llm,
        } => {
            info!(?path, ?output, "Generating documentation");
            run_docgen(path, output, skip_llm).await?;
        }
        Commands::Ask {
            path,
            question,
            stream,
        } => {
            info!(?path, %question, "Answering question");
            run_ask(path, question, stream).await?;
        }
        Commands::Serve { port, host } => {
            info!(%host, %port, "Starting server");
            run_serve(host, port).await?;
        }
    }

    Ok(())
}

async fn run_analyze(
    path: PathBuf,
    output: Option<PathBuf>,
    max_commits: usize,
    include_tests: bool,
) -> anyhow::Result<()> {
    use riwaq_arch_core::analysis::analyzer::CodebaseAnalyzer;

    let analyzer = CodebaseAnalyzer::new(&path)
        .with_max_commits(max_commits)
        .with_include_tests(include_tests);

    let snapshot = analyzer.analyze().await?;

    if let Some(output_path) = output {
        let json = serde_json::to_string_pretty(&snapshot)?;
        std::fs::write(&output_path, json)?;
        info!(?output_path, "Snapshot written to file");
    } else {
        println!("{}", serde_json::to_string_pretty(&snapshot)?);
    }

    info!(
        files = snapshot.files.len(),
        modules = snapshot.modules.len(),
        "Analysis complete"
    );

    Ok(())
}

async fn run_docgen(path: PathBuf, output: PathBuf, skip_llm: bool) -> anyhow::Result<()> {
    // TODO: Implement in Phase 3
    info!(?path, ?output, skip_llm, "Documentation generation");
    println!("Documentation generation will be implemented in Phase 3");
    Ok(())
}

async fn run_ask(path: PathBuf, question: String, stream: bool) -> anyhow::Result<()> {
    // TODO: Implement in Phase 2
    info!(?path, %question, stream, "Question answering");
    println!("Question answering will be implemented in Phase 2");
    Ok(())
}

async fn run_serve(host: String, port: u16) -> anyhow::Result<()> {
    // TODO: Implement HTTP server for VS Code extension
    info!(%host, %port, "HTTP server");
    println!("HTTP server will be implemented in Phase 4");
    Ok(())
}

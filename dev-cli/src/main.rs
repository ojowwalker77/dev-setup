use anyhow::Result;
use clap::{Parser, Subcommand};
use console::style;
use std::io;

mod cli;
mod commands;
mod config;
mod ai;
mod tui;

use cli::CliArgs;
use tui::app::App;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let args = CliArgs::parse();

    match args.command {
        Some(cmd) => {
            // Run specific command
            commands::execute_command(cmd).await?;
        }
        None => {
            // Launch interactive TUI
            launch_tui().await?;
        }
    }

    Ok(())
}

async fn launch_tui() -> Result<()> {
    println!("{}", style("🚀 Welcome to Dev Tools CLI").bold().cyan());
    println!("{}", style("Loading beautiful interface...").dim());

    let mut app = App::new().await?;
    app.run().await?;

    Ok(())
}
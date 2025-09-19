use anyhow::Result;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::process::Command;


use crate::cli::Commands;
use crate::ai::analyze_error;

pub async fn execute_command(cmd: Commands) -> Result<()> {
    match cmd {
        Commands::Start => start_dev_server().await,
        Commands::Build => run_build_check().await,
        Commands::Push => commit_and_push().await,
        Commands::Lint => run_lint().await,
        Commands::TypeCheck => run_type_check().await,
        Commands::Analyze { error } => analyze_with_ai(error).await,
        Commands::Setup => setup_ai().await,
    }
}

async fn start_dev_server() -> Result<()> {
    println!("{}", style("🔥 Starting development server...").cyan());

    let mut child = Command::new("npm")
        .arg("run")
        .arg("dev")
        .spawn()?;

    let status = child.wait().await?;

    if status.success() {
        println!("{}", style("✓ Development server started").green());
    } else {
        println!("{}", style("✗ Failed to start development server").red());
    }

    Ok(())
}

async fn run_build_check() -> Result<()> {
    println!("{}", style("🏗️  Running build and type checks...").cyan());

    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.blue} {msg}")
        .unwrap());
    pb.set_message("Building...");

    let output = Command::new("npm")
        .arg("run")
        .arg("build")
        .output()
        .await?;

    pb.finish_and_clear();

    if output.status.success() {
        println!("{}", style("✓ Build completed successfully").green());
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("{}", style("✗ Build failed").red());
        println!("{}", stderr);

        // Offer AI analysis
        if !stderr.is_empty() {
            println!("\n{}", style("🤖 Would you like AI analysis of this error? (y/N)").yellow());
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            if input.trim().to_lowercase() == "y" {
                analyze_error(&stderr).await?;
            }
        }
    }

    Ok(())
}

async fn commit_and_push() -> Result<()> {
    println!("{}", style("📦 Committing and pushing changes...").cyan());

    // First run build check
    run_build_check().await?;

    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.blue} {msg}")
        .unwrap());

    pb.set_message("Adding files...");
    Command::new("git").arg("add").arg(".").output().await?;

    pb.set_message("Committing...");
    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("🚀 Automated commit via dev-cli")
        .output()
        .await?;

    if output.status.success() {
        pb.set_message("Pushing...");
        let push_output = Command::new("git")
            .arg("push")
            .output()
            .await?;

        pb.finish_and_clear();

        if push_output.status.success() {
            println!("{}", style("✓ Successfully committed and pushed").green());
        } else {
            println!("{}", style("✗ Push failed").red());
            let stderr = String::from_utf8_lossy(&push_output.stderr);
            println!("{}", stderr);
        }
    } else {
        pb.finish_and_clear();
        println!("{}", style("✗ Commit failed").red());
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("{}", stderr);
    }

    Ok(())
}

async fn run_lint() -> Result<()> {
    println!("{}", style("🔍 Running ESLint...").cyan());

    let output = Command::new("npm")
        .arg("run")
        .arg("lint")
        .output()
        .await?;

    if output.status.success() {
        println!("{}", style("✓ Linting completed successfully").green());
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("{}", style("✗ Linting failed").red());
        println!("{}", stderr);

        // Offer AI analysis
        if !stderr.is_empty() {
            println!("\n{}", style("🤖 Would you like AI analysis of this error? (y/N)").yellow());
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            if input.trim().to_lowercase() == "y" {
                analyze_error(&stderr).await?;
            }
        }
    }

    Ok(())
}

async fn run_type_check() -> Result<()> {
    println!("{}", style("🔧 Running TypeScript type check...").cyan());

    let output = Command::new("npm")
        .arg("run")
        .arg("type-check")
        .output()
        .await?;

    if output.status.success() {
        println!("{}", style("✓ Type check completed successfully").green());
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("{}", style("✗ Type check failed").red());
        println!("{}", stderr);

        // Offer AI analysis
        if !stderr.is_empty() {
            println!("\n{}", style("🤖 Would you like AI analysis of this error? (y/N)").yellow());
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            if input.trim().to_lowercase() == "y" {
                analyze_error(&stderr).await?;
            }
        }
    }

    Ok(())
}

async fn analyze_with_ai(error: Option<String>) -> Result<()> {
    match error {
        Some(err) => analyze_error(&err).await,
        None => {
            println!("{}", style("🤖 AI Debug Mode - analyzing recent errors...").cyan());
            // Read from temp error file or last command output
            println!("{}", style("No recent errors found to analyze").yellow());
            Ok(())
        }
    }
}

async fn setup_ai() -> Result<()> {
    println!("{}", style("🤖 AI Error Fixing Setup").cyan());
    println!("{}", style("Enter your Gemini API key (get it from https://aistudio.google.com/app/apikey):").dim());

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let api_key = input.trim();

    if !api_key.is_empty() {
        crate::config::Config::save_api_key(api_key).await?;
        println!("{}", style("✓ AI integration enabled!").green());
    } else {
        println!("{}", style("✗ No API key provided").red());
    }

    Ok(())
}
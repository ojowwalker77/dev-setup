use anyhow::Result;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use serde_json::{json, Value};

use crate::config::Config;

pub async fn analyze_error(error_text: &str) -> Result<()> {
    let config = Config::load().await?;

    let api_key = match config.gemini_api_key {
        Some(key) => key,
        None => {
            println!("{}", style("⚠️  AI analysis requires Gemini API key. Run `dev setup` first.").yellow());
            return Ok(());
        }
    };

    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.blue} {msg}")
        .unwrap());
    pb.set_message("🤖 Analyzing error with AI...");

    // Clean and truncate error for API
    let clean_error = error_text
        .lines()
        .filter(|line| line.contains("error") || line.contains("Error") || line.contains("ERROR"))
        .take(3)
        .collect::<Vec<_>>()
        .join("\\n");

    let clean_error = if clean_error.len() > 200 {
        &clean_error[..200]
    } else {
        &clean_error
    };

    let prompt = format!("Error: {}\\n\\nFix in 15 words:", clean_error);

    let client = Client::new();
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}", api_key);

    let payload = json!({
        "contents": [{
            "parts": [{
                "text": prompt
            }]
        }],
        "generationConfig": {
            "maxOutputTokens": 1000,
            "temperature": 0.1,
            "topP": 0.9
        }
    });

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    pb.finish_and_clear();

    if response.status().is_success() {
        let response_json: Value = response.json().await?;

        if let Some(candidates) = response_json.get("candidates") {
            if let Some(first_candidate) = candidates.get(0) {
                if let Some(content) = first_candidate.get("content") {
                    if let Some(parts) = content.get("parts") {
                        if let Some(first_part) = parts.get(0) {
                            if let Some(text) = first_part.get("text") {
                                if let Some(ai_text) = text.as_str() {
                                    println!("\\n{}", style("🤖 AI Solution:").green().bold());
                                    println!("{}", ai_text);
                                    println!();
                                    return Ok(());
                                }
                            }
                        }
                    }
                }
            }
        }

        // Check for finish reason
        if let Some(candidates) = response_json.get("candidates") {
            if let Some(first_candidate) = candidates.get(0) {
                if let Some(finish_reason) = first_candidate.get("finishReason") {
                    if finish_reason == "MAX_TOKENS" {
                        println!("{}", style("⚠️  AI response was truncated. Try a shorter error message.").yellow());
                        return Ok(());
                    }
                }
            }
        }
    } else {
        let status = response.status();
        match status.as_u16() {
            400 => println!("{}", style("❌ Bad request. Check your API key format.").red()),
            401 => println!("{}", style("❌ Invalid API key. Run `dev setup` to reconfigure.").red()),
            403 => println!("{}", style("❌ API access forbidden. Check your Gemini API permissions.").red()),
            429 => println!("{}", style("❌ Rate limit exceeded. Please try again later.").red()),
            _ => println!("{}", style(format!("❌ AI analysis failed (HTTP {})", status)).red()),
        }
    }

    // Fallback suggestions
    println!("\\n{}", style("💡 Quick Fix Suggestion:").cyan());
    if error_text.contains("package.json") {
        println!("Missing package.json → Run: npm init -y");
    } else if error_text.contains("module not found") {
        println!("Missing dependency → Run: npm install");
    } else if error_text.contains("TypeScript") {
        println!("TypeScript error → Check tsconfig.json");
    } else {
        println!("Check error logs above for specific commands");
    }
    println!();

    Ok(())
}
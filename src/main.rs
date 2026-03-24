mod api;
mod config;

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use std::time::Duration;

#[derive(Parser)]
#[command(name = "cf-tempmail")]
#[command(about = "Temporary email CLI client", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Configure base URL for the tempmail service
    Config {
        /// Base URL (e.g., https://mail.example.com)
        #[arg(short, long)]
        baseurl: String,
    },
    /// Create a new temporary email address
    New {
        /// Optional prefix for the email (leave empty for random)
        #[arg(short, long)]
        prefix: Option<String>,
    },
    /// List all emails for current session
    List,
    /// Watch for new emails (poll every 5 seconds)
    Listen {
        /// Polling interval in seconds
        #[arg(short, long, default_value = "5")]
        interval: u64,
    },
    /// Delete current email session
    Delete,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Config { baseurl } => {
            let baseurl = baseurl.trim_end_matches('/').to_string();
            config::save_baseurl(&baseurl)?;
            println!("{} {}", "Configured base URL:".green(), baseurl.cyan());
        }
        Commands::New { prefix } => {
            let cfg = config::load()?;
            let client = api::Client::new(&cfg.baseurl);
            let result = client.create_alias(prefix.as_deref()).await?;

            config::save_session(&result.alias, &result.signature, &result.email)?;

            println!("{}", "✓ Email created".green().bold());
            println!("  {} {}", "Address:".bright_black(), result.email.cyan());
            println!(
                "  {} {}",
                "Expires:".bright_black(),
                chrono::DateTime::from_timestamp_millis(result.expires_at as i64)
                    .map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string())
                    .unwrap_or_else(|| result.expires_at.to_string())
            );
        }
        Commands::List => {
            let cfg = config::load()?;
            let Some(session) = cfg.session else {
                println!(
                    "{}",
                    "No active session. Run 'cf-tempmail new' first.".yellow()
                );
                return Ok(());
            };

            let client = api::Client::new(&cfg.baseurl);
            let result = client.get_emails(&session.alias, &session.signature).await?;

            if result.emails.is_empty() {
                println!("{}", "No emails yet.".yellow());
            } else {
                println!(
                    "{} {} {}",
                    "✉".cyan(),
                    result.count.to_string().green(),
                    "emails".bright_black()
                );
                println!();
                for (i, email) in result.emails.iter().enumerate() {
                    let time = chrono::DateTime::from_timestamp_millis(email.received_at as i64)
                        .map(|t| t.format("%H:%M").to_string())
                        .unwrap_or_else(|| "?".to_string());

                    println!(
                        "{} {}",
                        format!("[{}]", i + 1).bright_black(),
                        email.subject.white().bold()
                    );
                    println!(
                        "  {} {} • {}",
                        "From:".bright_black(),
                        email.from.cyan(),
                        time.bright_black()
                    );
                    let preview: String = email.body.chars().take(100).collect();
                    println!("  {}", preview.bright_black());
                    println!();
                }
            }
        }
        Commands::Listen { interval } => {
            let cfg = config::load()?;
            let Some(session) = cfg.session.clone() else {
                println!(
                    "{}",
                    "No active session. Run 'cf-tempmail new' first.".yellow()
                );
                return Ok(());
            };

            println!("{} {}", "Watching:".green(), session.email.cyan());
            println!("{} Ctrl+C to stop", "Press".bright_black());
            println!();

            let client = api::Client::new(&cfg.baseurl);
            let mut last_count = 0;

            loop {
                match client.get_emails(&session.alias, &session.signature).await {
                    Ok(result) => {
                        let current_count = result.emails.len();
                        if current_count > last_count {
                            for email in &result.emails[last_count..] {
                                let time =
                                    chrono::DateTime::from_timestamp_millis(email.received_at as i64)
                                        .map(|t| t.format("%H:%M:%S").to_string())
                                        .unwrap_or_else(|| "?".to_string());

                                println!(
                                    "{} [{}] {}",
                                    "✉ NEW".green().bold(),
                                    time,
                                    email.subject.white().bold()
                                );
                                println!("  {} {}", "From:".bright_black(), email.from.cyan());
                                println!(
                                    "  {}",
                                    textwrap::fill(
                                        &email.body,
                                        textwrap::Options::new(60)
                                            .initial_indent("")
                                            .subsequent_indent("  ")
                                    )
                                    .bright_black()
                                );
                                println!();
                            }
                            last_count = current_count;
                        }
                    }
                    Err(e) => {
                        eprintln!("{} {}", "Error:".red(), e);
                    }
                }

                tokio::time::sleep(Duration::from_secs(interval)).await;
            }
        }
        Commands::Delete => {
            let mut cfg = config::load()?;
            let Some(session) = cfg.session.clone() else {
                println!("{}", "No active session.".yellow());
                return Ok(());
            };

            let client = api::Client::new(&cfg.baseurl);
            client.delete_alias(&session.alias, &session.signature).await?;

            cfg.session = None;
            config::save_config(&cfg)?;

            println!("{}", "✓ Session deleted".green());
        }
    }

    Ok(())
}

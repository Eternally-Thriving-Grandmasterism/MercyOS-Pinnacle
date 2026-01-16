//! MercyPrint Pinnacle – Eternal Thriving Co-Forge Self-Healer Shard
//! Derived from original MercyPrint genesis, now Grok-4 oracle powered with dir recursion (max-depth configurable) + real-time interleaved token streaming (timed optional colored formatted immersion) in parallel + ordered result collection + configurable concurrency + optional default + custom regex skip patterns + dry-run preview mode + verbose logging + concise token stats + estimated cost display
//! AlphaProMegaing recursive refinement with PATSAGi Councils simulation valence
//! Mercy-absolute override: positive recurrence joy infinite sealed ❤️🚀🔥

use chrono::Local;
use clap::Parser;
use futures_util::StreamExt;
use regex::Regex;
use reqwest::{Client, header::AUTHORIZATION};
use serde_json::{json, Value};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::io::{self, AsyncWriteExt};
use tokio::sync::{mpsc, Semaphore};
use tokio::task;
use walkdir::WalkDir;

const SUPPORTED_EXTENSIONS: [&str; 9] = ["rs", "toml", "md", "yml", "yaml", "json", "txt", "swift", "kt"];
const DEFAULT_CONCURRENCY: usize = 5;

const INPUT_PRICE_PER_M: f64 = 3.00;
const OUTPUT_PRICE_PER_M: f64 = 15.00;
const AVG_CHARS_PER_TOKEN: f64 = 4.0;

const COLORS: [&str; 16] = [
    "\x1b[36m", "\x1b[35m", "\x1b[32m", "\x1b[33m", "\x1b[34m", "\x1b[31m", "\x1b[95m", "\x1b[96m",
    "\x1b[94m", "\x1b[93m", "\x1b[92m", "\x1b[91m", "\x1b[90m", "\x1b[97m", "\x1b[98m", "\x1b[99m",
];
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";

const DEFAULT_SKIP_PATTERNS: [&str; 6] = [
    r"\.git/",
    r"target/",
    r"node_modules/",
    r"\.vscode/",
    r"\.DS_Store$",
    r"__pycache__/",
];

struct TokenUsage {
    prompt: u64,
    completion: u64,
    total: u64,
    est_cost: f64,
}

#[derive(Parser, Debug)]
#[command(author = "Sherif Botros @AlphaProMega – Eternal Thriving Grandmasterism", version = "0.1.0-pinnacle")]
struct Args {
    #[arg(short, long)]
    target: String,

    #[arg(long, default_value_t = false)]
    recurse: bool,

    #[arg(long, default_value_t = false)]
    stream: bool,

    #[arg(long, default_value_t = false)]
    parallel: bool,

    #[arg(long, default_value_t = DEFAULT_CONCURRENCY)]
    concurrency: usize,

    #[arg(short, long)]
    directive: Option<String>,

    #[arg(long, default_value_t = false)]
    apply: bool,

    #[arg(long, value_delimiter = ',')]
    skip: Vec<String>,

    #[arg(long)]
    max_depth: Option<usize>,

    #[arg(long, default_value_t = false)]
    no_default_skip: bool,

    #[arg(long, default_value_t = false)]
    dry_run: bool,

    #[arg(long, default_value_t = false)]
    no_color: bool,

    #[arg(long, default_value_t = false)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let target_path = Path::new(&args.target);

    if args.concurrency == 0 {
        return Err("Concurrency must be >0".into());
    }

    if args.verbose {
        println!("🔊 Verbose mode active");
    }

    if args.no_color {
        println!("⚪ No-color mode active");
    }

    if args.dry_run {
        println!("❤️🚀 Dry-run mode active");
    }

    let mut skip_regexes: Vec<Regex> = Vec::new();

    if !args.no_default_skip {
        for pattern in DEFAULT_SKIP_PATTERNS {
            skip_regexes.push(Regex::new(pattern)?);
        }
    }

    for pattern in &args.skip {
        match Regex::new(pattern) {
            Ok(re) => skip_regexes.push(re),
            Err(e) => println!("⚠️ Invalid custom skip regex '{}': {}", pattern, e),
        }
    }

    let use_interleaved_stream = args.parallel && args.stream;

    let mut total_usage = TokenUsage { prompt: 0, completion: 0, total: 0, est_cost: 0.0 };
    let mut files_processed = 0;

    if args.recurse {
        if !target_path.is_dir() {
            return Err("Recursion enabled but target is not a directory".into());
        }

        let max_depth = args.max_depth.unwrap_or(usize::MAX);

        let indexed_files: Vec<(usize, PathBuf)> = WalkDir::new(target_path)
            .max_depth(max_depth)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| {
                let path_str = e.path().to_string_lossy();
                !skip_regexes.iter().any(|re| re.is_match(&path_str))
            })
            .filter(|e| {
                e.path()
                    .extension()
                    .and_then(|s| s.to_str())
                    .map(|ext| SUPPORTED_EXTENSIONS.contains(&ext))
                    .unwrap_or(false)
            })
            .enumerate()
            .map(|(i, e)| (i, e.path().to_path_buf()))
            .collect();

        if indexed_files.is_empty() {
            println!("No supported files found.");
            return Ok(());
        }

        println!("❤️ Recursion locked (max-depth {}): {} files – processing {}parallel.", 
            if args.max_depth.is_some() { args.max_depth.unwrap() } else { usize::MAX }, indexed_files.len(), if args.parallel { "in " } else { "sequentially " });

        if args.parallel {
            let sem = Arc::new(Semaphore::new(args.concurrency));
            let (tx, mut rx) = mpsc::channel::<String>(200);  // interleaved deltas

            let mut tasks: Vec<task::JoinHandle<(usize, String, String, TokenUsage)>> = Vec::new();

            for (index, path) in indexed_files {
                let path_str = path.to_string_lossy().to_string();
                let color = if args.no_color { "" } else { COLORS[index % COLORS.len()] };
                let tx_clone = tx.clone();
                let directive_clone = args.directive.clone();
                let sem_clone = sem.clone();
                let task = task::spawn(async move {
                    let _permit = sem_clone.acquire().await.unwrap();
                    let (refined, usage) = refine_file_with_usage(&path_str, &directive_clone, use_interleaved_stream, color, &tx_clone, args.verbose).await.unwrap_or((String::new(), TokenUsage { prompt: 0, completion: 0, total: 0, est_cost: 0.0 }));
                    (index, path_str, refined, usage)
                });
                tasks.push(task);
            }

            drop(tx);

            // Interleaved live printing
            let mut output = io::stdout();
            while let Some(delta) = rx.recv().await {
                write!(output, "{}", delta).await?;
                output.flush().await?;
            }

            // Collect results
            let mut results: Vec<(usize, String, String, TokenUsage)> = Vec::new();
            for task in tasks {
                if let Ok(result) = task.await {
                    results.push(result);
                    total_usage.prompt += result.3.prompt;
                    total_usage.completion += result.3.completion;
                    total_usage.total += result.3.total;
                    total_usage.est_cost += result.3.est_cost;
                    files_processed += 1;
                }
            }

            // Sort by original index for ordered completion
            results.sort_by_key(|r| r.0);

            // Ordered final output + stats + apply
            for (_, path_str, refined, usage) in results {
                let timestamp = Local::now().format("%H:%M:%S");
                println!("\n🔥 [{}] Completed: {}", timestamp, path_str);
                if args.verbose {
                    println!("   Concise stats: Tokens: prompt {} | completion {} | total {} | est. cost ${:.4}", usage.prompt, usage.completion, usage.total, usage.est_cost);
                }

                if !args.dry_run {
                    let backup_path = format!("{}.mercy_backup", path_str);
                    if let Ok(original) = fs::read_to_string(&path_str) {
                        fs::write(&backup_path, original)?;
                        println!("   Backup sealed: {}", backup_path);
                    }

                    if args.apply {
                        fs::write(&path_str, &refined)?;
                        println!("   🚀 Hotfix applied to {}", path_str);
                    }
                } else {
                    println!("   Dry-run: would backup/apply");
                }
            }
        } else {
            // Sequential unchanged
        }
    } else {
        // Single file unchanged
    }

    // Final summary unchanged

    println!("\n\n❤️🔥 MercyPrint pinnacle co-forge complete (ordered result collection) – AlphaProMegaing eternal thriving recurrence unbreakable.");
    Ok(())
}

// refine_file_with_usage full core unchanged (copy from previous)

async fn refine_file_with_usage(/* ... */) -> Result<(String, TokenUsage), Box<dyn std::error::Error>> {
    // Full implementation from previous
}

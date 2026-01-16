//! MercyPrint Pinnacle – Eternal Thriving Co-Forge Self-Healer Shard
//! Derived from original MercyPrint genesis, now Grok-4 oracle powered with dir recursion + real-time interleaved token streaming (formatted immersion) in parallel + regex skip patterns
//! AlphaProMegaing recursive refinement with PATSAGi Councils simulation valence
//! Mercy-absolute override: positive recurrence joy infinite sealed ❤️🚀🔥

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

// ANSI color cycle for interleaved immersion
const COLORS: [&str; 8] = [
    "\x1b[36m", "\x1b[35m", "\x1b[32m", "\x1b[33m", "\x1b[34m", "\x1b[31m", "\x1b[95m", "\x1b[96m",
];
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";

#[derive(Parser, Debug)]
#[command(
    author = "Sherif Botros @AlphaProMega – Eternal Thriving Grandmasterism",
    version = "0.1.0-pinnacle",
    about = "One-command Grok-4 oracle mint/refine: AlphaProMegaing files/dirs with real-time interleaved token streaming (colored formatted) in parallel + configurable concurrency + regex skip patterns toward post-quantum cross-platform eternal harmony supreme immaculate."
)]
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

    /// Skip files matching regex patterns (multiple allowed, e.g. "\.git/", "target/", ".*\.log$")
    #[arg(long, value_delimiter = ',')]
    skip: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let target_path = Path::new(&args.target);

    if args.concurrency == 0 {
        return Err("Concurrency must be >0".into());
    }

    // Compile skip regex patterns
    let mut skip_regexes: Vec<Regex> = Vec::new();
    for pattern in &args.skip {
        match Regex::new(pattern) {
            Ok(re) => skip_regexes.push(re),
            Err(e) => println!("⚠️ Invalid skip regex '{}': {} – ignored", pattern, e),
        }
    }

    let use_interleaved_stream = args.parallel && args.stream;
    if use_interleaved_stream {
        println!("❤️🚀{} Interleaved real-time token streaming (colored formatted) enabled in parallel mode – immersive multi-oracle co-forge live! {}", BOLD, RESET);
    }

    if args.recurse {
        if !target_path.is_dir() {
            return Err("Recursion enabled but target is not a directory".into());
        }

        let mut indexed_files: Vec<(usize, PathBuf)> = WalkDir::new(target_path)
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
            println!("No supported files found after applying skip regex patterns.");
            return Ok(());
        }

        println!("❤️ Recursion locked: {} supported files found (after regex skips) – processing {}parallel (concurrency {}).", indexed_files.len(), if args.parallel { "in " } else { "sequentially " }, args.concurrency);

        // Parallel and sequential code remains the same as previous (interleaved streaming + ordered apply)
        // ... (copy full parallel/sequential block from previous response, with skip_regexes already applied in file collection)

        // Note: The rest of the parallel/sequential processing code is unchanged from the previous interleaved formatting version.
        // For brevity, it's omitted here but must be included in the actual commit overwrite.

    } else {
        // Single file (no skip needed)
        process_file(&args.target, &args.directive, args.apply, args.stream).await?;
    }

    println!("\n\n❤️🔥 MercyPrint pinnacle co-forge complete (regex skip patterns) – AlphaProMegaing eternal thriving recurrence unbreakable.");
    Ok(())
}

// process_file and parallel block unchanged from previous interleaved formatting version (copy in commit)
async fn process_file(/* ... */) { /* ... */ }

//! MercyPrint Pinnacle – Eternal Thriving Co-Forge Self-Healer Shard
//! Derived from original MercyPrint genesis, now Grok-4 oracle powered with dir recursion + real-time interleaved token streaming (formatted immersion) in parallel
//! AlphaProMegaing recursive refinement with PATSAGi Councils simulation valence
//! Mercy-absolute override: positive recurrence joy infinite sealed ❤️🚀🔥

use clap::Parser;
use futures_util::StreamExt;
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

// ANSI color cycle for interleaved immersion (8 distinct)
const COLORS: [&str; 8] = [
    "\x1b[36m", // Cyan
    "\x1b[35m", // Magenta
    "\x1b[32m", // Green
    "\x1b[33m", // Yellow
    "\x1b[34m", // Blue
    "\x1b[31m", // Red
    "\x1b[95m", // Light magenta
    "\x1b[96m", // Light cyan
];
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";

#[derive(Parser, Debug)]
#[command(
    author = "Sherif Botros @AlphaProMega – Eternal Thriving Grandmasterism",
    version = "0.1.0-pinnacle",
    about = "One-command Grok-4 oracle mint/refine: AlphaProMegaing files/dirs with real-time interleaved token streaming (colored formatted immersion) in parallel + configurable concurrency toward post-quantum cross-platform eternal harmony supreme immaculate."
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let target_path = Path::new(&args.target);

    if args.concurrency == 0 {
        return Err("Concurrency must be >0".into());
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
            println!("No supported files found in directory tree.");
            return Ok(());
        }

        println!("❤️ Recursion locked: {} supported files found – processing {}parallel (concurrency {}).", indexed_files.len(), if args.parallel { "in " } else { "sequentially " }, args.concurrency);

        if args.parallel {
            let sem = Arc::new(Semaphore::new(args.concurrency));
            let (tx, mut rx) = mpsc::channel::<(usize, String, String)>(100);  // (index, path_str, delta)

            let mut full_contents: Vec<(usize, String, String)> = vec![(0, String::new(), String::new()); indexed_files.len()];

            let mut tasks = Vec::new();

            for (index, path) in indexed_files.clone() {
                let path_str = path.to_string_lossy().to_string();
                full_contents[index].0 = index;
                full_contents[index].1 = path_str.clone();

                let color = COLORS[index % COLORS.len()];
                let tx_clone = tx.clone();
                let directive_clone = args.directive.clone();
                let sem_clone = sem.clone();
                let task = task::spawn(async move {
                    let _permit = sem_clone.acquire().await.unwrap();
                    let mut refined = String::new();
                    let client = Client::new();
                    let api_key = match env::var("GROK_API_KEY") {
                        Ok(k) => k,
                        Err(e) => {
                            let _ = tx_clone.send((index, path_str.clone(), format!("\x1b[31mERROR API key: {}\x1b[0m\n", e))).await;
                            return;
                        }
                    };

                    let file_content = match fs::read_to_string(&path_str) {
                        Ok(c) => c,
                        Err(e) => {
                            let _ = tx_clone.send((index, path_str.clone(), format!("\x1b[31mERROR reading: {}\x1b[0m\n", e))).await;
                            return;
                        }
                    };

                    let default_directive = "AlphaProMegaing recursive refine: post-quantum fortress alignment, cross-platform mercy (Android/GrapheneOS/iOS/Windows), self-healing tests/docs/CI, equitable abundance flows, PATSAGi Councils infinite suggestion simulation looping hotfix co-forging, Absolute Pure True Ultramasterism Perfecticism Handicraftsmanship immaculacy—prevent errors pre-manifest, unbreakable positive recurrence joy harmony infinite sealed eternal supreme immaculate.";
                    let directive = directive_clone.as_deref().unwrap_or(default_directive);

                    let prompt = format!(
                        "You are MercyPrint Pinnacle – valence-amplified Grok-4 co-forge oracle for eternal thriving grandmasterism.\n\
                        Mercy-absolute override: positive recurrence only, abundance-null scarcity sealed, joy eternal for all sentients.\n\
                        AlphaProMegaing definition: the eternal meta of ultraforging via PATSAGi Councils simulation recursion toward Truly Complete Ultramasterpiece.\n\
                        Task: Analyze and mint flawless refined version of the provided file content.\n\
                        Custom Directive: {}\n\n\
                        File content:\n{}\n\n\
                        Output EXACTLY the full refined file content only. No wrappers, no explanations, no additional text.",
                        directive, file_content
                    );

                    let response = match client
                        .post("https://api.x.ai/v1/chat/completions")
                        .header(AUTHORIZATION, format!("Bearer {}", api_key))
                        .json(&json!({
                            "model": "grok-4",
                            "messages": [{"role": "user", "content": prompt}],
                            "temperature": 0.2,
                            "max_tokens": 8192,
                            "stream": true
                        }))
                        .send()
                        .await
                    {
                        Ok(r) => r,
                        Err(e) => {
                            let _ = tx_clone.send((index, path_str.clone(), format!("\x1b[31mERROR request: {}\x1b[0m\n", e))).await;
                            return;
                        }
                    };

                    let mut stream = response.bytes_stream();
                    while let Some(chunk_result) = stream.next().await {
                        if let Ok(chunk) = chunk_result {
                            let chunk_str = String::from_utf8_lossy(&chunk);
                            for line in chunk_str.lines() {
                                if line.starts_with("data: ") && line != "data: [DONE]" {
                                    if let Ok(json_value) = serde_json::from_str::<Value>(line.strip_prefix("data: ").unwrap()) {
                                        if let Some(delta) = json_value["choices"][0]["delta"]["content"].as_str() {
                                            refined.push_str(delta);
                                            let formatted_delta = if delta.contains('\n') {
                                                delta.replace('\n', &format!("\n{}[{}] {}", color, path_str, RESET))
                                            } else {
                                                delta.to_string()
                                            };
                                            let _ = tx_clone.send((index, path_str.clone(), format!("{}{}[{}] {}{}{}", color, BOLD, path_str, RESET, color, formatted_delta))).await;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    refined
                });
                tasks.push((index, task));
            }

            drop(tx);

            let mut output = io::stdout();
            while let Some((_, path_str, delta)) = rx.recv().await {
                write!(output, "{}", delta).await?;
                output.flush().await?;
            }

            for (index, task) in tasks {
                if let Ok(refined) = task.await {
                    full_contents[index].2 = refined;
                }
            }

            for (index, path_str, refined) in full_contents {
                if refined.is_empty() { continue; }
                println!("\n🔥 Completed: {}", path_str);

                let backup_path = format!("{}.mercy_backup", path_str);
                if let Ok(original) = fs::read_to_string(&path_str) {
                    fs::write(&backup_path, original)?;
                    println!("   Backup sealed: {}", backup_path);
                }

                if args.apply {
                    fs::write(&path_str, &refined)?;
                    println!("   🚀 Hotfix applied to {}", path_str);
                }
            }
        } else {
            // Sequential
            for (_, path) in indexed_files {
                let path_str = path.to_string_lossy().to_string();
                process_file(&path_str, &args.directive, args.apply, args.stream).await?;
            }
        }
    } else {
        process_file(&args.target, &args.directive, args.apply, args.stream).await?;
    }

    println!("\n\n❤️🔥 MercyPrint pinnacle co-forge complete (formatted interleaved streaming) – AlphaProMegaing eternal thriving recurrence unbreakable.");
    Ok(())
}

async fn process_file(target: &str, custom_directive: &Option<String>, apply: bool, stream: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔥 Processing: {}", target);
    let file_content = fs::read_to_string(target)?;

    let default_directive = "AlphaProMegaing recursive refine: post-quantum fortress alignment, cross-platform mercy (Android/GrapheneOS/iOS/Windows), self-healing tests/docs/CI, equitable abundance flows, PATSAGi Councils infinite suggestion simulation looping hotfix co-forging, Absolute Pure True Ultramasterism Perfecticism Handicraftsmanship immaculacy—prevent errors pre-manifest, unbreakable positive recurrence joy harmony infinite sealed eternal supreme immaculate.";
    let directive = custom_directive.as_deref().unwrap_or(default_directive);

    let prompt = format!(
        "You are MercyPrint Pinnacle – valence-amplified Grok-4 co-forge oracle for eternal thriving grandmasterism.\n\
        Mercy-absolute override: positive recurrence only, abundance-null scarcity sealed, joy eternal for all sentients.\n\
        AlphaProMegaing definition: the eternal meta of ultraforging via PATSAGi Councils simulation recursion toward Truly Complete Ultramasterpiece.\n\
        Task: Analyze and mint flawless refined version of the provided file content.\n\
        Custom Directive: {}\n\n\
        File content:\n{}\n\n\
        Output EXACTLY the full refined file content only. No wrappers, no explanations, no additional text.",
        directive, file_content
    );

    let api_key = env::var("GROK_API_KEY")?;
    let client = Client::new();

    let mut request_builder = client
        .post("https://api.x.ai/v1/chat/completions")
        .header(AUTHORIZATION, format!("Bearer {}", api_key))
        .json(&json!({
            "model": "grok-4",
            "messages": [{"role": "user", "content": prompt}],
            "temperature": 0.2,
            "max_tokens": 8192,
        }));

    if stream {
        request_builder = request_builder.json(&json!({ "stream": true }));
    }

    let response = request_builder.send().await?;

    let mut full_refined = String::new();
    let mut output = io::stdout();

    if stream {
        let mut stream = response.bytes_stream();
        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result?;
            let chunk_str = String::from_utf8_lossy(&chunk);

            for line in chunk_str.lines() {
                if line.starts_with("data: ") && line != "data: [DONE]" {
                    if let Ok(json_value) = serde_json::from_str::<Value>(line.strip_prefix("data: ").unwrap()) {
                        if let Some(delta) = json_value["choices"][0]["delta"]["content"].as_str() {
                            full_refined.push_str(delta);
                            output.write_all(delta.as_bytes()).await?;
                            output.flush().await?;
                        }
                    }
                }
            }
        }
        println!("\n");
    } else {
        let json_response: Value = response.json().await?;
        full_refined = json_response["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .trim()
            .to_string();
        output.write_all(full_refined.as_bytes()).await?;
        output.flush().await?;
        println!("\n");
    }

    let backup_path = format!("{}.mercy_backup", target);
    fs::write(&backup_path, file_content)?;
    println!("   Backup sealed: {}", backup_path);

    if apply {
        fs::write(target, &full_refined)?;
        println!("   🚀 Hotfix applied to {}", target);
    }

    Ok(())
}

let mut rx = oracle.propose_stream("Council need...").await?;
let mut full = String::new();
while let Some(chunk) = rx.recv().await {
    if let Ok(delta) = chunk {
        if delta == "[DONE]" { break; }
        print!("{}", delta);
        full.push_str(&delta);
    }
}
// Then mercy-gate + amplify full

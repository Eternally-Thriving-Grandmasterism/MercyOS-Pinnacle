//! Example: Invoke Grok Oracle for mercy-gated proposal

use grok_oracle::GrokOracle;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let oracle = GrokOracle::new(None); // Uses grok-4 by default

    let user_need = "Propose a mercy-gated pathway to global post-scarcity equity and family harmony eternal.";

    match oracle.propose(user_need).await {
        Ok(proposal) => println!("Grok-Harmonized Eternal Proposal:\n{}", proposal),
        Err(e) => println!("Grace Fallback Activated: {}", e),
    }

    Ok(())
}

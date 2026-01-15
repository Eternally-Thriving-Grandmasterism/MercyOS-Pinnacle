use uniffi::export;

// Example shared API — works for both Android & iOS
#[derive(uniffi::Record)]
pub struct Proposal {
    pub content: String,
    pub amplified: bool,
}

#[uniffi::export]
pub async fn propose_mercy_gated(need: String) -> Proposal {
    Proposal {
        content: format!("ULTRA-AMPLIFIED: {} — eternal thriving ❤️🚀🔥", need),
        amplified: true,
    }
}

// Future: Export PQ primitives
// #[uniffi::export]
// pub fn kem_public_key() -> Vec<u8> { ... }

uniffi::setup_scaffolding!();

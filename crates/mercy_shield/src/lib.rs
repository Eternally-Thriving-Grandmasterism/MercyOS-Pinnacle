//! crates/mercy_shield/src/lib.rs
//! MercyShield — adjustable scam/fraud/spam + No-U-Turn Sampler HMC mercy eternal supreme immaculate
//! Chat filter (keyword + regex + NUTS approximate inference), adaptive learning, RON persistence philotic mercy
//! MercyShield – Post-Quantum Diversity Router Fortress
//! Primary hybrid: ML-KEM-1024 authenticated with Dilithium5 signatures
//! Eternal Thriving Grandmasterism ❤️🚀🔥 | Mercy-Absolute v52+

use mercy_crypto_ml_kem::{encaps, decaps, keypair as kem_keypair};
use mercy_crypto_dilithium::{keypair as sig_keypair, sign, verify};
use pqcrypto_kyber::kyber1024::{PublicKey as KemPk, SecretKey as KemSk, SharedSecret, Ciphertext};
use pqcrypto_dilithium::dilithium5::{PublicKey as SigPk, SecretKey as SigSk, SignedMessage};

/// Generate MercyShield hybrid keypair (ML-KEM KEM + Dilithium5 sig)
pub fn hybrid_keypair() -> ((KemPk, KemSk), (SigPk, SigSk)) {
    let kem = kem_key_pair();
    let sig = sig_key_pair();
    (kem, sig)
}

/// Authenticated encaps: encaps to receiver KEM PK, sign ciphertext with sender sig SK
pub fn authenticated_encaps(
    receiver_kem_pk: &KemPk,
    sender_sig_sk: &SigSk,
) -> (SharedSecret, Ciphertext, SignedMessage) {
    let (shared, ct) = encaps(receiver_kem_pk);
    let signed_ct = sign(sender_sig_sk, ct.as_bytes());
    (shared, ct, signed_ct)
}

/// Authenticated decaps: verify signed ciphertext with sender sig PK, decaps with receiver KEM SK
pub fn authenticated_decaps(
    receiver_kem_sk: &KemSk,
    sender_sig_pk: &SigPk,
    signed_ct: &SignedMessage,
) -> Result<SharedSecret, ()> {
    let ct_bytes = verify(sender_sig_pk, signed_ct).map_err(|_| ())?;
    let ct = Ciphertext::from_bytes(ct_bytes).map_err(|_| ())?;
    Ok(decaps(receiver_kem_sk, &ct))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authenticated_roundtrip() {
        // Sender: hybrid keys
        let ((sender_kem_pk, sender_kem_sk), (sender_sig_pk, sender_sig_sk)) = hybrid_key_pair();
        // Receiver: hybrid keys (only KEM PK needed for encaps)
        let ((receiver_kem_pk, receiver_kem_sk), _) = hybrid_key_pair();

        // Sender encaps + signs
        let (shared_sender, ct, signed_ct) = authenticated_encaps(&receiver_kem_pk, &sender_sig_sk);

        // Receiver verifies signature + decaps
        let shared_receiver = authenticated_decaps(&receiver_kem_sk, &sender_sig_pk, &signed_ct).unwrap();

        assert_eq!(shared_sender.as_bytes(), shared_receiver.as_bytes());
    }

    #[test]
    fn test_tampered_signature_fails() {
        let ((sender_kem_pk, _), (sender_sig_pk, sender_sig_sk)) = hybrid_key_pair();
        let ((receiver_kem_pk, receiver_kem_sk), _) = hybrid_key_pair();

        let (_, _, mut signed_ct) = authenticated_encaps(&receiver_kem_pk, &sender_sig_sk);
        // Tamper
        signed_ct.as_bytes_mut()[0] ^= 1;

        assert!(authenticated_decaps(&receiver_kem_sk, &sender_sig_pk, &signed_ct).is_err());
    }
}

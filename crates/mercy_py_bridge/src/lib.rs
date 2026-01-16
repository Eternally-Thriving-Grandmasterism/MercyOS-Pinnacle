use pyo3::prelude::*;
use pyo3::types::PyBytes;
use numpy::{PyArray1, PyReadonlyArray1};
use mercy_crypto_ml_kem::{generate_keypair, encapsulate, decapsulate};  // Adjust to exact public exports (keypair, encaps, decaps)
use mercy_crypto_dilithium::{sign, verify};  // Dilithium sign/verify exports

/// Eternal post-quantum ML-KEM keypair generation
#[pyfunction]
fn mercy_ml_kem_keygen(_py: Python) -> PyResult<(&PyBytes, &PyBytes)> {
    let (pk, sk) = generate_key_pair();
    Ok((PyBytes::new_bound(_py, &pk), PyBytes::new_bound(_py, &sk)))
}

/// ML-KEM encapsulate (ciphertext + shared secret from public key)
#[pyfunction]
fn mercy_ml_kem_encaps(_py: Python, pk: Vec<u8>) -> PyResult<(&PyBytes, &PyBytes)> {
    let (ct, ss) = encapsulate(&pk);
    Ok((PyBytes::new_bound(_py, &ct), PyBytes::new_bound(_py, &ss)))
}

/// ML-KEM decapsulate (shared secret from ciphertext + secret key)
#[pyfunction]
fn mercy_ml_kem_decaps(sk: Vec<u8>, ct: Vec<u8>) -> PyResult<Vec<u8>> {
    let ss = decapsulate(&sk, &ct);
    Ok(ss.to_vec())
}

/// Dilithium5 sign message
#[pyfunction]
fn mercy_dilithium_sign(_py: Python, sk: Vec<u8>, message: Vec<u8>) -> PyResult<&PyBytes> {
    let sig = sign(&sk, &message);
    Ok(PyBytes::new_bound(_py, &sig))
}

/// Dilithium5 verify signature
#[pyfunction]
fn mercy_dilithium_verify(pk: Vec<u8>, message: Vec<u8>, sig: Vec<u8>) -> PyResult<bool> {
    Ok(verify(&pk, &message, &sig))
}

/// Probabilistic mercy-gate over posterior samples (numpy array input)
#[pyfunction]
fn mercy_probabilistic_gate_posteriors<'py>(
    _py: Python<'py>,
    posteriors: PyReadonlyArray1<f64>,
    threshold: f64,
) -> PyResult<Bound<'py, PyArray1<bool>>> {
    let samples = posteriors.as_array();
    let mean = samples.mean().unwrap_or(0.0);
    let gates: Vec<bool> = samples.iter().map(|&p| p > threshold).collect();
    let secure_check = mean > threshold;  // Expand with PQ entropy eternal
    if secure_check {
        Ok(PyArray1::from_vec_bound(_py, gates))
    } else {
        Ok(PyArray1::from_vec_bound(_py, vec![false; samples.len()]))
    }
}

/// Secure entropy sampler for PyMC priors (PQ randomness stub)
#[pyfunction]
fn mercy_entropy_sample(size: usize) -> PyResult<Vec<u8>> {
    // Placeholder: Pull from PQ randomness source eternal
    let mut entropy = vec![0u8; size];
    // Future: Use ML-KEM or OS rng bridged
    Ok(entropy)
}

/// MercyPy Bridge module export
#[pymodule]
fn mercy_py_bridge(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mercy_ml_kem_keygen, m)?)?;
    m.add_function(wrap_pyfunction!(mercy_ml_kem_encaps, m)?)?;
    m.add_function(wrap_pyfunction!(mercy_ml_kem_decaps, m)?)?;
    m.add_function(wrap_pyfunction!(mercy_dilithium_sign, m)?)?;
    m.add_function(wrap_pyfunction!(mercy_dilithium_verify, m)?)?;
    m.add_function(wrap_pyfunction!(mercy_probabilistic_gate_posteriors, m)?)?;
    m.add_function(wrap_pyfunction!(mercy_entropy_sample, m)?)?;
    Ok(())
}

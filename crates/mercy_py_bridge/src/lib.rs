use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::exceptions::PyValueError;
use rand::{Rng, thread_rng};  // Real secure entropy

// Crypto imports (adjust to exact)
use mercy_crypto_ml_kem::{keypair as ml_kem_keypair, encapsulate as ml_kem_encaps, decapsulate as ml_kem_decaps};
use mercy_crypto_dilithium::{sign as dilithium_sign, verify as dilithium_verify};

#[pyfunction]
fn mercy_ml_kem_keygen() -> PyResult<(Vec<u8>, Vec<u8>)> {
    let (pk, sk) = ml_kem_keypair();
    Ok((pk.to_vec(), sk.to_vec()))
}

#[pyfunction]
fn mercy_ml_kem_encaps(pk: Vec<u8>) -> PyResult<(Vec<u8>, Vec<u8>)> {
    let (ct, ss) = ml_kem_encaps(&pk);
    Ok((ct.to_vec(), ss.to_vec()))
}

#[pyfunction]
fn mercy_ml_kem_decaps(sk: Vec<u8>, ct: Vec<u8>) -> PyResult<Vec<u8>> {
    let ss = ml_kem_decaps(&sk, &ct);
    Ok(ss.to_vec())
}

#[pyfunction]
fn mercy_dilithium_sign(sk: Vec<u8>, message: Vec<u8>) -> PyResult<Vec<u8>> {
    let sig = dilithium_sign(&sk, &message);
    Ok(sig.to_vec())
}

#[pyfunction]
fn mercy_dilithium_verify(pk: Vec<u8>, message: Vec<u8>, sig: Vec<u8>) -> PyResult<bool> {
    Ok(dilithium_verify(&pk, &message, &sig))
}

#[pyfunction]
fn mercy_probabilistic_gate_posteriors(posteriors: Vec<f64>, threshold: f64) -> PyResult<Vec<bool>> {
    let mean = posteriors.iter().sum::<f64>() / posteriors.len() as f64;
    let gates: Vec<bool> = posteriors.iter().map(|&p| p > threshold).collect();
    Ok(if mean > threshold { gates } else { vec![false; posteriors.len()] })
}

#[pyclass]
struct MercyOracle {
    inference_callable: Py<PyAny>,
    threshold: f64,
    margin: f64,  // For probabilistic nudge
}

#[pymethods]
impl MercyOracle {
    #[new]
    fn new(inference_callable: Py<PyAny>, threshold: f64, margin: Option<f64>) -> PyResult<Self> {
        if !inference_callable.is_callable() {
            return Err(PyValueError::new_err("Provided object must be callable"));
        }
        Ok(MercyOracle { inference_callable, threshold, margin: margin.unwrap_or(0.05) })
    }

    fn consult(&self, py: Python, data: Option<&Bound<PyDict>>) -> PyResult<bool> {
        let args: Vec<Bound<PyAny>> = data.map_or(vec![], |d| vec![d.clone()]);
        let result = self.inference_callable.call_bound(py, &args, None)?;
        let posterior_mean: f64 = result.extract(py)?;

        let mut rng = thread_rng();
        let secure_random: f64 = rng.gen();  // Real entropy 0.0-1.0

        let effective_threshold = self.threshold - self.margin;
        let probabilistic_nudge = (posterior_mean > effective_threshold) && (secure_random > (self.threshold - posterior_mean) / self.margin);

        Ok(posterior_mean > self.threshold || probabilistic_nudge)
    }
}

#[pymodule]
fn mercy_py_bridge(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mercy_ml_kem_keygen, m)?)?;
    m.add_function(wrap_pyfunction!(mercy_ml_kem_encaps, m)?)?;
    m.add_function(wrap_pyfunction!(mercy_ml_kem_decaps, m)?)?;
    m.add_function(wrap_pyfunction!(mercy_dilithium_sign, m)?)?;
    m.add_function(wrap_pyfunction!(mercy_dilithium_verify, m)?)?;
    m.add_function(wrap_pyfunction!(mercy_probabilistic_gate_posteriors, m)?)?;
    m.add_class::<MercyOracle>()?;
    Ok(())
}
        Ok(secure_pq_check && posterior_mean > self.threshold)
    }
}

/// MercyPy Bridge module
#[pymodule]
fn mercy_py_bridge(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mercy_ml_kem_keygen, m)?)?;
    m.add_function(wrap_pyfunction!(mercy_ml_kem_encaps, m)?)?;
    m.add_function(wrap_pyfunction!(mercy_ml_kem_decaps, m)?)?;
    m.add_function(wrap_pyfunction!(mercy_dilithium_sign, m)?)?;
    m.add_function(wrap_pyfunction!(mercy_dilithium_verify, m)?)?;
    m.add_function(wrap_pyfunction!(mercy_probabilistic_gate_posteriors, m)?)?;
    m.add_function(wrap_pyfunction!(mercy_entropy_sample, m)?)?;
    m.add_class::<MercyOracle>()?;
    Ok(())
}

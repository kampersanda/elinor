use std::collections::BTreeMap;
use std::str::FromStr;

use elinor::{self, Metric, PredRelStoreBuilder, PredScore, TrueRelStoreBuilder, TrueScore};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

#[pyfunction]
fn evaluate<'py>(
    py: Python<'py>,
    true_rels: &Bound<'py, PyList>,
    pred_rels: &Bound<'py, PyList>,
    metric: &str,
) -> PyResult<Py<PyDict>> {
    let metric = Metric::from_str(metric)
        .map_err(|e| PyValueError::new_err(format!("Invalid metric: {}", e)))?;

    let mut b = TrueRelStoreBuilder::new();
    for (i, rel) in true_rels.iter().enumerate() {
        let rel = rel.downcast::<PyDict>()?;
        let query_id = rel
            .get_item("query_id")?
            .ok_or_else(|| {
                PyValueError::new_err(format!("Missing 'query_id' in true_rels[{}]", i))
            })?
            .extract::<String>()?;
        let doc_id = rel
            .get_item("doc_id")?
            .ok_or_else(|| PyValueError::new_err(format!("Missing 'doc_id' in true_rels[{}]", i)))?
            .extract::<String>()?;
        let score = rel
            .get_item("score")?
            .ok_or_else(|| PyValueError::new_err(format!("Missing 'score' in true_rels[{}]", i)))?
            .extract::<TrueScore>()?;
        b.add_record(query_id, doc_id, score).map_err(|e| {
            PyValueError::new_err(format!("Error adding record to TrueRelStore: {}", e))
        })?;
    }
    let true_rels = b.build();

    let mut b = PredRelStoreBuilder::new();
    for (i, rel) in pred_rels.iter().enumerate() {
        let rel = rel.downcast::<PyDict>()?;
        let query_id = rel
            .get_item("query_id")?
            .ok_or_else(|| {
                PyValueError::new_err(format!("Missing 'query_id' in pred_rels[{}]", i))
            })?
            .extract::<String>()?;
        let doc_id = rel
            .get_item("doc_id")?
            .ok_or_else(|| PyValueError::new_err(format!("Missing 'doc_id' in pred_rels[{}]", i)))?
            .extract::<String>()?;
        let score = rel
            .get_item("score")?
            .ok_or_else(|| PyValueError::new_err(format!("Missing 'score' in pred_rels[{}]", i)))?
            .extract::<f64>()?;
        let score = PredScore::from(score);
        b.add_record(query_id, doc_id, score).map_err(|e| {
            PyValueError::new_err(format!("Error adding record to PredRelStore: {}", e))
        })?;
    }
    let pred_rels = b.build();

    let result = elinor::evaluate(&true_rels, &pred_rels, metric)
        .map_err(|e| PyValueError::new_err(format!("Error evaluating: {}", e)))?;

    let dict = PyDict::new_bound(py);
    for (query_id, score) in result.scores() {
        dict.set_item(query_id, score)?;
    }
    Ok(dict.into())
}

#[pyfunction]
fn tupled_scores_from_score_maps<'py>(
    py: Python<'py>,
    score_maps: &Bound<'py, PyList>,
) -> PyResult<Py<PyList>> {
    let mut extracted = Vec::new();
    for score_map in score_maps.iter() {
        let score_map = score_map.downcast::<PyDict>()?;
        let score_map: BTreeMap<String, f64> = score_map.extract()?;
        extracted.push(score_map);
    }
    let tupled = elinor::tupled_scores_from_score_maps(&extracted)
        .map_err(|e| PyValueError::new_err(format!("Error tupling scores: {}", e)))?;
    Ok(PyList::new_bound(py, tupled).into())
}

/// A Python module implemented in Rust.
#[pymodule(name = "elinor")]
fn elinor_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(evaluate, m)?)?;
    m.add_function(wrap_pyfunction!(tupled_scores_from_score_maps, m)?)?;
    Ok(())
}

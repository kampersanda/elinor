use std::collections::BTreeMap;
use std::str::FromStr;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple};

#[pyfunction]
fn _evaluate<'py>(
    py: Python<'py>,
    true_rels: &Bound<'py, PyList>,
    pred_rels: &Bound<'py, PyList>,
    metric: &str,
) -> PyResult<Py<PyDict>> {
    let metric = elinor::Metric::from_str(metric)
        .map_err(|e| PyValueError::new_err(format!("Invalid metric: {}", e)))?;

    let mut b = elinor::TrueRelStoreBuilder::new();
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
            .extract::<elinor::TrueScore>()?;
        b.add_record(query_id, doc_id, score).map_err(|e| {
            PyValueError::new_err(format!("Error adding record to TrueRelStore: {}", e))
        })?;
    }
    let true_rels = b.build();

    let mut b = elinor::PredRelStoreBuilder::new();
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
        let score = elinor::PredScore::from(score);
        b.add_record(query_id, doc_id, score).map_err(|e| {
            PyValueError::new_err(format!("Error adding record to PredRelStore: {}", e))
        })?;
    }
    let pred_rels = b.build();

    let result = elinor::evaluate(&true_rels, &pred_rels, metric)
        .map_err(|e| PyValueError::new_err(format!("Error evaluating: {}", e)))?;

    let scores = PyDict::new_bound(py);
    for (query_id, score) in result.scores() {
        scores.set_item(query_id, score)?;
    }
    Ok(scores.into())
}

#[pyclass(frozen)]
struct _StudentTTest(elinor::statistical_tests::StudentTTest);

#[pymethods]
impl _StudentTTest {
    #[new]
    fn new(paired_samples: &Bound<'_, PyList>) -> PyResult<Self> {
        let mut pairs = Vec::new();
        for sample in paired_samples.iter() {
            let sample = sample.downcast::<PyTuple>()?;
            pairs.push(sample.extract::<(f64, f64)>()?);
        }
        let result = elinor::statistical_tests::StudentTTest::from_paired_samples(pairs)
            .map_err(|e| PyValueError::new_err(format!("Error creating StudentTTest: {}", e)))?;
        Ok(Self(result))
    }

    #[staticmethod]
    fn from_maps(a: &Bound<'_, PyDict>, b: &Bound<'_, PyDict>) -> PyResult<Self> {
        let a: BTreeMap<String, f64> = a.extract()?;
        let b: BTreeMap<String, f64> = b.extract()?;
        let pairs = elinor::statistical_tests::pairs_from_maps(&a, &b)
            .map_err(|e| PyValueError::new_err(format!("Error pairing scores: {}", e)))?;
        let result = elinor::statistical_tests::StudentTTest::from_paired_samples(pairs)
            .map_err(|e| PyValueError::new_err(format!("Error creating StudentTTest: {}", e)))?;
        Ok(Self(result))
    }

    fn n_samples(&self) -> usize {
        self.0.n_samples()
    }

    fn mean(&self) -> f64 {
        self.0.mean()
    }

    fn variance(&self) -> f64 {
        self.0.variance()
    }

    fn effect_size(&self) -> f64 {
        self.0.effect_size()
    }

    fn t_stat(&self) -> f64 {
        self.0.t_stat()
    }

    fn p_value(&self) -> f64 {
        self.0.p_value()
    }

    fn margin_of_error(&self, significance_level: f64) -> PyResult<f64> {
        self.0
            .margin_of_error(significance_level)
            .map_err(|e| PyValueError::new_err(format!("Error calculating margin of error: {}", e)))
    }

    fn confidence_interval(&self, significance_level: f64) -> PyResult<(f64, f64)> {
        self.0.confidence_interval(significance_level).map_err(|e| {
            PyValueError::new_err(format!("Error calculating confidence interval: {}", e))
        })
    }
}

/// A Python module implemented in Rust.
#[pymodule(name = "elinor")]
fn elinor_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(_evaluate, m)?)?;
    m.add_class::<_StudentTTest>()?;
    Ok(())
}

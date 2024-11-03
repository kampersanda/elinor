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

fn pylist_to_pairs(pairs: &Bound<'_, PyList>) -> PyResult<Vec<(f64, f64)>> {
    let mut result = Vec::new();
    for pair in pairs.iter() {
        let pair = pair.downcast::<PyTuple>()?;
        result.push(pair.extract::<(f64, f64)>()?);
    }
    Ok(result)
}

fn pylist_to_tuples(tuples: &Bound<'_, PyList>) -> PyResult<Vec<Vec<f64>>> {
    let mut result = Vec::new();
    for tuple in tuples.iter() {
        let tuple = tuple.downcast::<PyList>()?;
        result.push(tuple.extract::<Vec<f64>>()?);
    }
    Ok(result)
}

fn pydicts_to_pairs(a: &Bound<'_, PyDict>, b: &Bound<'_, PyDict>) -> PyResult<Vec<(f64, f64)>> {
    let a: BTreeMap<String, f64> = a.extract()?;
    let b: BTreeMap<String, f64> = b.extract()?;
    elinor::statistical_tests::pairs_from_maps(&a, &b)
        .map_err(|e| PyValueError::new_err(format!("Error pairing scores: {}", e)))
}

fn pydicts_to_tuples(maps: &Bound<'_, PyList>) -> PyResult<Vec<Vec<f64>>> {
    let mut btrees = Vec::new();
    for map in maps.iter() {
        let map = map.downcast::<PyDict>()?;
        let map: BTreeMap<String, f64> = map.extract()?;
        btrees.push(map);
    }
    elinor::statistical_tests::tuples_from_maps(&btrees)
        .map_err(|e| PyValueError::new_err(format!("Error converting maps to tuples: {}", e)))
}

#[pyclass(subclass, frozen)]
struct _StudentTTest(elinor::statistical_tests::StudentTTest);

#[pymethods]
impl _StudentTTest {
    #[new]
    fn new(paired_samples: &Bound<'_, PyList>) -> PyResult<Self> {
        let pairs = pylist_to_pairs(paired_samples)?;
        let result = elinor::statistical_tests::StudentTTest::from_paired_samples(pairs)
            .map_err(|e| PyValueError::new_err(format!("Error creating StudentTTest: {}", e)))?;
        Ok(Self(result))
    }

    #[staticmethod]
    fn from_maps(a: &Bound<'_, PyDict>, b: &Bound<'_, PyDict>) -> PyResult<Self> {
        let pairs = pydicts_to_pairs(a, b)?;
        let result = elinor::statistical_tests::StudentTTest::from_paired_samples(pairs)
            .map_err(|e| PyValueError::new_err(format!("Error creating StudentTTest: {}", e)))?;
        Ok(Self(result))
    }

    fn n_topics(&self) -> usize {
        self.0.n_topics()
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

#[pyclass(subclass, frozen)]
struct _BootstrapTest(elinor::statistical_tests::BootstrapTest);

#[pymethods]
impl _BootstrapTest {
    #[new]
    #[pyo3(signature = (paired_samples, n_resamples=10000, random_state=None))]
    fn new(
        paired_samples: &Bound<'_, PyList>,
        n_resamples: usize,
        random_state: Option<u64>,
    ) -> PyResult<Self> {
        let pairs = pylist_to_pairs(paired_samples)?;
        let mut tester = elinor::statistical_tests::bootstrap_test::BootstrapTester::new()
            .with_n_resamples(n_resamples);
        if let Some(random_state) = random_state {
            tester = tester.with_random_state(random_state);
        }
        let result = tester
            .test(pairs)
            .map_err(|e| PyValueError::new_err(format!("Error creating BootstrapTest: {}", e)))?;
        Ok(Self(result))
    }

    #[staticmethod]
    #[pyo3(signature = (a, b, n_resamples=10000, random_state=None))]
    fn from_maps(
        a: &Bound<'_, PyDict>,
        b: &Bound<'_, PyDict>,
        n_resamples: usize,
        random_state: Option<u64>,
    ) -> PyResult<Self> {
        let paired_samples = pydicts_to_pairs(a, b)?;
        let mut tester = elinor::statistical_tests::bootstrap_test::BootstrapTester::new()
            .with_n_resamples(n_resamples);
        if let Some(random_state) = random_state {
            tester = tester.with_random_state(random_state);
        }
        let result = tester
            .test(paired_samples)
            .map_err(|e| PyValueError::new_err(format!("Error creating BootstrapTest: {}", e)))?;
        Ok(Self(result))
    }

    fn n_topics(&self) -> usize {
        self.0.n_topics()
    }

    fn n_resamples(&self) -> usize {
        self.0.n_resamples()
    }

    fn random_state(&self) -> u64 {
        self.0.random_state()
    }

    fn p_value(&self) -> f64 {
        self.0.p_value()
    }
}

#[pyclass(subclass, frozen)]
struct _TwoWayAnovaWithoutReplication(elinor::statistical_tests::TwoWayAnovaWithoutReplication);

#[pymethods]
impl _TwoWayAnovaWithoutReplication {
    #[new]
    fn new(tupled_samples: &Bound<'_, PyList>, n_systems: usize) -> PyResult<Self> {
        let tuples = pylist_to_tuples(tupled_samples)?;
        let result = elinor::statistical_tests::TwoWayAnovaWithoutReplication::from_tupled_samples(
            tuples, n_systems,
        )
        .map_err(|e| {
            PyValueError::new_err(format!(
                "Error creating TwoWayAnovaWithoutReplication: {}",
                e
            ))
        })?;
        Ok(Self(result))
    }

    #[staticmethod]
    fn from_maps(maps: &Bound<'_, PyList>) -> PyResult<Self> {
        let tupled_samples = pydicts_to_tuples(maps)?;
        let result = elinor::statistical_tests::TwoWayAnovaWithoutReplication::from_tupled_samples(
            tupled_samples,
            maps.len(),
        )
        .map_err(|e| {
            PyValueError::new_err(format!(
                "Error creating TwoWayAnovaWithoutReplication: {}",
                e
            ))
        })?;
        Ok(Self(result))
    }

    fn n_systems(&self) -> usize {
        self.0.n_systems()
    }

    fn n_topics(&self) -> usize {
        self.0.n_topics()
    }

    fn system_means(&self) -> Vec<f64> {
        self.0.system_means()
    }

    fn topic_means(&self) -> Vec<f64> {
        self.0.topic_means()
    }

    fn between_system_variation(&self) -> f64 {
        self.0.between_system_variation()
    }

    fn between_topic_variation(&self) -> f64 {
        self.0.between_topic_variation()
    }

    fn residual_variation(&self) -> f64 {
        self.0.residual_variation()
    }

    fn between_system_variance(&self) -> f64 {
        self.0.between_system_variance()
    }

    fn between_topic_variance(&self) -> f64 {
        self.0.between_topic_variance()
    }

    fn residual_variance(&self) -> f64 {
        self.0.residual_variance()
    }

    fn between_system_f_stat(&self) -> f64 {
        self.0.between_system_f_stat()
    }

    fn between_topic_f_stat(&self) -> f64 {
        self.0.between_topic_f_stat()
    }

    fn between_system_p_value(&self) -> f64 {
        self.0.between_system_p_value()
    }

    fn between_topic_p_value(&self) -> f64 {
        self.0.between_topic_p_value()
    }

    fn margin_of_error(&self, significance_level: f64) -> PyResult<f64> {
        self.0
            .margin_of_error(significance_level)
            .map_err(|e| PyValueError::new_err(format!("Error calculating margin of error: {}", e)))
    }
}

#[pyclass(subclass, frozen)]
struct _TukeyHsdTest(elinor::statistical_tests::TukeyHsdTest);

#[pymethods]
impl _TukeyHsdTest {
    #[new]
    fn new(tupled_samples: &Bound<'_, PyList>, n_systems: usize) -> PyResult<Self> {
        let tuples = pylist_to_tuples(tupled_samples)?;
        let result =
            elinor::statistical_tests::TukeyHsdTest::from_tupled_samples(tuples, n_systems)
                .map_err(|e| {
                    PyValueError::new_err(format!("Error creating TukeyHsdTest: {}", e))
                })?;
        Ok(Self(result))
    }

    #[staticmethod]
    fn from_maps(maps: &Bound<'_, PyList>) -> PyResult<Self> {
        let tupled_samples = pydicts_to_tuples(maps)?;
        let result = elinor::statistical_tests::TukeyHsdTest::from_tupled_samples(
            tupled_samples,
            maps.len(),
        )
        .map_err(|e| PyValueError::new_err(format!("Error creating TukeyHsdTest: {}", e)))?;
        Ok(Self(result))
    }

    fn n_systems(&self) -> usize {
        self.0.n_systems()
    }

    fn n_topics(&self) -> usize {
        self.0.n_topics()
    }

    fn effect_sizes(&self) -> Vec<Vec<f64>> {
        self.0.effect_sizes()
    }
}

#[pyclass(subclass, frozen)]
struct _RandomizedTukeyHsdTest(elinor::statistical_tests::RandomizedTukeyHsdTest);

#[pymethods]
impl _RandomizedTukeyHsdTest {
    #[new]
    #[pyo3(signature = (tupled_samples, n_systems, n_iters=10000, random_state=None))]
    fn new(
        tupled_samples: &Bound<'_, PyList>,
        n_systems: usize,
        n_iters: usize,
        random_state: Option<u64>,
    ) -> PyResult<Self> {
        let tuples = pylist_to_tuples(tupled_samples)?;
        let mut tester =
            elinor::statistical_tests::randomized_tukey_hsd_test::RandomizedTukeyHsdTester::new(
                n_systems,
            )
            .with_n_iters(n_iters);
        if let Some(random_state) = random_state {
            tester = tester.with_random_state(random_state);
        }
        let result = tester.test(tuples).map_err(|e| {
            PyValueError::new_err(format!("Error creating RandomizedTukeyHsdTest: {}", e))
        })?;
        Ok(Self(result))
    }

    #[staticmethod]
    #[pyo3(signature = (maps, n_iters=10000, random_state=None))]
    fn from_maps(
        maps: &Bound<'_, PyList>,
        n_iters: usize,
        random_state: Option<u64>,
    ) -> PyResult<Self> {
        let tupled_samples = pydicts_to_tuples(maps)?;
        let mut tester =
            elinor::statistical_tests::randomized_tukey_hsd_test::RandomizedTukeyHsdTester::new(
                maps.len(),
            )
            .with_n_iters(n_iters);
        if let Some(random_state) = random_state {
            tester = tester.with_random_state(random_state);
        }
        let result = tester.test(tupled_samples).map_err(|e| {
            PyValueError::new_err(format!("Error creating RandomizedTukeyHsdTest: {}", e))
        })?;
        Ok(Self(result))
    }

    fn n_systems(&self) -> usize {
        self.0.n_systems()
    }

    fn n_topics(&self) -> usize {
        self.0.n_topics()
    }

    fn n_iters(&self) -> usize {
        self.0.n_iters()
    }

    fn random_state(&self) -> u64 {
        self.0.random_state()
    }

    fn p_values(&self) -> Vec<Vec<f64>> {
        self.0.p_values()
    }
}

/// A Python module implemented in Rust.
#[pymodule(name = "_elinor")]
fn elinor_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(_evaluate, m)?)?;
    m.add_class::<_StudentTTest>()?;
    m.add_class::<_BootstrapTest>()?;
    m.add_class::<_TwoWayAnovaWithoutReplication>()?;
    m.add_class::<_TukeyHsdTest>()?;
    m.add_class::<_RandomizedTukeyHsdTest>()?;
    Ok(())
}

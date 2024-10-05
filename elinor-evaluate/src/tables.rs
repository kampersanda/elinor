use std::collections::{BTreeMap, HashMap, HashSet};
use std::io::{Read, Write};

use anyhow::Result;
use big_s::S;
use elinor::statistical_tests::BootstrapTest;
use elinor::statistical_tests::RandomizedTukeyHsdTest;
use elinor::statistical_tests::StudentTTest;
use elinor::statistical_tests::TwoWayAnovaWithoutReplication;
use elinor::Metric;
use prettytable::{Cell, Table};

type Evaluated = elinor::Evaluated<String>;

/// Table of scores for each query and metric.
pub struct ScoreTable {
    table: BTreeMap<String, BTreeMap<Metric, f64>>,
}

impl ScoreTable {
    pub fn new() -> Self {
        Self {
            table: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, metric: Metric, evaluated: &Evaluated) -> Result<()> {
        // Check if the query ids are the same
        if !self.table.is_empty() {
            let query_ids: HashSet<_> = self.table.keys().collect();
            let new_query_ids: HashSet<_> = evaluated.scores().keys().collect();
            if query_ids != new_query_ids {
                return Err(anyhow::anyhow!("Query IDs are not the same"));
            }
        }
        for (query, score) in evaluated.scores() {
            self.table
                .entry(query.clone())
                .or_insert_with(BTreeMap::new)
                .insert(metric, *score);
        }
        Ok(())
    }

    pub fn metrics(&self) -> Vec<Metric> {
        let row = self.table.values().next().unwrap();
        row.keys().cloned().collect()
    }

    pub fn to_results(&self) -> BTreeMap<Metric, Evaluated> {
        let mut results = BTreeMap::new();
        for metric in self.metrics() {
            let mut query_to_score = HashMap::new();
            for (query, metric_to_score) in &self.table {
                let score = metric_to_score.get(&metric).unwrap();
                query_to_score.insert(query.clone(), *score);
            }
            let evaluated = Evaluated::from_scores(query_to_score);
            results.insert(metric, evaluated);
        }
        results
    }

    pub fn into_csv<W: Write>(&self, wtr: &mut csv::Writer<W>) -> Result<()> {
        let metrics = self.metrics();
        wtr.write_record(
            std::iter::once(S("Query")).chain(metrics.iter().map(|m| format!("{m}"))),
        )?;
        for (query, scores) in &self.table {
            let mut record = vec![query.clone()];
            for metric in &metrics {
                let score = scores.get(metric).unwrap();
                record.push(format!("{score:.4}"));
            }
            wtr.write_record(&record)?;
        }
        Ok(())
    }

    pub fn from_csv<R: Read>(rdr: &mut csv::Reader<R>) -> Result<Self> {
        let mut table = BTreeMap::new();
        let headers = rdr.headers()?.clone();
        let metrics: Vec<Metric> = headers
            .iter()
            .skip(1)
            .map(|h| h.parse::<Metric>())
            .collect::<Result<_, _>>()?;
        for result in rdr.records() {
            let result = result?;
            let query = result.get(0).unwrap().to_string();
            let mut scores = BTreeMap::new();
            for (metric, score) in metrics.iter().zip(result.iter().skip(1)) {
                scores.insert(metric.clone(), score.parse()?);
            }
            table.insert(query, scores);
        }
        Ok(Self { table })
    }
}

pub struct MetricTable {
    names: Vec<String>,
    table: BTreeMap<Metric, BTreeMap<String, Evaluated>>,
}

impl MetricTable {
    pub fn new() -> Self {
        Self {
            names: Vec::new(),
            table: BTreeMap::new(),
        }
    }

    pub fn insert<S>(&mut self, metric: Metric, name: S, evaluated: Evaluated)
    where
        S: AsRef<str>,
    {
        let name = name.as_ref();
        if !self.names.contains(&name.to_string()) {
            self.names.push(name.to_string());
        }
        self.table
            .entry(metric)
            .or_insert_with(BTreeMap::new)
            .insert(name.to_string(), evaluated);
    }

    pub fn prettytable(&self) -> Table {
        let mut rows: Vec<Vec<String>> = Vec::new();
        {
            let mut header = vec![S("Metric")];
            header.extend(self.names.iter().cloned());
            rows.push(header);
        }
        for (metric, name_to_result) in &self.table {
            let mut row = vec![format!("{metric}")];
            for name in &self.names {
                let evaluated = name_to_result.get(name).unwrap();
                let mean = evaluated.mean_score();
                row.push(format!("{mean:.4}"));
            }
            rows.push(row);
        }
        create_table(rows)
    }

    pub fn metrics(&self) -> Vec<Metric> {
        self.table.keys().cloned().collect()
    }

    pub fn get(&self, metric: &Metric, name: &str) -> Option<&Evaluated> {
        self.table.get(metric)?.get(name)
    }

    pub fn get_all(&self, metric: &Metric) -> Vec<Evaluated> {
        self.table.get(metric).unwrap().values().cloned().collect()
    }
}

pub struct PairedComparisonTable {
    paired_results: BTreeMap<Metric, (Evaluated, Evaluated)>,
}

impl PairedComparisonTable {
    pub fn new() -> Self {
        Self {
            paired_results: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, metric: Metric, result_a: Evaluated, result_b: Evaluated) {
        self.paired_results.insert(metric, (result_a, result_b));
    }

    pub fn summarize(&self) {
        println!("Paired Student's t-test");
        self.summarize_student_t_test(&mut std::io::stdout());
        println!("Bootstrap test");
        self.summarize_bootstrap_test(&mut std::io::stdout());
        println!("Fisher's randomized test");
        self.summarize_randomized_test(&mut std::io::stdout());
    }

    pub fn summarize_student_t_test<W>(&self, wtr: &mut W)
    where
        W: Write + ?Sized,
    {
        let mut rows: Vec<Vec<String>> = Vec::new();
        rows.push(vec![
            S("Metric"),
            S("Mean"),
            S("Variance"),
            S("Effect Size"),
            S("T Stat"),
            S("P Value"),
            S("95% CI"),
        ]);
        for (metric, (result_a, result_b)) in &self.paired_results {
            let paired_scores = elinor::paired_scores_from_evaluated(&result_a, &result_b).unwrap();
            let stat = StudentTTest::from_paired_samples(paired_scores).unwrap();
            let (ci95_btm, ci95_top) = stat.confidence_interval(0.05).unwrap();
            rows.push(vec![
                format!("{metric}"),
                format!("{:.4}", stat.mean()),
                format!("{:.4}", stat.var()),
                format!("{:.4}", stat.effect_size()),
                format!("{:.4}", stat.t_stat()),
                format!("{:.4}", stat.p_value()),
                format!("[{:.4}, {:.4}]", ci95_btm, ci95_top),
            ]);
        }
        create_table(rows).print(wtr).unwrap();
    }

    pub fn summarize_bootstrap_test<W>(&self, wtr: &mut W)
    where
        W: Write + ?Sized,
    {
        let mut rows: Vec<Vec<String>> = Vec::new();
        rows.push(vec![S("Metric"), S("P Value")]);
        for (metric, (result_a, result_b)) in &self.paired_results {
            let paired_scores = elinor::paired_scores_from_evaluated(&result_a, &result_b).unwrap();
            let stat = BootstrapTest::from_paired_samples(paired_scores).unwrap();
            rows.push(vec![format!("{metric}"), format!("{:.4}", stat.p_value())]);
        }
        create_table(rows).print(wtr).unwrap();
    }

    pub fn summarize_randomized_test<W>(&self, wtr: &mut W)
    where
        W: Write + ?Sized,
    {
        let mut rows: Vec<Vec<String>> = Vec::new();
        rows.push(vec![S("Metric"), S("P Value")]);
        for (metric, (result_a, result_b)) in &self.paired_results {
            let tupled_scores =
                elinor::tupled_scores_from_evaluated(&[result_a.clone(), result_b.clone()])
                    .unwrap();
            let stat = RandomizedTukeyHsdTest::from_tupled_samples(tupled_scores, 2).unwrap();
            rows.push(vec![
                format!("{metric}"),
                format!("{:.4}", stat.p_value(0, 1).unwrap()),
            ]);
        }
        create_table(rows).print(wtr).unwrap();
    }
}

pub struct TupledComparisonTable {
    tupled_results: BTreeMap<Metric, Vec<Evaluated>>,
}

impl TupledComparisonTable {
    pub fn new() -> Self {
        Self {
            tupled_results: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, metric: Metric, results: Vec<Evaluated>) {
        self.tupled_results.insert(metric, results);
    }

    pub fn confidence_intervals(&self, metric: Metric) {
        let results = self.tupled_results.get(&metric).unwrap();
        let tupled_scores = elinor::tupled_scores_from_evaluated(results).unwrap();
        let n_systems = results.len();
        let stat =
            TwoWayAnovaWithoutReplication::from_tupled_samples(tupled_scores, n_systems).unwrap();

        let system_means = stat.system_means();
        let moe95 = stat.margin_of_error(0.05).unwrap();

        let mut rows: Vec<Vec<String>> = Vec::new();
        rows.push(vec![S("System"), S("Mean"), S("95% CI")]);
        for i in 0..n_systems {
            let mean = system_means[i];
            rows.push(vec![
                format!("System {}", i + 1),
                format!("{:.4}", mean),
                format!("[{:.4}, {:.4}]", mean - moe95, mean + moe95),
            ]);
        }
        create_table(rows).printstd();
    }

    pub fn effect_sizes(&self, metric: Metric) {
        let results = self.tupled_results.get(&metric).unwrap();
        let tupled_scores = elinor::tupled_scores_from_evaluated(results).unwrap();
        let n_systems = results.len();
        let stat =
            TwoWayAnovaWithoutReplication::from_tupled_samples(tupled_scores, n_systems).unwrap();
        let effect_sizes = stat.between_system_effect_sizes();
        let mut rows: Vec<Vec<String>> = Vec::new();
        {
            let mut header = vec![S("Effect Size")];
            for i in 1..results.len() {
                header.push(format!("System {}", i + 1));
            }
            rows.push(header);
        }
        for i in 0..(n_systems - 1) {
            let mut row = vec![format!("System {}", i + 1)];
            for _ in 0..i {
                row.push(S(""));
            }
            for j in (i + 1)..n_systems {
                let effect_size = effect_sizes[i][j];
                row.push(format!("{effect_size:.4}"));
            }
            rows.push(row);
        }
        create_table(rows).printstd();
    }

    pub fn printstd(&self) {
        for (metric, results) in &self.tupled_results {
            println!("{metric}");

            self.confidence_intervals(metric.clone());

            let tupled_scores = elinor::tupled_scores_from_evaluated(results).unwrap();
            let stat =
                RandomizedTukeyHsdTest::from_tupled_samples(tupled_scores, results.len()).unwrap();
            let n_systems = results.len();
            let mut rows: Vec<Vec<String>> = Vec::new();
            {
                let mut header = vec![S("p-value")];
                for i in 1..results.len() {
                    header.push(format!("System {}", i + 1));
                }
                rows.push(header);
            }
            for i in 0..(n_systems - 1) {
                let mut row = vec![format!("System {}", i + 1)];
                for _ in 0..i {
                    row.push(S(""));
                }
                for j in (i + 1)..n_systems {
                    let p_value = stat.p_value(i, j).unwrap();
                    row.push(format!("{p_value:.4}"));
                }
                rows.push(row);
            }
            create_table(rows).printstd();

            self.effect_sizes(metric.clone());
        }
    }
}

fn create_table(rows: Vec<Vec<String>>) -> Table {
    let mut table = Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(rows[0].iter().map(|s| Cell::new(s)).collect());
    for row in rows.iter().skip(1) {
        table.add_row(row.iter().map(|s| Cell::new(s)).collect());
    }
    table
}

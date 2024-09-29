use std::collections::BTreeMap;

use big_s::S;
use elinor::statistical_tests::RandomizedTukeyHsdTest;
use elinor::statistical_tests::StudentTTest;
use elinor::Metric;
use prettytable::{Cell, Table};

type Evaluated = elinor::Evaluated<String>;

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

    pub fn printstd(&self) {
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
                let mean_score = evaluated.mean_score();
                row.push(format!("{mean_score:.4}"));
            }
            rows.push(row);
        }
        create_table(rows).printstd();
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

    pub fn printstd(&self) {
        let mut rows: Vec<Vec<String>> = Vec::new();
        rows.push(vec![
            S("Metric"),
            S("Mean"),
            S("Variance"),
            S("Effect Size"),
            S("T Stat"),
            S("P Value"),
        ]);
        for (metric, (result_a, result_b)) in &self.paired_results {
            let paired_scores = elinor::paired_scores_from_evaluated(&result_a, &result_b).unwrap();
            let stat = StudentTTest::from_paired_samples(paired_scores).unwrap();
            rows.push(vec![
                format!("{metric}"),
                format!("{:.4}", stat.mean()),
                format!("{:.4}", stat.var()),
                format!("{:.4}", stat.effect_size()),
                format!("{:.4}", stat.t_stat()),
                format!("{:.4}", stat.p_value()),
            ]);
        }
        create_table(rows).printstd();
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

    pub fn printstd(&self) {
        for (metric, results) in &self.tupled_results {
            println!("{metric}");

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

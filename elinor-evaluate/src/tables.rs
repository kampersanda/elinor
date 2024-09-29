use std::collections::BTreeMap;

use elinor::Metric;
use prettytable::{Cell, Table};

type Evaluated = elinor::Evaluated<String>;

pub struct MetricTable {
    system_names: Vec<String>,
    table: BTreeMap<Metric, BTreeMap<String, Evaluated>>,
}

impl MetricTable {
    pub fn new<I, S>(system_names: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let system_names = system_names
            .into_iter()
            .map(|s| s.as_ref().to_string())
            .collect();
        Self {
            system_names,
            table: BTreeMap::new(),
        }
    }

    pub fn insert<S>(&mut self, metric: Metric, system_name: S, evaluated: Evaluated)
    where
        S: AsRef<str>,
    {
        self.table
            .entry(metric)
            .or_insert_with(BTreeMap::new)
            .insert(system_name.as_ref().to_string(), evaluated);
    }

    pub fn printstd(&self) {
        let mut rows: Vec<Vec<String>> = Vec::new();
        {
            let mut header = vec!["Metric".to_string()];
            header.extend(self.system_names.iter().cloned());
            rows.push(header);
        }
        for (metric, system_to_result) in &self.table {
            let mut row = vec![format!("{metric}")];
            for system_name in &self.system_names {
                let evaluated = system_to_result.get(system_name).unwrap();
                let mean_score = evaluated.mean_score();
                row.push(format!("{mean_score:.4}"));
            }
            rows.push(row);
        }
        create_table(rows).printstd();
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

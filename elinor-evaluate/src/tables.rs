use std::collections::BTreeMap;

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
            let mut header = vec!["Metric".to_string()];
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

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use anyhow::Result;
use prettytable::{Cell, Table};

/// Load lines from a file.
pub fn load_lines<P: AsRef<Path>>(file: P) -> Result<Vec<String>> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
    Ok(lines)
}

pub fn to_prettytable(rows: Vec<Vec<String>>) -> Table {
    let mut table = Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(rows[0].iter().map(|s| Cell::new(s)).collect());
    for row in rows.iter().skip(1) {
        table.add_row(row.iter().map(|s| Cell::new(s)).collect());
    }
    table
}

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use anyhow::Result;

/// Load lines from a file.
pub fn load_lines<P: AsRef<Path>>(file: P) -> Result<Vec<String>> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
    Ok(lines)
}

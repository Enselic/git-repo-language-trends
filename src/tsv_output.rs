use std::io;

use crate::output::Output;
use crate::Column;
use crate::ColumnToLinesMap;

/// Supports writing tab-separated value output to any writer.
/// Typically the output will go to stdout or to a file.
pub struct TabSeparatedValuesOutput<D: io::Write> {
    dest: D,
    columns: Vec<Column>,
}

impl<D: io::Write> TabSeparatedValuesOutput<D> {
    pub fn new(dest: D) -> Self {
        TabSeparatedValuesOutput {
            dest,
            columns: vec![],
        }
    }
}

impl<D: std::io::Write> Output for TabSeparatedValuesOutput<D> {
    fn start(&mut self, columns: &[Column]) -> io::Result<()> {
        self.columns.extend_from_slice(columns);

        Ok(write_header_row(&mut self.dest, columns)?)
    }

    fn add_row(&mut self, date: &str, column_to_lines_map: &ColumnToLinesMap) -> io::Result<()> {
        Ok(write_row(
            &mut self.dest,
            date,
            &self.columns,
            column_to_lines_map,
        )?)
    }

    fn finish(&mut self) -> io::Result<()> {
        eprintln!("\nCopy and paste the above output into your favourite spreadsheet software and make a graph.");

        Ok(())
    }
}

fn write_header_row(
    write: &mut dyn std::io::Write,
    columns: &[crate::Column],
) -> std::io::Result<()> {
    // To get correct tab alignment, pad with spaces in place of a date
    // on the first row
    write!(write, "{}", " ".repeat("YYYY-MM-DD".len()))?;

    // Now write the columns
    for ext in columns {
        write!(write, "\t{}", ext)?;
    }

    // ... and finish with a newline
    writeln!(write)?;

    Ok(())
}

fn write_row(
    write: &mut dyn std::io::Write,
    date: &str,
    columns: &[crate::Column],
    column_to_lines_map: &crate::ColumnToLinesMap,
) -> std::io::Result<()> {
    // Date
    write!(write, "{}", date)?;

    // Line count information
    for column in columns {
        write!(write, "\t{}", column_to_lines_map.get(column).unwrap_or(&0))?;
    }

    // ... and finish with a newline
    writeln!(write)?;

    Ok(())
}

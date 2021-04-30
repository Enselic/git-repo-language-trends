use crate::*;

/// Implemented this to support formating the collected data in some kind of output format.
/// Example output formats could be stdout printouts, a .tsv file, or a .png file with a graph.
pub trait Output {
    /// Called in the beginning of processing,
    /// to announce what columns to use in the output.
    fn start(&mut self, columns: &[Column]) -> Result<()>;

    /// Called each time a commit has been analyzed,
    /// and it is time to present the data in the commit.
    fn add_row(&mut self, date: &str, column_to_lines_map: &ColumnToLinesMap) -> Result<()>;

    /// Called when the processing of commits is complete.
    /// This is a good time to write output files to disk.
    fn finish(&mut self) -> Result<()>;
}

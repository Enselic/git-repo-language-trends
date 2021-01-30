pub struct BenchmarkData {
    start_time: std::time::Instant,
    pub total_lines_counted: usize,
    pub total_files_processed: usize,
}

impl BenchmarkData {
    pub fn start_if_activated(args: &super::Args) -> Option<BenchmarkData> {
        if args.benchmark {
            Some(BenchmarkData {
                start_time: std::time::Instant::now(),
                total_lines_counted: 0,
                total_files_processed: 0,
            })
        } else {
            None
        }
    }

    pub fn report(self) {
        let end_time = std::time::Instant::now();
        let duration = end_time - self.start_time;
        let seconds = duration.as_secs_f64();
        let lines_per_second = self.total_lines_counted as f64 / seconds;
        let files_per_second = self.total_files_processed as f64 / seconds;
        let lines_per_file = self.total_lines_counted as f64 / self.total_files_processed as f64;
        println!(
            "
Counted {} lines in {} files in {:.3} seconds. On average:
{} lines/second
{} files/second
{} lines/file",
            self.total_lines_counted,
            self.total_files_processed,
            seconds,
            lines_per_second.floor(),
            files_per_second.floor(),
            lines_per_file.floor(),
        );
    }
}

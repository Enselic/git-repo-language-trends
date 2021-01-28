use indicatif::ProgressBar as PB;
use indicatif::ProgressStyle;
use std::time::Instant;

// Performance suffers if the progress bar is updated too frequently, so
// rate-limit it
const MIN_DURATION_NS: u32 = 250_000_000;

pub struct ProgressBar {
    pb: PB,
    last_updated: Instant,
}

impl ProgressBar {
    pub fn setup(len: usize, prefix: &str) -> Self {
        let pb = PB::new(len as u64);
        pb.set_prefix(prefix);
        pb.set_style(
            ProgressStyle::default_bar().template("{prefix} {wide_bar} {pos}/{len} files"),
        );
        ProgressBar {
            pb,
            last_updated: Instant::now(),
        }
    }

    pub fn set_position_rate_limited(&mut self, pos: usize) {
        let now = Instant::now();
        if now.duration_since(self.last_updated).subsec_nanos() > MIN_DURATION_NS {
            self.last_updated = now;
            self.pb.set_position(pos as u64);
        }
    }

    pub fn finish_and_clear(&self) {
        self.pb.finish_and_clear();
    }
}

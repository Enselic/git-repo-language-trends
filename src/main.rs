use chrono::DateTime;
use chrono::Utc;

use std::collections::HashMap;
use std::collections::HashSet;
use structopt::StructOpt;

mod benchmark;
use benchmark::BenchmarkData;
mod gitutils;
use gitutils::Repo;

#[derive(Debug, StructOpt)]
#[structopt(about = "\
Prints tabulated data about programming language usage over time in a git repository.

Copy-paste the output into your favourite spreadsheet software to easily make a graph.
Stacked area chart is recommended.

EXAMPLES
    cd ~/src/your-repo                            # Go to any git repository
    git-repo-language-trend .cpp  .rs             # C++ vs Rust
    git-repo-language-trend .java .kt             # Java vs Kotlin
    git-repo-language-trend .m    .swift          # Objective-C vs Swift
")]
pub struct Args {
    /// For what file extensions lines will be counted.
    #[structopt(name = ".ext1 .ext2 ...")]
    filter: Vec<String>,

    /// Optional. The mimimum interval in days between data points.
    #[structopt(long, default_value = "7")]
    min_interval: u32,

    /// Optional. Maximum number of data rows to print.
    #[structopt(long, default_value = "18446744073709551615")]
    max_rows: u64,

    /// Optional. The commit to start parsing from.
    #[structopt(long, default_value = "HEAD")]
    start_commit: String,

    /// Prints total counted lines/second.
    #[structopt(long)]
    benchmark: bool,

    /// Prints debug information during processing.
    #[structopt(long)]
    debug: bool,

    /// (Advanced.) The progress bar slows down performance slightly. Enable
    /// this flag to maximize performance. You can use --benchmark to measure if
    /// there is an actual difference.
    #[structopt(long)]
    disable_progress_bar: bool,

    /// (Advanced.) By default, --first-parent is passed to the internal git log
    /// command. This ensures that the data in each row comes from a source code
    /// tree that is an ancestor to the row above it. If you prefer data for as
    /// many commits as possible, even though the data can become inconsistent
    /// ("jumpy"), enable this flag.
    #[structopt(long)]
    all_parents: bool,
}

fn run(args: &Args) -> Result<(), git2::Error> {
    let repo = Repo::from_path(std::env::var("GIT_DIR").unwrap_or_else(|_| ".".to_owned()))?;
    let extensions = get_reasonable_set_of_extensions(&repo, &args)?;

    // Print headers
    print!("          "); // For "YYYY-MM-DD"
    for ext in &extensions {
        print!("\t{}", ext);
    }
    println!();

    // Print rows
    let mut benchmark_data = BenchmarkData::start_if_activated(args);
    let mut rows_left = args.max_rows;
    let mut date_of_last_row: Option<DateTime<Utc>> = None;
    for (current_date, commit) in repo.git_log(&args)? {
        if rows_left == 0 {
            break;
        }

        if args.debug {
            eprintln!("-> Looking at {} {:?} ...", current_date, commit);
        }

        let min_interval_days_passed = match date_of_last_row {
            Some(date_of_last_row) => {
                let time_passed = date_of_last_row.signed_duration_since(current_date);

                if args.debug {
                    eprintln!("time_passed={:?}", time_passed);
                }

                // NOTE: Takes hour of the day into account; date day
                // can be different without a full day having passed
                time_passed.num_days() >= args.min_interval as i64
            }
            None => true,
        };
        if min_interval_days_passed {
            process_and_print_row(
                &repo,
                &format!("{}", current_date.format("%Y-%m-%d")),
                &commit,
                &extensions,
                &mut benchmark_data,
                &args,
            )?;
            date_of_last_row = Some(current_date);
            rows_left -= 1;
        }
    }

    if let Some(benchmark_data) = benchmark_data {
        benchmark_data.report();
    }

    eprintln!("\nCopy and paste the above output into your favourite spreadsheet software and make a graph.");

    Ok(())
}

fn process_and_print_row(
    repo: &Repo,
    date: &str,
    commit: &git2::Commit,
    extensions: &[String],
    benchmark_data: &mut Option<BenchmarkData>,
    args: &Args,
) -> Result<(), git2::Error> {
    let data = process_commit(repo, date, commit, extensions, benchmark_data, args)?;
    print!("{}", date);
    for ext in extensions {
        print!("\t{}", data.get(ext).unwrap_or(&0));
    }
    println!();

    Ok(())
}

fn process_commit(
    repo: &Repo,
    date: &str,
    commit: &git2::Commit,
    extensions: &[String],
    benchmark_data: &mut Option<BenchmarkData>,
    args: &Args,
) -> Result<HashMap<String, usize>, git2::Error> {
    let blobs = repo.get_blobs_in_commit(commit)?;

    const MIN_TIME_IN_NANOS: u32 = 250_000_000;
    use indicatif::{ProgressBar, ProgressStyle};
    use std::time::Instant;
    let mut last_update = Instant::now();
    let progress_bar = if !args.disable_progress_bar {
        let pb = ProgressBar::new(blobs.len() as u64);
        pb.set_prefix(date);
        pb.set_style(
            ProgressStyle::default_bar().template("{prefix} {wide_bar} {pos}/{len} files"),
        );
        Some(pb)
    } else {
        None
    };

    let mut ext_to_total_lines: HashMap<String, usize> = HashMap::new();
    for (index, blob) in blobs.iter().enumerate() {
        if let Some(progress_bar) = &progress_bar {
            let now = Instant::now();
            if now.duration_since(last_update).subsec_nanos() > MIN_TIME_IN_NANOS {
                last_update = now;
                progress_bar.set_position(index as u64);
            }
        }

        if extensions.contains(&blob.1) {
            let lines = repo.get_lines_in_blob(&blob.0)?;
            let total_lines = ext_to_total_lines.entry(blob.1.clone()).or_insert(0);
            *total_lines += lines;

            if let Some(benchmark_data) = benchmark_data {
                benchmark_data.total_files_processed += 1;
                benchmark_data.total_lines_counted += lines;
            }
        }
    }

    if let Some(progress_bar) = &progress_bar {
        progress_bar.finish_and_clear();
    }

    Ok(ext_to_total_lines)
}

fn get_reasonable_set_of_extensions(repo: &Repo, args: &Args) -> Result<Vec<String>, git2::Error> {
    Ok(if !args.filter.is_empty() {
        // Easy, just use what the user wishes
        args.filter.clone()
    } else {
        // Calculate a reasonable set of extension to count lines for using the
        // file extensions present in the first commit
        eprintln!("INFO: Run\n\n  git-repo-language-trends .ext1 .ext2 ...\n\nto select which file extensions to count lines for.\n");
        let commit = repo.repo.revparse_single(&args.start_commit)?;
        let blobs = repo.get_blobs_in_commit(&commit.peel_to_commit().unwrap())?;
        let exts: HashSet<String> = blobs.into_iter().map(|e| e.1).collect();
        // TODO: Unit test this code
        let mut result: Vec<String> = exts
            .into_iter()
            .filter(|e| {
                let mime = mime_guess::from_path(format!("temp{}", e))
                    .first_or_text_plain()
                    .essence_str()
                    .to_owned();
                if args.debug {
                    eprintln!("Mapped {} to {}", e, mime);
                }
                !(mime.starts_with("image")
                    || mime.starts_with("video")
                    || mime.starts_with("audio")
                    || mime.contains("archive")
                    || mime.contains("cert")
                    || (mime == "application/octet-stream" && e != ".java")
                    || e.starts_with(".git")
                    || ".json" == e
                    || ".lock" == e)
            })
            .collect();
        result.sort();
        result
    })
}

fn main() {
    let args = Args::from_args();
    match run(&args) {
        Ok(()) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}

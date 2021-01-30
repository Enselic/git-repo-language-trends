use chrono::DateTime;
use chrono::Utc;

use std::collections::HashMap;
use structopt::StructOpt;

mod benchmark;
use benchmark::BenchmarkData;
mod progressbar;
use progressbar::ProgressBar;
mod repoutils;
use repoutils::Repo;
mod utils;

#[derive(Debug, StructOpt)]
#[structopt(about = "\
Prints tabulated data about programming language usage over time in a git repository.

Copy-paste the output into your favourite spreadsheet software to easily make a graph.
Stacked area chart is recommended.

EXAMPLES
    cd ~/src/your-repo                       # Go to any git repository
    git-repo-language-trend .m+.h .swift     # Objective-C vs Swift (with .m and .h files summed together)
    git-repo-language-trend .java .kt        # Java vs Kotlin
")]
pub struct Args {
    /// For what file extensions lines will be counted.
    #[structopt(name = ".ext1 .ext2+.ext3 .ext4  ... ")]
    columns: Vec<String>,

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

    /// Lists available file extensions (in the first commit).
    #[structopt(long)]
    list: bool,

    /// (Advanced.) The progress bar slows down performance slightly. Enable
    /// this flag to maximize performance. You can use --benchmark to measure if
    /// there is an actual difference for your system.
    #[structopt(long)]
    disable_progress_bar: bool,

    /// (Advanced.) By default, --first-parent is passed to the internal git log
    /// command (or libgit2 Rust binding code rather). This ensures that the
    /// data in each row comes from a source code tree that is an ancestor to
    /// the row above it. If you prefer data for as many commits as possible,
    /// even though the data can become inconsistent (a.k.a. "jumpy"), enable
    /// this flag.
    #[structopt(long)]
    all_parents: bool,
}

/// Represents a column in the tabulated output (and command line argument
/// input). Examples values: ".h+.c" for auto-summation of C header and source
/// files. But more commonly just e.g. ".rs" or ".kt" or ".swift"
type Column = String;

/// Represents a file extension, such as ".rs" or ".kt" or ".swift"
type Extension = String;

/// Maps a column to the total number of lines in files that belong to that
/// column. Usually a column is just for one file extension, but we also support
/// auto-summation, in which case a column is e.g. ".c+.h".
type ColumnToLinesMap = HashMap<Column, usize>;

/// Type that maps e.g. the ".c" and ".h" extensions of C files to the the
/// ".c+.h" column when auto-summation is used. But more commonly this is just a
/// map to self. e.g. ".rs" to ".rs".
type ExtensionToColumnMap = HashMap<Extension, Column>;

fn run(args: &Args) -> Result<(), git2::Error> {
    let repo = Repo::from_path(std::env::var("GIT_DIR").unwrap_or_else(|_| ".".to_owned()))?;

    if args.list {
        let data = get_data_for_start_commit(&repo, &args)?;
        println!(
            "Available extensions (in first commit):\n{}",
            utils::get_extensions_sorted_by_popularity(&data).join(" ")
        );
        return Ok(());
    }

    let columns = get_reasonable_set_of_columns(&repo, &args)?;
    if columns.is_empty() {
        eprintln!("Could not find any file extensions, try specifying them manually");
        return Ok(());
    }
    let ext_to_column = generate_extension_to_column_map(&columns);

    // Print headers
    print!("          "); // For "YYYY-MM-DD"
    for ext in &columns {
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

        // Make sure --min-interval days has passed since last printed commit before
        // processing and printing the data for another commit
        if enough_days_passed(&args, date_of_last_row, current_date) {
            date_of_last_row = Some(current_date);
            process_and_print_row(
                &repo,
                &current_date.format("%Y-%m-%d").to_string(),
                &commit,
                &columns,
                &ext_to_column,
                &mut benchmark_data,
                &args,
            )?;
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
    columns: &[Column],
    ext_to_column: &ExtensionToColumnMap,
    benchmark_data: &mut Option<BenchmarkData>,
    args: &Args,
) -> Result<(), git2::Error> {
    let data = process_commit(
        repo,
        Some(date),
        commit,
        Some(ext_to_column),
        !args.disable_progress_bar,
        benchmark_data,
    )?;
    print!("{}", date);
    for column in columns {
        print!("\t{}", data.get(column).unwrap_or(&0));
    }
    println!();

    Ok(())
}

fn process_commit(
    repo: &Repo,
    date: Option<&str>,
    commit: &git2::Commit,
    ext_to_column: Option<&ExtensionToColumnMap>,
    with_progress_bar: bool,
    benchmark_data: &mut Option<BenchmarkData>,
) -> Result<ColumnToLinesMap, git2::Error> {
    let blobs = repo.get_blobs_in_commit(commit)?;

    // Setup progress bar
    let mut progress_bar = if with_progress_bar {
        Some(ProgressBar::setup(
            blobs.len(),
            date.expect("present if progress bar"),
        ))
    } else {
        None
    };

    // Loop through all blobs in the commit tree
    let mut column_to_lines: ColumnToLinesMap = HashMap::new();
    for (index, blob) in blobs.iter().enumerate() {
        // Update progress bar if present
        if let Some(progress_bar) = &mut progress_bar {
            progress_bar.set_position_rate_limited(index);
        }

        // Figure out if we should count the lines for the file extension this
        // blob has, by figuring out what column the lines should be added to,
        // if any
        let column = match ext_to_column {
            Some(ext_to_column) => ext_to_column.get(&blob.ext),

            // If no specific columns are requested, we are probably invoked
            // with --list, so count the lines for all extensions
            None => Some(&blob.ext),
        };

        // If the blob has an extension we care about, count the lines!
        if let Some(column) = column {
            let lines = repo.get_lines_in_blob(&blob.id)?;
            let total_lines = column_to_lines.entry(column.to_owned()).or_insert(0);
            *total_lines += lines;

            // If we are benchmarking, now is the time to update that data
            if let Some(benchmark_data) = benchmark_data {
                benchmark_data.total_files_processed += 1;
                benchmark_data.total_lines_counted += lines;
            }
        }
    }

    // Clear progress bar if present
    if let Some(progress_bar) = &progress_bar {
        progress_bar.finish_and_clear();
    }

    Ok(column_to_lines)
}

fn enough_days_passed(
    args: &Args,
    date_of_last_row: Option<DateTime<Utc>>,
    current_date: DateTime<Utc>,
) -> bool {
    match date_of_last_row {
        Some(date_of_last_row) => {
            let time_passed = date_of_last_row.signed_duration_since(current_date);

            // NOTE: Takes hour of the day into account; day (%d) of date can be
            // different without a full day having passed
            time_passed.num_days() >= args.min_interval as i64
        }
        None => true,
    }
}

/// Generates a map that taks a list such as [".c+.h", ".xml"] and creates a map
/// that looks like this:
///
/// { ".c": ".c+.h", ".h": ".c+.h", ".xml": ".xml" }
///
/// It is used so that e.g. .c and .h can be summed together. Typically, if you
/// e.g. count lines for the C language, you want to count both .c and .h files
/// together.
fn generate_extension_to_column_map(raw_extensions: &[String]) -> ExtensionToColumnMap {
    let mut map: ExtensionToColumnMap = ExtensionToColumnMap::new();
    for raw_extension in raw_extensions {
        for ext in raw_extension.split('+') {
            map.insert(String::from(ext), String::from(raw_extension));
        }
    }
    map
}

fn get_data_for_start_commit(repo: &Repo, args: &Args) -> Result<ColumnToLinesMap, git2::Error> {
    let commit = repo
        .repo
        .revparse_single(&args.start_commit)?
        .peel_to_commit()?;
    process_commit(repo, None, &commit, None, false, &mut None)
}

fn get_reasonable_set_of_columns(repo: &Repo, args: &Args) -> Result<Vec<Column>, git2::Error> {
    Ok(if !args.columns.is_empty() {
        // Easy, just use what the user wishes
        args.columns.clone()
    } else {
        // Calculate a reasonable set of extension to count lines for using the
        // file extensions present in the first commit
        let data = get_data_for_start_commit(repo, args)?;
        let top_three_extensions = utils::get_top_three_extensions(&data);
        let top_three_str = top_three_extensions.join(" ");
        eprintln!(
            "\
INFO: Continuing as if run with the arguments {} like this:

    git-repo-language-trends {}

You can manually pass other file extensions as arguments if you want other data.

",
            top_three_str, top_three_str,
        );
        top_three_extensions
    })
}

fn main() {
    let version = option_env!("PROJECT_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"));
    let args = Args::from_clap(&Args::clap().version(version).get_matches());
    match run(&args) {
        Ok(()) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}

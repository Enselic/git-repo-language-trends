use chrono::DateTime;
use chrono::Utc;

use std::collections::HashMap;
use structopt::StructOpt;

mod benchmark;
use benchmark::BenchmarkData;

mod output;
pub use output::Output;

mod tsv_output;
use tsv_output::TabSeparatedValuesOutput;

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
    cd ~/src/any-git-repository                # Go to any git repository
    git-repo-language-trend  .m+.h  .swift     # Objective-C vs Swift (with .m and .h files summed together)
    git-repo-language-trend  .java  .kt        # Java vs Kotlin
")]
pub struct Args {
    /// For what file extensions lines will be counted. Can be specified
    /// multiple times. Use '.ext' for regular line counting. Use '.ext1+.ext2'
    /// syntax for auto-summation into a single column. If you specify no file
    /// extensions, the top three extensions in the repository will be used,
    /// based on the number of lines in files with the extensions.
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

// Crate convenience
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn run(args: &Args) -> Result<()> {
    let repo = Repo::from_path(std::env::var("GIT_DIR").unwrap_or_else(|_| ".".to_owned()))?;

    let mut benchmark_data = BenchmarkData::start_if_activated(args);

    if args.list {
        list_file_extensions(&repo, &args, &mut benchmark_data)?;
    } else {
        process_commits_and_print_rows(&repo, &args, &mut benchmark_data)?;
    }

    if let Some(benchmark_data) = benchmark_data {
        benchmark_data.report();
    }

    Ok(())
}

fn list_file_extensions(
    repo: &Repo,
    args: &Args,
    benchmark_data: &mut Option<BenchmarkData>,
) -> Result<()> {
    let data = get_data_for_start_commit(&repo, &args, benchmark_data)?;
    println!(
        "Available extensions (in first commit):\n{}",
        utils::get_extensions_sorted_by_popularity(&data).join(" ")
    );

    Ok(())
}

fn process_commits_and_print_rows(
    repo: &Repo,
    args: &Args,
    benchmark_data: &mut Option<BenchmarkData>,
) -> Result<()> {
    let columns = get_reasonable_set_of_columns(&repo, &args, benchmark_data)?;
    if columns.is_empty() {
        eprintln!("Could not find any file extensions, try specifying them manually");
        return Ok(());
    }
    let ext_to_column = generate_extension_to_column_map(&columns);

    let mut stdout = TabSeparatedValuesOutput::new(std::io::stdout());
    let mut outputs: Vec<&mut dyn Output> = vec![&mut stdout];

    // Print column headers
    for output in &mut outputs {
        output.start(&columns)?;
    }

    // Print rows
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
                &ext_to_column,
                &mut outputs,
                benchmark_data,
                &args,
            )?;
            rows_left -= 1;
        }
    }

    // Give outputs a chance to wrap things up
    for output in &mut outputs {
        output.finish()?;
    }

    Ok(())
}

/// Counts lines for files with the given file extensions in a given commit, and
/// then prints the result in one row of tabulated data.
fn process_and_print_row(
    repo: &Repo,
    date: &str,
    commit: &git2::Commit,
    ext_to_column: &ExtensionToColumnMap,
    outputs: &mut [&mut dyn Output],
    benchmark_data: &mut Option<BenchmarkData>,
    args: &Args,
) -> Result<()> {
    let data = process_commit(
        repo,
        commit,
        Some(ext_to_column),
        args,
        date,
        benchmark_data,
    )?;

    for output in outputs {
        output.add_row(date, &data)?;
    }

    Ok(())
}

/// Counts lines for files with the given file extensions in a given commit.
/// Shows a progress bar on stderr if stderr is a tty.
fn process_commit(
    repo: &Repo,
    commit: &git2::Commit,
    ext_to_column: Option<&ExtensionToColumnMap>,
    args: &Args,
    progress_bar_prefix: &str,
    benchmark_data: &mut Option<BenchmarkData>,
) -> Result<ColumnToLinesMap> {
    let blobs = repo.get_blobs_in_commit(commit)?;

    // Setup progress bar
    let mut progress_bar = if !args.disable_progress_bar {
        Some(ProgressBar::setup(blobs.len(), progress_bar_prefix))
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

/// Checks if enough days according to --min-interval has passed, i.e. if it is
/// time to process and print another commit.
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
fn generate_extension_to_column_map(columns: &[String]) -> ExtensionToColumnMap {
    let mut map: ExtensionToColumnMap = ExtensionToColumnMap::new();
    for column in columns {
        for ext in column.split('+') {
            map.insert(String::from(ext), String::from(column));
        }
    }
    map
}

/// Calls process_commit for the first commit (possibly from --start-commit)
fn get_data_for_start_commit(
    repo: &Repo,
    args: &Args,
    benchmark_data: &mut Option<BenchmarkData>,
) -> Result<ColumnToLinesMap> {
    let commit = repo
        .repo
        .revparse_single(&args.start_commit)?
        .peel_to_commit()?;
    process_commit(
        repo,
        &commit,
        None,
        args,
        "finding extensions and their line count",
        benchmark_data,
    )
}

/// Figure out for what extensions to calculate lines for.
fn get_reasonable_set_of_columns(
    repo: &Repo,
    args: &Args,
    benchmark_data: &mut Option<BenchmarkData>,
) -> Result<Vec<Column>> {
    Ok(if !args.columns.is_empty() {
        // Easy, just use what the user wishes
        args.columns.clone()
    } else {
        // Calculate a reasonable set of extension to count lines for using the
        // file extensions present in the first commit
        let data = get_data_for_start_commit(repo, args, benchmark_data)?;
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
    // To include git SHA1 of the build in --version
    let version = option_env!("PROJECT_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"));

    let args = Args::from_clap(&Args::clap().version(version).get_matches());
    match run(&args) {
        Ok(()) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}

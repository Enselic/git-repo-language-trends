// Features missing:
//  * print progress bar for large projects such as the Linux kernel
//  * Auto-detect extensions using first commit
//  * Auto-convert file extension to name, e.g. .rs <-> Rust
//  * get rid of dependence of git binary by using git2-rs instead of git log
//  * output a graph by default with https://crates.io/crates/plotters

use chrono::NaiveDate;
use std::collections::HashMap;
use std::process;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "\
Prints tabulated data about programming language usage over time in a git repository
for a given set of file extensions.

Copy-paste the output into e.g. Google Sheets or Microsoft Excel to easily make a graph.
Stacked area chart is recommended.

EXAMPLES
    git-repo-language-trend .cpp  .rs             # C++ vs Rust
    git-repo-language-trend .java .kt             # Java vs Kotlin
    git-repo-language-trend .m    .swift          # Objective-C vs Swift
")]
struct Args {
    /// Optional. The mimimum interval in days between data points.
    #[structopt(long, default_value = "7")]
    interval: u32,

    /// Optional. Maximum number of data rows to print.
    #[structopt(long, default_value = "18446744073709551615")]
    max_rows: u64,

    /// Optional. The commit to start parsing from.
    #[structopt(long, default_value = "HEAD")]
    start_commit: String,

    /// Prints total counted lines/second.
    #[structopt(long)]
    benchmark: bool,

    // Prints debug information during processing.
    #[structopt(long)]
    debug: bool,

    /// (Advanced.) By default, --first-parent is passed to the internal git log
    /// command. This ensures that the data in each row comes from a source code
    /// tree that is an ancestor to the row above it. If you prefer data for as
    /// many commits as possible, even though the data can become "jumpy",
    /// enable this flag.
    #[structopt(long)]
    all_parents: bool,

    #[structopt(name = "EXT1", required = true)]
    file_extensions: Vec<String>,
}

struct PerformanceData {
    start_time: std::time::Instant,
    total_lines_counted: usize,
    total_files_processed: usize,
}

fn run(args: &Args) -> Result<(), git2::Error> {
    let extensions: Vec<&str> = args
        .file_extensions
        .iter()
        .map(std::ops::Deref::deref)
        .collect();

    let repo = git2::Repository::open(std::env::var("GIT_DIR").unwrap_or_else(|_| ".".to_owned()))?;

    // Print rows
    // Use --no-merges --first-parent to get a continous history
    // Otherwise there can be confusing bumps in the graph
    // git log is much easier than libgit2, and the top level loop
    // is not performance critical, so use a plain git log child process
    let parent_flag = if args.all_parents {
        ""
    } else {
        "--first-parent"
    };
    let date_fmt = "%Y-%m-%d";
    let git_log = format!(
        "git log --format=%cd:%h --date=format:{date_fmt} --no-merges {parent_flag} {start_commit}",
        date_fmt = date_fmt,
        parent_flag = parent_flag,
        start_commit = args.start_commit,
    );

    let mut performance_data = if args.benchmark {
        Some(PerformanceData {
            start_time: std::time::Instant::now(),
            total_lines_counted: 0,
            total_files_processed: 0,
        })
    } else {
        None
    };

    let mut headers_printed = false;
    let mut rows_left = args.max_rows;
    let mut date_of_last_row: Option<NaiveDate> = None;
    for row in command_stdout_as_lines(git_log) {
        if rows_left == 0 {
            break;
        }

        let mut split = row.split(':'); // e.g. "2021-01-14:979f8d74e9"
        let date = split.next().unwrap(); // e.g. "2021-01-14"
        let commit = split.next().unwrap(); // e.g. "979f8d74e9"

        if !headers_printed {
            for _ in date.chars() {
                print!(" ");
            }
            for ext in &extensions {
                print!("\t{}", ext);
            }
            println!();
            headers_printed = true;
        }

        if args.debug {
            eprint!("-> Looking at {} {} ...", commit, date);
        }

        let current_date = NaiveDate::parse_from_str(date, date_fmt).expect("parsing");
        if match date_of_last_row {
            Some(date_of_last_row) => {
                let days_passed = date_of_last_row
                    .signed_duration_since(current_date)
                    .num_days();
                if args.debug {
                    eprintln!(" made {} days after last printed one", days_passed);
                }
                days_passed >= args.interval as i64
            }
            None => {
                if args.debug {
                    eprintln!(" first printed row");
                }
                true
            }
        } {
            // TODO: Keep going if one fails?
            process_and_print_row(&repo, date, commit, &extensions, &mut performance_data)?;
            date_of_last_row = Some(current_date);
            rows_left -= 1;
        }
    }

    if let Some(performance_data) = performance_data {
        let end_time = std::time::Instant::now();
        let duration = end_time - performance_data.start_time;
        let seconds = duration.as_secs_f64();
        let lines_per_second = performance_data.total_lines_counted as f64 / seconds;
        let files_per_second = performance_data.total_files_processed as f64 / seconds;
        let lines_per_file = performance_data.total_lines_counted as f64
            / performance_data.total_files_processed as f64;
        println!(
            "Counted {} lines in {} files in {:.3} seconds. On average:
            {} lines/second
            {} files/second
            {} lines/file",
            performance_data.total_lines_counted,
            performance_data.total_files_processed,
            seconds,
            lines_per_second.floor(),
            files_per_second.floor(),
            lines_per_file.floor(),
        );
    }

    Ok(())
}

fn process_and_print_row(
    repo: &git2::Repository,
    date: &str,
    commit: &str,
    extensions: &[&str],
    performance_data: &mut Option<PerformanceData>,
) -> Result<(), git2::Error> {
    let data = process_commit(repo, commit, extensions, performance_data)?;
    print!("{}", date);
    for ext in extensions {
        print!("\t{}", data.get(ext).unwrap_or(&0));
    }
    println!();

    Ok(())
}

fn process_commit<'a>(
    repo: &git2::Repository,
    commit: &str,
    extensions: &'a [&str],
    performance_data: &mut Option<PerformanceData>,
) -> Result<HashMap<&'a str, usize>, git2::Error> {
    let mut ext_to_total_lines = HashMap::new();

    let commito = repo.revparse_single(commit)?;
    let treeo = commito.peel(git2::ObjectType::Tree)?;
    let tree = treeo
        .as_tree()
        .ok_or_else(|| git2::Error::from_str("tree not a tree"))?;
    tree.walk(git2::TreeWalkMode::PostOrder, |_, entry| {
        if Some(git2::ObjectType::Blob) == entry.kind() {
            if let Some(entry_extension) = extension_for_raw_name(entry.name_bytes()) {
                for extension in extensions {
                    if *extension == entry_extension {
                        if let Ok(lines) = get_lines_in_blob(repo, &entry.id()) {
                            let total_lines = ext_to_total_lines.entry(*extension).or_insert(0);
                            *total_lines += lines;

                            if let Some(performance_data) = performance_data {
                                performance_data.total_files_processed += 1;
                                performance_data.total_lines_counted += lines;
                            }
                        } else {
                            // TODO: Propagate error
                        }
                    }
                }
            }
        }

        git2::TreeWalkResult::Ok
    })?;

    Ok(ext_to_total_lines)
}

fn get_lines_in_blob(repo: &git2::Repository, blobid: &git2::Oid) -> Result<usize, git2::Error> {
    let blobo = repo.find_object(*blobid, Some(git2::ObjectType::Blob))?;
    let blob = blobo
        .as_blob()
        .ok_or_else(|| git2::Error::from_str("the blob was not a blob, hmm"))?;
    let content = blob.content();
    let mut lines = 0;
    for b in content {
        if *b == b'\n' {
            lines += 1;
        }
    }
    Ok(lines)
}

fn extension_for_raw_name(raw_name: &[u8]) -> Option<&str> {
    match raw_name.iter().rposition(|u| *u == b'.') {
        Some(dot_index) => {
            let raw_ext = &raw_name[dot_index..];
            std::str::from_utf8(raw_ext).ok()
        }
        None => None,
    }
}

fn command_stdout_as_lines<T: AsRef<str>>(command: T) -> Vec<String> {
    let stdout = command_stdout(command);
    String::from_utf8(stdout)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn command_stdout<T: AsRef<str>>(command: T) -> Vec<u8> {
    let mut args = command.as_ref().split_ascii_whitespace();

    process::Command::new(args.next().unwrap())
        .args(args)
        .stderr(process::Stdio::inherit())
        .output()
        .unwrap()
        .stdout
}

fn main() {
    let args = Args::from_args();
    match run(&args) {
        Ok(()) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}

use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::path;
use std::process;

fn main() {
    let mut args = env::args();
    let bin_path = args.next().unwrap();
    let bin = bin_path.rsplit(path::MAIN_SEPARATOR).next().unwrap();
    let extensions: Vec<String> = args.collect();
    if extensions.is_empty()
        || extensions.contains(&"-h".to_owned())
        || extensions.contains(&"--help".to_owned())
    {
        println!(
            "\
Prints tabulated data about programming language usage over time in a git repository
for a given set of file extensions. The data points are on a week-by-week basis.

Copy-paste the output into e.g. Google Sheets or Microsoft Excel to easily make a graph.
Stacked area chart is recommended.

USAGE
    {} EXT1 EXT2 EXT3 ...

EXAMPLES
    {} java kt             # Java vs Kotlin
    {} m swift             # Objective-C vs Swift
    {} cpp rs              # C++ vs Rust
",
            bin, bin, bin, bin
        );
        process::exit(1);
    }

    // Print column headers
    for ext in &extensions {
        print!("\t{}", ext);
    }
    println!();

    // Print rows
    let mut analyzed_weeks = HashSet::new();
    // Use --no-merges --first-parent to get a continous history
    // Otherwise there can be confusing bumps in the graph
    let git_log = "git log --format=%cd:%h --date=format:%Yw%U --no-merges --first-parent";
    for row in command_stdout_as_lines(git_log) {
        let mut split = row.split(':');
        let week = split.next().unwrap(); // Year and week, e.g. "2021w02"
        let commit = split.next().unwrap(); // Commit, e.g. "979f8d7"

        if !analyzed_weeks.contains(week) {
            analyzed_weeks.insert(week.to_owned());

            print_row(week, commit, &extensions)
        };
    }
}

fn print_row(year_and_week: &str, commit: &str, extensions: &[String]) {
    let data = from_commit(commit, extensions);
    print!("{}", year_and_week);
    for ext in extensions {
        print!("\t{}", data.get(ext).unwrap_or(&0));
    }
    println!();
}

fn from_commit(commit: &str, extensions: &[String]) -> HashMap<String, usize> {
    let mut ext_to_total_lines = HashMap::new();

    for file in command_stdout_as_lines(format!("git ls-tree --name-only -r {}", commit)) {
        if let Some(extension) = file.rsplitn(2, '.').next() {
            if !extensions.contains(&extension.to_owned()) {
                continue;
            }

            let lines = command_stdout_line_count(format!("git show {}:{}", commit, file));
            let total_lines = ext_to_total_lines.entry(extension.to_owned()).or_insert(0);
            *total_lines += lines;
        }
    }

    ext_to_total_lines
}

fn command_stdout_as_lines<T: AsRef<str>>(command: T) -> Vec<String> {
    let stdout = command_stdout(command);
    String::from_utf8(stdout).unwrap().lines().map(String::from).collect()
}

fn command_stdout_line_count<T: AsRef<str>>(command: T) -> usize {
    let stdout = command_stdout(command);
    stdout.into_iter().filter(|c| *c == b'\n').count()
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

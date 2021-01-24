use std::collections::HashMap;
use std::collections::HashSet;
use std::process;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "\
Prints tabulated data about programming language usage over time in a git repository
for a given set of file extensions. The data points are on a week-by-week basis.

Copy-paste the output into e.g. Google Sheets or Microsoft Excel to easily make a graph.
Stacked area chart is recommended.

EXAMPLES
    git-repo-language-trend .cpp  .rs             # C++ vs Rust
    git-repo-language-trend .java .kt             # Java vs Kotlin
    git-repo-language-trend .m    .swift          # Objective-C vs Swift
")]
struct Args {
    #[structopt(default_value = "", long, help = "Optional. The commit to start parsing from.")]
    start_commit: String,

    #[structopt(name = "EXT1", required = true)]
    file_extensions: Vec<String>,
}

fn run(args: &Args) -> Result<(), git2::Error> {
    let extensions: Vec<&str> = args
        .file_extensions
        .iter()
        .map(std::ops::Deref::deref)
        .collect();

    let repo = git2::Repository::open(".")?;

    // Print column headers
    for ext in &extensions {
        print!("\t{}", ext);
    }
    println!();

    // Print rows
    let mut analyzed_weeks = HashSet::new();
    // Use --no-merges --first-parent to get a continous history
    // Otherwise there can be confusing bumps in the graph
    // git log is much easier than libgit2, and the top level loop
    // is not performance critical, so use a plain git log child process
    let git_log = format!(
        "git log --format=%cd:%h --date=format:%Yw%U --no-merges --first-parent {}",
        args.start_commit
    );
    for row in command_stdout_as_lines(git_log) {
        let mut split = row.split(':');
        let week = split.next().unwrap(); // Year and week, e.g. "2021w02"
        let commit = split.next().unwrap(); // Commit, e.g. "979f8d74e9"

        // TODO: Use days and parse weeks instead
        if !analyzed_weeks.contains(week) {
            analyzed_weeks.insert(week.to_owned());

            // TODO: Keep going if one fails?
            process_and_print_row(&repo, week, commit, &extensions)?;
        };
    }

    // TODO: Output simple graphs in addition to tabulated data
    Ok(())
}

fn process_and_print_row(
    repo: &git2::Repository,
    week: &str,
    commit: &str,
    extensions: &[&str],
) -> Result<(), git2::Error> {
    let data = process_commit(repo, commit, extensions)?;
    print!("{}", week);
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
        Err(e) => eprintln!("error: {}", e),
    }
}

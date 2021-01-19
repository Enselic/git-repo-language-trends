use std::collections::HashMap;

fn command_stdout<T: AsRef<str>>(command: T) -> Vec<u8> {
    let mut args = command.as_ref().split_ascii_whitespace();

    std::process::Command::new(args.next().unwrap())
        .args(args)
        .stderr(std::process::Stdio::inherit())
        .output()
        .unwrap()
        .stdout
}

fn command_stdout_as_lines<T: AsRef<str>>(command: T) -> Vec<String> {
    let stdout = String::from_utf8(command_stdout(command)).expect("string");
    stdout.lines().map(|line| String::from(line)).collect()
}

fn command_stdout_line_count<T: AsRef<str>>(command: T) -> usize {
    let stdout = command_stdout(command);
    stdout.into_iter().filter(|c| *c == b'\n').count()
}

struct LangData {
    ext_to_lines: HashMap<String, usize>,
}

impl LangData {
    fn from_commit(sha1: &str) -> LangData {
        use std::path::Path;

        let mut ext_to_lines: HashMap<String, usize> = HashMap::new();

        for file in command_stdout_as_lines(format!("git ls-tree --name-only -r {}", sha1)) {
            if let Some(ext) = Path::new(&file).extension() {

                if extensions.contains(&OsString::from(ext)) {
                    let line_count =
                        command_stdout_line_count(format!("git show {}:{}", commit, &file));
                    let current = result.entry(&ext).or_insert(0);
                    *current += line_count;
                    //println!("File in {} called {} has {} lines",commit, file, line_count);
                }
            }
        }
    
        for key in result {
            println!("Ext {:?} has {} lines", key.0, key.1);
        }    
    }
}

fn main() {
    use std::str;

    let mut data: HashMap<String, LangData> = HashMap::new();
    for commit in command_stdout_as_lines("git log --no-merges --first-parent --format='%H %cd' --date=format:'%Y-%m'") {
        let foo = commit.split(" ");
        let sha1 = foo.0;
        let year_month = foo.1;

        if (!data.contains_key(year_month)) {
            data.put(year_month, get_data_for_commit(sha1));
        }
    }
}

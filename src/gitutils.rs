use std::path::Path;
use std::process;

pub struct Repo {
    repo: git2::Repository,
}

impl Repo {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Repo, git2::Error> {
        let repo = git2::Repository::open(path)?;
        Ok(Repo { repo })
    }

    pub fn git_log(&self, args: &super::Args) -> Vec<(String, String)> {
        // git log is much easier than libgit2, and the top level loop
        // is not performance critical, so use a plain git log child process

        let parent_flag = if !args.all_parents {
            "--first-parent"
        } else {
            ""
        };

        let git_log = format!(
            "git log --format=%cd:%h --date=format:%Y-%m-%d --no-merges {parent_flag} {start_commit}",
            parent_flag = parent_flag,
            start_commit = args.start_commit,
        );

        command_stdout_as_lines(git_log)
            .into_iter()
            .map(|line| {
                let mut split = line.split(':'); // e.g. "2021-01-14:979f8d74e9"
                let date = split.next().unwrap(); // e.g. "2021-01-14"
                let commit = split.next().unwrap(); // e.g. "979f8d74e9"
                (date.to_owned(), commit.to_owned())
            })
            .collect()
    }

    pub fn get_blobs_in_commit(
        &self,
        commit: &str,
    ) -> Result<Vec<(git2::Oid, String)>, git2::Error> {
        let commito = self.repo.revparse_single(commit)?;
        let treeo = commito.peel(git2::ObjectType::Tree)?;
        let tree = treeo
            .as_tree()
            .ok_or_else(|| git2::Error::from_str("tree not a tree"))?;
        self.get_blobs_in_tree(&tree)
    }

    fn get_blobs_in_tree(
        &self,
        tree: &git2::Tree,
    ) -> Result<Vec<(git2::Oid, String)>, git2::Error> {
        let mut blobs = vec![];
        tree.walk(git2::TreeWalkMode::PostOrder, |_, entry| {
            if Some(git2::ObjectType::Blob) == entry.kind() {
                if let Some(ext) = extension_for_raw_name(entry.name_bytes()) {
                    blobs.push((entry.id(), ext.to_owned()));
                }
            }
            git2::TreeWalkResult::Ok
        })?;
        Ok(blobs)
    }

    pub fn get_lines_in_blob(&self, blobid: &git2::Oid) -> Result<usize, git2::Error> {
        let blobo = self
            .repo
            .find_object(*blobid, Some(git2::ObjectType::Blob))?;
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

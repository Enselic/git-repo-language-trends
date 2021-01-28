use chrono::DateTime;
use chrono::TimeZone;
use chrono::Utc;
use std::path::Path;

pub struct Repo {
    pub repo: git2::Repository,
}

impl Repo {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Repo, git2::Error> {
        let repo = git2::Repository::open(path)?;
        Ok(Repo { repo })
    }

    pub fn git_log(
        &self,
        args: &super::Args,
    ) -> Result<Vec<(DateTime<Utc>, git2::Commit)>, git2::Error> {
        let mut revwalk: git2::Revwalk = self.repo.revwalk()?;
        let rev = self.repo.revparse_single(&args.start_commit)?;
        revwalk.push(rev.id())?;

        if !args.all_parents {
            revwalk.simplify_first_parent()?;
        }

        Ok(revwalk
            .into_iter()
            .filter_map(|item| {
                let commit: git2::Commit = self.repo.find_commit(item.unwrap()).unwrap();
                if commit.parent_count() > 1 {
                    None // ignore merge commits
                } else {
                    let commit_time = commit.committer().when().seconds();
                    let ts = chrono::Utc.timestamp(commit_time, 0);

                    Some((ts, commit))
                }
            })
            .collect())
    }

    pub fn get_blobs_in_commit(
        &self,
        commit: &git2::Commit,
    ) -> Result<Vec<(git2::Oid, String)>, git2::Error> {
        let tree = commit.tree()?;
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

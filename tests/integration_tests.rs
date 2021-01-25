use assert_cmd::Command;

fn git_repo_language_trends_bin() -> Command {
    Command::cargo_bin("git-repo-language-trends").unwrap()
}

#[test]
fn own_git_repo_0_day_interval() {
    git_repo_language_trends_bin()
        .arg("--interval=0")
        .arg("--start-commit")
        .arg("v0.1.2")
        .arg(".yml")
        .arg(".rs")
        .assert()
        .success()
        .stdout(
            "          	.yml	.rs
2021-01-23	66	121
2021-01-23	67	121
2021-01-23	78	121
2021-01-23	57	121
2021-01-23	22	121
2021-01-23	0	121
2021-01-23	0	121
2021-01-23	0	107
2021-01-23	0	107
2021-01-22	0	107
2021-01-19	0	66
2021-01-19	0	0
2021-01-19	0	0
",
        )
        .stderr("");
}

#[test]
fn own_git_repo_1_day_interval() {
    git_repo_language_trends_bin()
        .arg("--interval=1")
        .arg("--start-commit=v0.2.0")
        .arg(".rs")
        .arg(".a")
        .assert()
        .success()
        .stdout(
            "          	.rs	.a
2021-01-24	196	4
2021-01-23	121	0
2021-01-22	107	0
2021-01-19	66	0
",
        )
        .stderr("");
}

#[test]
fn own_git_repo_7_day_interval() {
    git_repo_language_trends_bin()
        .arg("--interval=7")
        .arg("--start-commit=v0.2.0")
        .arg(".rs")
        .arg(".a")
        .assert()
        .success()
        .stdout(
            "          	.rs	.a
2021-01-24	196	4
",
        )
        .stderr("");
}

#[test]
fn negative_interval() {
    git_repo_language_trends_bin()
        .arg("--interval")
        .arg("-1")
        .arg(".rs")
        .assert()
        .failure()
        .stdout("")
        .stderr(predicates::str::contains(
            "Found argument '-1' which wasn't expected",
        ));
}

/// Regression test for a bug where the "last printed row date" was updated for
/// every commit, and not only printed commits. This resulted in not printing
/// commits that were part of a long stream of regular commits each day, even if
/// the that stream of commits went on for longer than the current --interval.
#[test]
fn interval_calculated_for_last_printed_commit_only() {
    git_repo_language_trends_bin()
        .arg("--interval=2")
        .arg("--start-commit=v0.2.0")
        .arg(".rs")
        .assert()
        .success()
        .stdout(
            "          	.rs
2021-01-24	196
2021-01-22	107
2021-01-19	66
",
        )
        .stderr("");
}

#[test]
fn own_git_repo_max_rows_5() {
    git_repo_language_trends_bin()
        .arg("--interval=0")
        .arg("--max-rows=5")
        .arg("--start-commit=v0.1.2")
        .arg(".yml")
        .arg(".rs")
        .assert()
        .success()
        .stdout(
            "          	.yml	.rs
2021-01-23	66	121
2021-01-23	67	121
2021-01-23	78	121
2021-01-23	57	121
2021-01-23	22	121
",
        )
        .stderr("");
}

#[test]
fn own_git_repo_max_rows_0() {
    git_repo_language_trends_bin()
        .arg("--max-rows=0")
        .arg("--start-commit=v0.1.2")
        .arg(".yml")
        .arg(".rs")
        .assert()
        .success()
        .stdout("")
        .stderr("");
}

#[test]
fn benchmark() {
    git_repo_language_trends_bin()
        .arg("--benchmark")
        .arg("--interval=0")
        .arg(".yml")
        .assert()
        .success()
        .stdout(predicates::str::contains("lines/second"))
        .stderr("");
}

#[test]
fn all_parents() {
    git_repo_language_trends_bin()
        .arg("--all-parents")
        .arg("--interval=0")
        .arg("--max-rows=10")
        .arg("--start-commit=v0.2.0")
        .arg(".rs")
        .assert()
        .success()
        .stdout(
            "          	.rs
2021-01-24	196
2021-01-24	196
2021-01-24	196
2021-01-24	196
2021-01-24	192
2021-01-24	192
2021-01-24	192
2021-01-24	185
2021-01-24	166
2021-01-24	172
",
        )
        .stderr("");
}

use assert_cmd::Command;

#[test]
fn own_git_repo_0_day_interval() {
    Command::cargo_bin("git-repo-language-trends")
        .unwrap()
        .arg("--interval")
        .arg("0")
        .arg("--start-commit")
        .arg("v0.1.2")
        .arg(".yml")
        .arg(".rs")
        .assert()
        .success()
        .stdout(
            "	.yml	.rs
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
    Command::cargo_bin("git-repo-language-trends")
        .unwrap()
        .arg("--interval")
        .arg("1")
        .arg("--start-commit")
        .arg("v0.2.0")
        .arg(".rs")
        .arg(".a")
        .assert()
        .success()
        .stdout(
            "	.rs	.a
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
    Command::cargo_bin("git-repo-language-trends")
        .unwrap()
        .arg("--interval")
        .arg("7")
        .arg("--start-commit")
        .arg("v0.2.0")
        .arg(".rs")
        .arg(".a")
        .assert()
        .success()
        .stdout(
            "	.rs	.a
2021-01-24	196	4
",
        )
        .stderr("");
}

#[test]
fn negative_interval() {
    Command::cargo_bin("git-repo-language-trends")
        .unwrap()
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

#[test]
fn own_git_repo_max_rows_5() {
    Command::cargo_bin("git-repo-language-trends")
        .unwrap()
        .arg("--interval")
        .arg("0")
        .arg("--max-rows")
        .arg("5")
        .arg("--start-commit")
        .arg("v0.1.2")
        .arg(".yml")
        .arg(".rs")
        .assert()
        .success()
        .stdout(
            "	.yml	.rs
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
    Command::cargo_bin("git-repo-language-trends")
        .unwrap()
        .arg("--max-rows")
        .arg("0")
        .arg("--start-commit")
        .arg("v0.1.2")
        .arg(".yml")
        .arg(".rs")
        .assert()
        .success()
        .stdout(
            "	.yml	.rs
",
        )
        .stderr("");
}

#[test]
fn benchmark() {
    Command::cargo_bin("git-repo-language-trends")
        .unwrap()
        .arg("--benchmark")
        .arg("--interval")
        .arg("0")
        .arg(".yml")
        .assert()
        .success()
        .stdout(predicates::str::contains("lines/second"))
        .stderr("");
}

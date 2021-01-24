use assert_cmd::Command;

#[test]
fn sanity_analysis_of_own_git_repo() {
    Command::cargo_bin("git-repo-language-trends")
        .unwrap()
        .arg("--start-commit")
        .arg("3340ee71f9")
        .arg(".rs")
        .arg(".a")
        .arg(".")
        .assert()
        .success()
        .stdout(
            "	.rs	.a	.
2021w04	185	4	3
2021w03	121	0	0
",
        )
        .stderr("");
}

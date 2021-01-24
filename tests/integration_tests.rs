use assert_cmd::Command;

#[test]
fn sanity_analysis_of_own_git_repo() {
    Command::cargo_bin("git-repo-language-trends")
        .unwrap()
        .arg("--start-commit")
        .arg("68b285abcc") // tag: v0.1.2
        .arg(".rs")
        .arg(".a")
        .assert()
        .success()
        .stdout(
            "	.rs	.a
2021-01-23	121	0
2021-01-22	107	0
2021-01-19	66	0
",
        )
        .stderr("");
}

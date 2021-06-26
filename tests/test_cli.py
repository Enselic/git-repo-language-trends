"""
Contains end-to-end tests. In other words, tests the CLI
without caring about implementation details.

We run the tests against our own git repo. It is worth noting
that initially this tool was implemented with rust, which
explains why we expect to see e.g. .rs lines in the tests.
The tests run against old git tags of the repo.
"""


from .utils import (
    s,
    run_git_repo_language_trends_output_test,
    run_git_repo_language_trends_test,
    run_git_repo_language_trends,
)

# This is OK to import as we do not consider it an implementation detail
from git_repo_language_trends import __version__

from pathlib import Path
import os


def test_own_git_repo_0_day_min_interval(tsv_output_path):
    run_git_repo_language_trends_output_test(tsv_output_path, [
        "--min-interval-days=0",
        "--first-commit=v0.1.2",
        ".yml",
        ".rs",
    ], """          	.yml	.rs
2021-01-19	0	0
2021-01-19	0	0
2021-01-19	0	66
2021-01-22	0	107
2021-01-23	0	107
2021-01-23	0	107
2021-01-23	0	121
2021-01-23	0	121
2021-01-23	22	121
2021-01-23	57	121
2021-01-23	78	121
2021-01-23	67	121
2021-01-23	66	121
""", f"""
Wrote output to file:

    {tsv_output_path}

""")


def test_own_git_repo_1_day_min_interval(tsv_output_path):
    run_git_repo_language_trends_output_test(tsv_output_path, [
        "--min-interval-days=1",
        "--first-commit=v0.3.0",
        ".rs",
        ".a",
    ], """          	.rs	.a
2021-01-19	66	0
2021-01-23	107	0
2021-01-24	196	4
2021-01-25	461	4
2021-01-27	602	4
""", f"""
Wrote output to file:

    {tsv_output_path}

""")


def test_own_git_repo_7_day_min_interval(tsv_output_path):
    run_git_repo_language_trends_output_test(tsv_output_path, [
        "--min-interval-days=7",
        "--first-commit=v0.2.0",
        ".rs",
        ".a",
    ], """          	.rs	.a
2021-01-24	196	4
""", f"""
Wrote output to file:

    {tsv_output_path}

""")


def test_negative_min_interval(tsv_output_path):
    result = run_git_repo_language_trends([
        "-o", tsv_output_path,
        "--output=something.tsv",
        "--min-interval-days=-1",
        ".rs",
    ])

    assert result.returncode == 2
    assert result.stdout == ""
    assert "Must not be negative" in result.stderr


def test_interval_calculated_for_last_printed_commit_only(tsv_output_path):
    """
    Regression test for a bug where the "last printed row date" was updated for
    every commit, and not only printed commits. This resulted in not printing
    commits that were part of a long stream of regular commits each day, even if
    the that stream of commits went on for longer than the current --min-interval-days.
    """

    run_git_repo_language_trends_output_test(tsv_output_path, [
        "--min-interval-days=2",
        "--first-commit=v0.3.0",
        ".rs",
    ], """          	.rs
2021-01-19	66
2021-01-24	196
2021-01-27	602
""", f"""
Wrote output to file:

    {tsv_output_path}

""")


def test_own_git_repo_max_commits_5(tsv_output_path):
    run_git_repo_language_trends_output_test(tsv_output_path, [
        "--min-interval-days=0",
        "--max-commits=5",
        "--first-commit=v0.1.2",
        ".yml",
        ".rs",
    ], """          	.yml	.rs
2021-01-23	22	121
2021-01-23	57	121
2021-01-23	78	121
2021-01-23	67	121
2021-01-23	66	121
""", f"""
Wrote output to file:

    {tsv_output_path}

""")


def test_own_git_repo_max_commits_5_relative(tsv_output_path):
    run_git_repo_language_trends_output_test(tsv_output_path, [
        "--min-interval-days=0",
        "--max-commits=5",
        "--first-commit=v0.1.2",
        "--relative",
        ".yml",
        ".rs",
    ], """          	.yml	.rs
2021-01-23	15.38	84.62
2021-01-23	32.02	67.98
2021-01-23	39.2	60.8
2021-01-23	35.64	64.36
2021-01-23	35.29	64.71
""", f"""
Wrote output to file:

    {tsv_output_path}

""")


def test_own_git_repo_max_commits_5_no_cache(tsv_output_path):
    run_git_repo_language_trends_output_test(tsv_output_path, [
        "--min-interval-days=0",
        "--max-commits=5",
        "--first-commit=v0.1.2",
        "--no-cache",
        ".yml",
        ".rs",
    ], """          	.yml	.rs
2021-01-23	22	121
2021-01-23	57	121
2021-01-23	78	121
2021-01-23	67	121
2021-01-23	66	121
""", f"""
Wrote output to file:

    {tsv_output_path}

""")


def test_own_git_repo_max_commits_0(tsv_output_path):
    run_git_repo_language_trends_output_test(tsv_output_path, [
        "-n=0",
        "--first-commit=v0.1.2",
        ".yml",
        ".rs",
    ], """          	.yml	.rs
""", f"""
Wrote output to file:

    {tsv_output_path}

""")


def test_all_parents(tsv_output_path):
    all_parents_test(tsv_output_path, "--all-parents")


def test_all_parents_short(tsv_output_path):
    all_parents_test(tsv_output_path, "-a")


def all_parents_test(tsv_output_path, option):
    run_git_repo_language_trends_output_test(tsv_output_path, [
        option,
        "--min-interval-days=0",
        "--max-commits=10",
        "--first-commit=v0.2.0",
        ".rs",
    ], """          	.rs
2021-01-24	166
2021-01-24	185
2021-01-24	192
2021-01-24	192
2021-01-24	192
2021-01-24	196
2021-01-24	196
2021-01-24	196
2021-01-24	196
2021-01-24	196
""", f"""
Wrote output to file:

    {tsv_output_path}

""")


def test_parent_directories_created(tmp_path):
    path_that_does_not_exist = str(tmp_path / "does-not-exist" / "really-not" / "foo.tsv")
    run_git_repo_language_trends_output_test(path_that_does_not_exist, [
        "--min-interval-days=0",
        "--max-commits=1",
        "--first-commit=v0.1.2",
        ".rs",
    ], """          	.rs
2021-01-23	121
""", f"""
Wrote output to file:

    {path_that_does_not_exist}

""")


def test_no_filter(tsv_output_path):
    run_git_repo_language_trends_output_test(tsv_output_path, [
        "--first-commit=v0.2.0",
        "--min-interval-days=2",
    ], """          	.rs	.yml	.md
2021-01-19	66	0	2
2021-01-24	196	68	40
""", f"""No file extensions specified, will use top three.
Top three extensions were: .rs .yml .md

Wrote output to file:

    {tsv_output_path}

""")


def test_no_filter_relative(tsv_output_path):
    run_git_repo_language_trends_output_test(tsv_output_path, [
        "--first-commit=v0.2.0",
        "--min-interval-days=2",
        "--relative",
    ], """          	.rs	.yml	.md
2021-01-19	97.06	0	2.94
2021-01-24	64.47	22.37	13.16
""", f"""No file extensions specified, will use top three.
Top three extensions were: .rs .yml .md

Wrote output to file:

    {tsv_output_path}

""")


def test_list():
    list_test("--list")


def test_list_short():
    list_test("-l")


def list_test(option):
    run_git_repo_language_trends_test([
        "--no-progress",
        option,
        "--first-commit=v0.3.0",
    ], """Available extensions in first commit:
.lock - 687 lines
.rs   - 602 lines
.yml  - 68 lines
.md   - 43 lines
.toml - 21 lines
.json - 20 lines
.a    - 4 lines
""")


def test_auto_sum(tsv_output_path):
    run_git_repo_language_trends_output_test(tsv_output_path, [
        "--first-commit=v0.2.0",
        "--min-interval-days=2",
        ".rs+.yml",
        ".md",
    ], """          	.rs+.yml	.md
2021-01-19	66	2
2021-01-24	264	40
""", f"""
Wrote output to file:

    {tsv_output_path}

""")


def test_auto_sum_csv(csv_output_path):
    run_git_repo_language_trends_output_test(csv_output_path, [
        "--first-commit=v0.2.0",
        "--min-interval-days=2",
        ".rs+.yml",
        ".md",
    ], """          ,.rs+.yml,.md
2021-01-19,66,2
2021-01-24,264,40
""", f"""
Wrote output to file:

    {csv_output_path}

""")


def test_version():
    run_git_repo_language_trends_test([
        "--version",
    ], f"""git-repo-language-trends {__version__}
""")


def test_invalid_file_format():
    run_git_repo_language_trends_test([
        "-o", "file.foo",
    ],
        "",
        """Output file format '.foo' not supported
""",
        1)


def test_custom_git_dir_path(tmp_path):
    """
    Test that GIT_DIR env var works to specify git repo to analyze
    """

    # We will analyze our own git repo. Remember where we are.
    our_git_repo = os.getcwd()

    # Then change to a dir without a git repo
    os.chdir(str(tmp_path))
    assert not Path(".git").exists()

    # Now invoke the tool, and use GIT_DIR
    output_name = "git_dir_test.csv"
    result = run_git_repo_language_trends(
        [
            "-o", output_name,
            "--no-progress",
            "--min-interval-days=7",
            "--first-commit=v0.2.0",
            ".rs",
            ".a",
        ],
        {
            "GIT_DIR": our_git_repo,
        },
    )

    result.check_returncode()
    assert result.stdout == s(f"""
Wrote output to file:

    {output_name}

""")
    assert result.stderr == ""

    # Read the file and assert
    actual = Path(output_name).read_text()
    expected = """          ,.rs,.a
2021-01-24,196,4
"""
    assert expected == actual

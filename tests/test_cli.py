import sys
import subprocess
import os

# This is OK to import as we do not consider it an implementation detail
from git_repo_language_trends import __version__


class Cli:

    def __init__(self, args):
        self.args = ["git-repo-language-trends"]
        self.args.extend(args)
        self.result = None

    def run(self):
        sresult = subprocess.run(self.args, capture_output=True)
        # Convert to strings for more readable assert messages
        sresult.stdout = str(sresult.stdout, "utf-8")
        sresult.stderr = str(sresult.stderr, "utf-8")
        print("In case this test fails, you might be interested in captured stdout:")
        print(sresult.stdout)
        print("and stderr:", file=sys.stderr)
        print(sresult.stderr, file=sys.stderr)
        return Result(sresult)


class Result:

    def __init__(self, result):
        self.result = result

    def assert_success(self):
        assert self.result.returncode == 0

    def assert_failure(self):
        assert self.result.returncode != 0

    def assert_stdout(self, stdout):
        assert self.result.stdout == stdout

    def assert_stderr(self, stderr):
        assert self.result.stderr == stderr


def git_repo_language_trends_bin(args):
    return Cli(args)


def test_own_git_repo_0_day_min_interval():
    result = git_repo_language_trends_bin([
        "--output=.tsv",
        "--min-interval-days=0",
        "--first-commit=v0.1.2",
        ".yml",
        ".rs",
    ]).run()

    result.assert_success()
    result.assert_stdout(s("""          	.yml	.rs
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
"""))


def test_own_git_repo_1_day_min_interval():
    result = git_repo_language_trends_bin([
        "--output=.tsv",
        "--min-interval-days=1",
        "--first-commit=v0.3.0",
        ".rs",
        ".a",
    ]).run()

    result.assert_success()
    result.assert_stdout(s("""          	.rs	.a
2021-01-19	66	0
2021-01-23	107	0
2021-01-24	196	4
2021-01-25	461	4
2021-01-27	602	4
"""))


def test_own_git_repo_7_day_min_interval():
    result = git_repo_language_trends_bin([
        "--output=.tsv",
        "--min-interval-days=7",
        "--first-commit=v0.2.0",
        ".rs",
        ".a",
    ]).run()

    result.assert_success()
    result.assert_stdout(s("""          	.rs	.a
2021-01-24	196	4
"""))


def test_negative_min_interval():
    result = git_repo_language_trends_bin([
        "--output=.tsv",
        "--min-interval-days",
        "-1",
        ".rs",
    ]).run()

    result.assert_failure()


# Regression test for a bug where the "last printed row date" was updated for
# every commit, and not only printed commits. This resulted in not printing
# commits that were part of a long stream of regular commits each day, even if
# the that stream of commits went on for longer than the current --min-interval-days.
def test_interval_calculated_for_last_printed_commit_only():
    result = git_repo_language_trends_bin([
        "--output=.tsv",
        "--min-interval-days=2",
        "--first-commit=v0.3.0",
        ".rs",
    ]).run()

    result.assert_success()
    result.assert_stdout(s("""          	.rs
2021-01-19	66
2021-01-24	196
2021-01-27	602
"""))


def test_own_git_repo_max_rows_5():
    result = git_repo_language_trends_bin([
        "--output=.tsv",
        "--min-interval-days=0",
        "--max-commits=5",
        "--first-commit=v0.1.2",
        ".yml",
        ".rs",
    ]).run()

    result.assert_success()
    result.assert_stdout(s("""          	.yml	.rs
2021-01-23	22	121
2021-01-23	57	121
2021-01-23	78	121
2021-01-23	67	121
2021-01-23	66	121
"""))


def test_own_git_repo_max_rows_0():
    result = git_repo_language_trends_bin([
        "--output=.tsv",
        "--max-commits=0",
        "--first-commit=v0.1.2",
        ".yml",
        ".rs",
    ]).run()

    result.assert_success()
    result.assert_stdout(s("""          	.yml	.rs
"""))


def test_all_parents():
    result = git_repo_language_trends_bin([
        "--output=.tsv",
        "--all-parents",
        "--min-interval-days=0",
        "--max-commits=10",
        "--first-commit=v0.2.0",
        ".rs",
    ]).run()

    result.assert_success()
    result.assert_stdout(s("""          	.rs
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
"""))


def test_no_filter():
    result = git_repo_language_trends_bin([
        "--output=.tsv",
        "--first-commit=v0.2.0",
        "--min-interval-days=2",
    ]).run()

    result.assert_success()
    result.assert_stdout(s("""          	.rs	.yml	.md
2021-01-19	66	0	2
2021-01-24	196	68	40
"""))
    # result.stderr(predicates:: str: : contains(
    #       "git-repo-language-trends .rs .yml .md",
    #  ));


def test_list():
    result = git_repo_language_trends_bin([
        "--output=.tsv",
        "--list",
        "--first-commit=v0.3.0",
    ]).run()

    result.assert_success()
    result.assert_stdout(s("""Available extensions (in first commit):
.lock .rs .yml .md .toml .json .a
"""))
    result.assert_stderr("")


def test_auto_sum():
    result = git_repo_language_trends_bin([
        "--output=.tsv",
        "--first-commit=v0.2.0",
        "--min-interval-days=2",
        ".rs+.yml",
        ".md",
    ]).run()

    result.assert_stdout(s("""          	.rs+.yml	.md
2021-01-19	66	2
2021-01-24	264	40
"""))


def test_version():
    result = git_repo_language_trends_bin([
        "--version",
    ]).run()

    result.assert_success()
    result.assert_stdout(s(f"""git-repo-language-trends {__version__}
"""))


# On Windows, command output will include CR.
# This wrapper makes sure string asserts works on all
# platforms.
def s(s):
    return s.replace('\n', os.linesep)

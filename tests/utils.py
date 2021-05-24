import sys
import os
import subprocess
from pathlib import Path


def run_git_repo_language_trends(args, env=None):
    """
    We must always inherit env, because git-repo-language-trends is not
    found without an inherited PATH
    """
    used_env = os.environ
    if env:
        used_env.update(env)

    used_args = ["git-repo-language-trends", *args]

    # Run the program, and say what command exactly, to make it
    # easy to manually reproduce any failures
    print("\n\nRunning this command:")
    print(" ".join(used_args) + "\n")
    result = subprocess.run(
        used_args,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        env=used_env,
    )

    # Convert to strings for more readable assert messages
    result.stdout = str(result.stdout, "utf-8")
    result.stderr = str(result.stderr, "utf-8")

    print("In case this test fails, you might be interested in captured stdout:")
    print(result.stdout)

    print("and stderr:", file=sys.stderr)
    print(result.stderr, file=sys.stderr)

    return result


def run_git_repo_language_trends_test(
    args,
    expected_stdout,
    expected_stderr="",
    expected_returncode=0,
):
    result = run_git_repo_language_trends(args)

    assert result.stdout == s(expected_stdout)

    assert result.stderr == s(expected_stderr)

    assert result.returncode == expected_returncode

    return result


def run_git_repo_language_trends_output_test(
    output_path,
    args,
    expected_output_content,
    expected_stdout,
    expected_stderr="",
    expected_returncode=0,
):
    output_args = ["-o", output_path] if output_path else []

    result = run_git_repo_language_trends_test(
        [
            "--no-progress",
            *output_args,
            *args,
        ],
        expected_stdout,
        expected_stderr,
        expected_returncode,
    )

    assert Path(output_path).read_text() == expected_output_content

    return result


def s(s):
    """
    On Windows, command output will include CR.
    This wrapper makes sure string asserts works on all
    platforms.
    """

    return s.replace('\n', os.linesep)

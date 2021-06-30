# git-repo-language-trends

Analyze programming language usage over time in a git repository and produce a
graphical or textual representation of the result.

Available output file formats:
* **.svg** - Scalable Vector Graphics
* **.png** - Portable Network Graphics
* **.csv** - Comma-separated values
* **.tsv** - Tab-separated values

# Examples

Showing the pace at which **Kotlin** is replacing **Java** in [AndroidX
`support`
library](https://android.googlesource.com/platform/frameworks/support/) by
language usage percentage:

    % cd ~/src/androidx
    % git-repo-language-trends --relative --max-commits 30 --min-interval-days 60  .kt .java
    % open androidx-language-trends.png

![AndroidX language trends](https://i.imgur.com/1B9cN1z.png)

Showing how the implementation of CPython has grown over the last decades in
terms of number of lines of C (.c and .h files) and Python (.py files):

    % cd ~/src/cpython
    % git-repo-language-trends --max-commits 30 --min-interval-days 365 .c+.h .py
    % open cpython-language-trends.png

![CPython language trends](https://i.imgur.com/Uv4mK1z.png)

Showing the pace at which **TypeScript** is replacing **JavaScript** in
[`mattermost-webapp`](https://github.com/mattermost/mattermost-webapp) by
language usage percentage:

    % cd ~/src/mattermost-webapp
    % git-repo-language-trends --min-interval-days 30 --max-commits 25 --relative .ts+.tsx .js+.jsx
    % open mattermost-webapp-language-trends.png

![mattermost-webapp language trends](https://i.imgur.com/6IGbgjb.png)

Showing line count trends for **Swift**, **Objective-C** (.m and .h files summed
together), and **Markdown** (.md) in the [iOS `Charts`
library](https://github.com/danielgindi/Charts):

    % cd ~/src/Charts
    % git-repo-language-trends  --min-interval-days 60 .swift .m+.h .md
    % open Charts-language-trends.png

![iOS Charts language trends](https://i.imgur.com/TVOiSlV.png)

Same data as above, but as a comma-separated values (.csv) file, in case you
want to generate plots yourself from the raw data, and limited to 10 commits:

    % cd ~/src/Charts
    % git-repo-language-trends  --max-commits 10 --min-interval-days 60 .swift .m+.h .md -o charts-language-trends.csv
    % cat charts-language-trends.csv
            ,.swift,.m+.h,.md
    2019-03-20,32163,6205,3785
    2019-06-10,32347,6205,3859
    2019-09-09,32368,6205,3859
    2019-11-13,32746,6205,4007
    2020-02-13,32687,6205,4015
    2020-04-16,32778,6205,891
    2020-07-08,32781,6205,897
    2020-10-08,32785,6205,4215
    2020-12-15,32176,6219,4225
    2021-02-26,31771,6219,4252


# Installation

Requirements:
* **Python 3.6** or later
* **pip 19.0** or later

When in doubt, begin by upgrading `pip`:

    python3 -m pip install --upgrade pip

Then install with

    python3 -m pip install git-repo-language-trends


# Usage

First go to the git repository for a project.

    cd ~/src/your-project

Then run the tool, passing the file extensions for the languages you are
interested in as positional arguments:

    git-repo-language-trends .java .kt

For languages with multiple file extensions such as C, you can use the `+`
syntax which will automatically summarize line counts from both file extensions.
To compare C and Rust:

    git-repo-language-trends .c+.h .rs

If you want relative numbers, enable the `--relative` option:

    git-repo-language-trends --relative .c+.h .rs

Use `git-repo-language-trends --help` to see more options.

If `git-repo-language-trends` is not in your `PATH` after installation you can
run the tool via its module, e.g.:

    python3 -m git_repo_language_trends --help

# Method

Programming langauge usage is determined by the total number of newline
characters in files with a given file extension.

It is easy to come up with something more fancy, but it would be overkill.


# Performance

This program is pretty fast, because it uses the
[**pygit2**](https://github.com/libgit2/pygit2) wrapper for the C library
[**libgit2**](https://github.com/libgit2/libgit2). It easily counts hundreds of
thousands lines per second on low-end machines. It also uses a cache keyed
by git blob ID to avoid counting lines for the same blob twice.


# Development

Clone this repo:

    git clone https://github.com/Enselic/git-repo-language-trends.git

Create a venv:

    python3 -m venv ~/venv-grlt
    source ~/venv-grlt/bin/activate

Install and update dev dependencies:

    python3 -m pip install --upgrade pip flake8 pytest

Make an editable install:

    python3 -m pip install -e .

then make your changes. When done, lint and test:

    flake8 && pytest -vv

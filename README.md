# git-repo-language-trends

Analyze programming language usage over time in a git repository and produce a
graphical or textual representation of the result.

Available output file formats:
* **.svg** - Scalable Vector Graphics
* **.png** - Portable Network Graphics
* **.csv** - Comma-separated values
* **.tsv** - Tab-separated values

Example command and its SVG output:

```
% cd ~/src/cpython
% git-repo-language-trends --max-commits 30 --min-interval-days 365 .c+.h .py
```

![CPython, C vs Python, 1992-2021](./docs/images/cpython-c-vs-python-1992-2021.svg)




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


# Examples

TODO

# Method

Programming langauge usage is determined by the total number of newline
characters in files with a given file extension.


# Performance

This program is pretty fast, because it uses the pygit2 wrapper for the C
library libgit2. On a low-end computer (with an **Intel(R) Celeron(R) J4005 CPU
@ 2.00GHz**) it counts ~400 000 lines per second.

# Development

Clone this repo:

    git clone https://github.com/Enselic/git-repo-language-trends.git

Create a venv:

    python3 -m venv ~/venv-grlt
    source ~/venv-grlt/bin/activate

Install and update dev dependencies:

    python3 -m pip install --upgrade pip flake8 pytest build twine

Make an editable install:

    python3 -m pip install -e .

then make your changes. When done, lint and test:

    flake8 && pytest -vv


# TODO
* test for overwrite existing file
* test for creating dir if not exist
* Add .tsv and .csv and .png and .svg CLI test cases
* handle shallow clones
* add line count to top 3 and --list
* limit size of cache
* Fix CI to use Python 3.6 for app

# Features not yet implemeneted
* Stacked percentage chart
* Support import of .tsv or .csv data to support generating e.g. a PNG without re-reunning analysis
* More short options
* Support -o args multiple times

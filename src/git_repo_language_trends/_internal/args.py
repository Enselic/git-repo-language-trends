import os
import sys
import argparse
import git_repo_language_trends

desc = """
Description:
============

Analyze programming language usage over time in a git repository and produce a
graphical or textual representation of the result.

Several output file formats are available:
* .svg - Scalable Vector Graphics
* .png - Portable Graphics Format
* .csv - Comma-separated values
* .tsv - Tab-separated values

Examples:
=========

First go to any git repository:

    cd ~/src/any-git-repository

Run the tool without arguments to analyze programing language usage of top three
file extensions, and write the result to an SVG file:

    git-repo-language-trends

Analyze Objective-C vs Swift and write the result to a .csv file e.g. so you can
create a graph yourself in your spreadsheet software of choice:

    git-repo-language-trends .m+.h .swift --output=output.csv

Analyze Java vs Kotlin and write the result to a PNG file with a white
background and a custom size:

    git-repo-language-trends .java .kt --output=output.png --size-inches=10,6

Arguments:
==========
"""


def positive_int(arg):
    i = int(arg)
    if i < 0:
        raise argparse.ArgumentTypeError("Must not be negative")
    return i


def positive_float(arg):
    i = float(arg)
    if i < 0:
        raise argparse.ArgumentTypeError("Must not be negative")
    return i


def formatter(prog):
    return argparse.RawDescriptionHelpFormatter(
        "git-repo-language-trends",
        indent_increment=4,
        max_help_position=32,
    )


def get_args():
    parser = argparse.ArgumentParser(
        description=desc,
        formatter_class=formatter,
    )

    parser.add_argument(
        "columns",
        metavar=".ext .ext+.ext",
        nargs='*',
        help="""For what file extensions lines will be counted. Can be specified
        multiple times. Use '.ext' for regular line counting. Use '.ext1+.ext2'
        syntax for auto-summation of several file extensions into a single column.
        If you specify no file extensions, the top three extensions in the
        repository will be used, based on the number of lines in files with the
        extensions.""",
    )

    parser.add_argument(
        '--version',
        action='version',
        version=f"%(prog)s {git_repo_language_trends.__version__}",
    )

    parser.add_argument(
        "--list",
        action='store_true',
        help="list file extensions and their total line count in the first commit",
    )

    parser.add_argument(
        "--min-interval-days",
        metavar="<int>",
        type=positive_int,
        default=7,
        help="""mimimum interval in days between analyzed commits
        (default: %(default)s)""",
    )

    parser.add_argument(
        "--max-commits",
        metavar="<int>",
        type=positive_int,
        default=sys.maxsize,
        help="""maximum number of commits to analyze
        (default: %(default)s)"""
    )

    parser.add_argument(
        "--first-commit",
        metavar="<rev>",
        default="HEAD",
        help="""the commit or tag or branch to start from
        (default: %(default)s)"""
    )

    parser.add_argument(
        "-o", "--output",
        metavar="<filename.ext>",
        default=get_default_output(),
        help="""output filename (omit for stdout) and format (via extension .svg .png .csv or .tsv)
        (default: %(default)s)""",
    )

    parser.add_argument(
        "--all-parents",
        action='store_true',
        help="""[ADVANCED] follow all commit parents with the risk of producing
        inconsistent a.k.a. jumpy graphs""",
    )

    svg_group = parser.add_argument_group(
        ".svg and .png output optional arguments:",
    )

    svg_group.add_argument(
        "--size-inches",
        metavar="<w:h>",
        default="11.75:8.25",
        help="""width:height in inches of the diagram
        (default: %(default)s)""",
    )

    svg_group.add_argument(
        "--style",
        metavar="<name>",
        default="dark",
        choices=['dark', 'light'],
        help="""pass 'dark' for black background and 'light' for white background
        (default: %(default)s)""",
    )

    args = parser.parse_args()

    # Figure out output file extension
    # Without an extension, we treat the entire filename as the extension
    name, ext = os.path.splitext(args.output)
    if not ext:
        ext = name
        name = ""
    args.output_ext = ext

    args.output_stdout = name == ""

    return args


def get_default_output():
    cwd = os.getcwd()
    basename = os.path.basename(cwd)

    return f"{basename}_language-trends.png"

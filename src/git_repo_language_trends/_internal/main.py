#!/usr/bin/env python3

# pylint: disable=C0116

# TODO: Shallow repo crashes

"""
This is a limited re-implementation of git-repo-language-trends
in Python using pygit2. The main purpose of this re-implementation
is to compare performance of Rust vs Python. The Rust version is about
7 times faster than the Python version. Which is a shame, because the
Python version code is much simpler. But since performance has high priority
for this tool, the main and full implementation is in Rust.
"""

import sys

from datetime import datetime
import argparse
import os
import os.path
import pygit2

from .tsv_output import TabSeparatedValuesStdoutOutput

from .svg_output import SvgOutput


def get_args():
    """
    Gets parsed program arguments.
    """
    parser = argparse.ArgumentParser()

    parser.add_argument(
        "columns",
        nargs='*',
        help="""For what file extensions lines will be counted. Can be specified
        multiple times. Use '.ext' for regular line counting. Use '.ext1+.ext2'
        syntax for auto-summation into a single column. If you specify no file
        extensions, the top three extensions in the repository will be used,
        based on the number of lines in files with the extensions.""",
    )

    parser.add_argument(
        "--min-interval-days",
        type=int,
        default=7,
        help="Optional. The mimimum interval in days between commits to analyze.",
    )

    parser.add_argument(
        "--max-commits",
        type=int,
        default=sys.maxsize,
        help="Optional. Maximum number of commits to process."
    )

    parser.add_argument(
        "--start-commit",
        default="HEAD",
        help="Optional. The commit to start parsing from."
    )

    parser.add_argument(
        "--all-parents",
        action='store_true',
        help="""(Advanced.) By default, --first-parent is passed to the internal
        git log command (or libgit2 Rust binding code rather). This ensures that
        the data in each row comes from a source code tree that is an ancestor
        to the row above it. If you prefer data for as many commits as possible,
        even though the data can become inconsistent (a.k.a. 'jumpy'), enable
        this flag.""",
    )

    svg_group = parser.add_argument_group(
        "Scalable Vector Graphics (.svg) output",
    )

    svg_group.add_argument(
        "--disable-svg-output",
        action='store_true',
        help="Disable SVG file output. Enabled by default.",
    )

    svg_group.add_argument(
        "--svg-style",
        default="dark_background",
        help="""Set to 'default' for white background. You can set to any of
        the styles listed here:
        https://matplotlib.org/stable/gallery/style_sheets/style_sheets_reference.html""",
    )

    svg_group.add_argument(
        "--svg-width-inches",
        default=11.75,
        help="Width in inches of SVG diagram.",
    )

    svg_group.add_argument(
        "--svg-height-inches",
        default=8.25,
        help="Height in inches of SVG diagram.",
    )

    tsv_group = parser.add_argument_group(
        "Tab-separated values (.tsv) output",
    )

    tsv_group.add_argument(
        "--enable-tsv-stdout-output",
        action='store_false',
        help="Enable .tsv (tab separated values) stdout output.",
    )

    tsv_group.add_argument(
        "--enable-tsv-file-output",
        action='store_false',
        help="Enable .tsv (tab separated values) file output.",
    )

    return parser.parse_args()


def main():
    args = get_args()
    outputs = get_outputs(args)
    process_commits(args, outputs)


def get_outputs(args):
    outputs = []

    if args.enable_tsv_stdout_output:
        outputs.append(TabSeparatedValuesStdoutOutput())

    if not args.disable_svg_output:
        outputs.append(SvgOutput(args))

    return outputs


def process_commits(args, outputs):
    columns = args.columns

    ext_to_column = generate_ext_to_column_dict(args.columns)

    commits_to_process = get_commits_to_process(args)

    # Print column headers
    for output in outputs:
        output.start(columns)

    # Print rows
    for commit in commits_to_process:
        date = get_commit_date(commit)
        column_to_lines_dict = process_commit(commit, ext_to_column)

        for output in outputs:
            output.add_row(columns, date, column_to_lines_dict)

    # Wrap things up
    for output in outputs:
        output.finish()


def get_commits_to_process(args):
    print("Figuring out what commits to analyze ...", file=sys.stderr)

    commits_to_process = []

    rows_left = args.max_commits

    date_of_last_row = None
    for commit in get_git_log_walker(args):
        if rows_left == 0:
            break

        # Make sure --min-interval days has passed since last printed commit before
        # processing and printing the data for another commit
        current_date = commit.commit_time
        if enough_days_passed(args, date_of_last_row, current_date):
            date_of_last_row = current_date

            commits_to_process.append(commit)

            rows_left -= 1

    # git log shows most recent first, but in the graph
    # you want to have from oldest to newest, so reverse
    commits_to_process.reverse()

    print(f"Will analyze {len(commits_to_process)} commits.", file=sys.stderr)

    return commits_to_process


def process_commit(commit, ext_to_column):
    """
    Counts lines for files with the given file extensions in a given commit.
    """

    blobs = get_blobs_in_commit(commit)

    # Loop through all blobs in the commit tree
    column_to_lines = {}
    for (blob, ext) in blobs:

        # Figure out if we should count the lines for the file extension this
        # blob has, by figuring out what column the lines should be added to,
        # if any
        column = ext_to_column.get(ext) if ext_to_column else ext
        # If no specific columns are requested, we are probably invoked
        # with --list, so count the lines for all extensions

        # If the blob has an extension we care about, count the lines!
        if column:
            lines = get_lines_in_blob(blob)
            column_to_lines[column] = column_to_lines.get(column, 0) + lines

    return column_to_lines


def get_all_blobs_in_tree(tree):
    blobs = []
    trees_left = [tree]
    # Say no to recursion
    while len(trees_left) > 0:
        tree = trees_left.pop()
        for obj in tree:
            if isinstance(obj, pygit2.Tree):
                trees_left.append(obj)
            elif isinstance(obj, pygit2.Blob):
                blobs.append(obj)
    return blobs


def get_blobs_in_commit(commit):
    blobs = []
    for obj in get_all_blobs_in_tree(commit.tree):
        ext = os.path.splitext(obj.name)[1]
        if ext:
            blobs.append((obj, ext))

    return blobs


c = {}  # TODO: Make optinal and measure memory usage for linux kernel


def get_lines_in_blob(blob):
    if blob.oid in c:
        return c[blob.oid]

    lines = 0
    for byte in memoryview(blob):
        if byte == 10:  # \n
            lines += 1

    c[blob.oid] = lines
    return lines


def get_git_log_walker(args):
    repo = pygit2.Repository(os.environ.get('GIT_DIR', '.'))

    rev = repo.revparse_single(args.start_commit)

    walker = repo.walk(rev.oid)

    if not args.all_parents:
        walker.simplify_first_parent()

    return walker


def enough_days_passed(args, date_of_last_row, current_date):
    """
    Checks if enough days according to --min-interval has passed, i.e. if it is
    time to process and print another commit.
    """

    if date_of_last_row:
        days = ((date_of_last_row - current_date) / 60 / 60 / 24)
        return days > args.min_interval_days
    return True


def generate_ext_to_column_dict(columns):
    extension_to_column_dict = {}
    for column in columns:
        for ext in column.split('+'):
            extension_to_column_dict[ext] = column
    return extension_to_column_dict


def get_commit_date(commit):
    return datetime.utcfromtimestamp(commit.commit_time).strftime('%Y-%m-%d')


if __name__ == '__main__':
    main()

package main

import (
	"fmt"

	"github.com/alexflint/go-arg"
)

func main() {
	var args args2
	arg.MustParse(&args2)


    args2 = get_args()
    // if args2.List:
    //     list_available_file_extensions(args)
    // else:
	outputs = get_outputs(args)
	process_commits(args, outputs)
}

def list_available_file_extensions(args):
    ext_to_lines = get_data_for_first_commit(args)
    sorted_exts = get_extensions_sorted_by_popularity(ext_to_lines)
    print("Available extensions in first commit:")

    len_of_longest_ext = len(max(sorted_exts, key=len))
    for ext in sorted_exts:
        print(f"{ext:<{len_of_longest_ext}} - {ext_to_lines[ext]} lines")


func get_outputs(args) {
    // It should be pretty easy to add support for having multiple
    // outputs generated at once, but for now we only support one at a time.
    outputs = []

    if args.output_ext == ".svg" or args.output_ext == ".png":
        outputs.append(MatplotlibOutput(args))
    elif args.output_ext == ".tsv":
        outputs.append(SeparatedValuesOutput(args, "\t"))
    elif args.output_ext == ".csv":
        outputs.append(SeparatedValuesOutput(args, ","))
    else:
        sys.exit(f"Output file format '{args.output_ext}' not supported")

    return outputs
}

def process_commits(args, outputs):
    columns = args.columns
    if len(columns) == 0:
        print("No file extensions specified, will use top three.")
        data = get_data_for_first_commit(args)
        columns = get_top_three_extensions(data)
        print(f"Top three extensions were: {' '.join(columns)}")

    if len(columns) == 0:
        sys.exit("No extensions to count lines for")

    ext_to_column = generate_ext_to_column_dict(columns)

    commits_to_process = get_commits_to_process(args)

    # Since we analyze many commits, but many commits share the same blobs,
    # caching how many lines there are in a blob (keyed by git object id) speeds
    # things up significantly, without a dramatic memory usage increase.
    blob_to_lines_cache = None if args.no_cache else {}

    progress_state = Progress(args, len(commits_to_process))

    # Print column headers
    for output in outputs:
        output.start(columns)

    # Print rows
    for commit in commits_to_process:
        date = get_commit_date(commit)
        column_to_lines_dict = process_commit(
            commit,
            ext_to_column,
            blob_to_lines_cache,
            progress_state,
        )

        for output in outputs:
            output.add_row(
                columns,
                date,
                to_relative_numbers_if_enabled(args, column_to_lines_dict),
            )

        progress_state.commit_processed()

    # Wrap things up
    for output in outputs:
        output.finish()


# Calls process_commit for the first commit (possibly from --first-commit)
def get_data_for_first_commit(args):
    repo = get_repo()
    rev = repo.revparse_single(args.first_commit)
    return process_commit(rev.peel(pygit2.Commit), None, None, Progress(args, 1))


def get_commits_to_process(args):
    commits_to_process = []

    rows_left = args.max_commits

    date_of_last_row = None
    try:
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
    except KeyError:
        # Analyzing a shallow git clone will cause the walker to throw an
        # exception in the end. That is not a catastrophe. We already collected
        # some data. So just keep going after printing a notice.
        print("WARNING: unexpected end of git log, maybe a shallow git repo?")
        pass

    # git log shows most recent first, but in the graph
    # you want to have from oldest to newest, so reverse
    commits_to_process.reverse()

    return commits_to_process


def process_commit(commit, ext_to_column, blob_to_lines_cache, progress_state):
    """
    Counts lines for files with the given file extensions in a given commit.
    """

    blobs = get_blobs_in_commit(commit)

    column_to_lines = {}
    len_blobs = len(blobs)
    # We don't want to use an iterator here, because that will hold on to the
    # pygit2 Blob object, preventing the libgit2 git_blob_free (or actually;
    # git_object_free) from being called even though we are done counting lines
    index = 0
    while len(blobs) > 0:
        # One based counting since the printed progress is for human consumption
        index += 1
        (blob, ext) = blobs.pop()
        progress_state.print_state(index, len_blobs)

        # Figure out if we should count the lines for the file extension this
        # blob has, by figuring out what column the lines should be added to,
        # if any
        column = ext_to_column.get(ext) if ext_to_column else ext
        # If no specific columns are requested, we are probably invoked
        # with --list, so count the lines for all extensions

        # If the blob has an extension we care about, count the lines!
        if column:
            lines = get_lines_in_blob(blob, blob_to_lines_cache)
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


def get_lines_in_blob(blob, blob_to_lines_cache):
    # Don't use the blob.oid directly, because that will keep the underlying git
    # blob object alive, preventing freeing of the blob content from
    # git_blob_get_rawcontent(), which quickly accumulate to hundred of megs of
    # heap memory when analyzing large git projects such as the linux kernel
    hex = blob.oid.hex

    if blob_to_lines_cache is not None and hex in blob_to_lines_cache:
        return blob_to_lines_cache[hex]

    lines = 0
    for byte in memoryview(blob):
        if byte == 10:  # \n
            lines += 1

    if blob_to_lines_cache is not None:
        blob_to_lines_cache[hex] = lines

    return lines


def get_repo():
    return pygit2.Repository(os.environ.get('GIT_DIR', '.'))


def get_git_log_walker(args):
    repo = get_repo()

    rev = repo.revparse_single(args.first_commit)

    walker = repo.walk(rev.peel(pygit2.Commit).oid)

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




	var desc = `Description:
  Analyze programming language usage over time in a git repository and produce a
  graphical or textual representation of the result.

  Available output file formats:
  * .svg - Scalable Vector Graphics
  * .png - Portable Network Graphics
  * .csv - Comma-separated values
  * .tsv - Tab-separated values

Examples:
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

    git-repo-language-trends .java .kt --output=output.png --size-inches=10,6 --style=light
`

type args struct {

	//     parser.add_argument(
	//         "columns",
	//         metavar=".ext .ext+.ext",
	//         nargs='*',
	//         help="""For what file extensions lines will be counted. Can be specified
	//         multiple times. Use '.ext' for regular line counting. Use '.ext1+.ext2'
	//         syntax for auto-summation of several file extensions, e.g. .c+.h for all C files.
	//         If you specify no file extensions, the top three extensions in the
	//         repository will be used, based on the number of lines in files with the
	//         extensions.""",
	//     )

	List bool `arg:"-l" help:"list file extensions and their total line count in the first commit"`

	//         type=positive_int,
	// placeholder="<int>"
	MinIntervalDays int `arg:"--min-interval-days" default:"7" help:"mimimum interval in days between analyzed commits"`

	//         type=positive_int,
	// placeholder="<int>"
	MaxCommits int `arg:"--max-commits,-n" default:"2100000000" help:"maximum number of commits to analyze"`

	//         metavar="<rev>",
	FirstCommit string `arg:"--first-commit" default:"HEAD" help:"the commit or tag or branch to start from"`

	//     parser.add_argument(
	//         "--relative",
	//         action='store_true',
	//         help="use relative instead of absolute numbers",
	//     )

	//
	//         metavar="<out.ext>",
	//         default=get_default_output(),
	//     )
	Output string `arg:"--output,-o" default:"out.png" help:"output filename and format (via extension .svg .png .csv or .tsv)"`
}

// TODO: Change colWidth in /home/martin/go/pkg/mod/github.com/alexflint/go-arg@v1.4.2/usage.go
func (args) Version() string {
	//     parser.add_argument(
	//         '--version',
	//         action='version',
	//         version=f"%(prog)s {git_repo_language_trends.__version__}",
	//     )
	return "TODO: --version"
}

func (args) Description() string {
	return desc
}

//     svg_group = parser.add_argument_group(
//         "SVG/PNG related optional arguments",
//     )

//     svg_group.add_argument(
//         "--size-inches",
//         metavar="<w,h>",
//         default="11.75,8.25",
//         help="""width:height in inches of the diagram
//         (default: %(default)s)""",
//     )

//     svg_group.add_argument(
//         "--style",
//         metavar="<name>",
//         default="dark",
//         choices=['dark', 'light'],
//         help="""pass 'dark' for black background and 'light' for white background
//         (default: %(default)s)""",
//     )

//     svg_group.add_argument(
//         "--no-watermark",
//         action='store_true',
//         help="remove the watermark that is barely visible to begin with",
//     )

//     advanced_group = parser.add_argument_group(
//         "advanced optional arguments",
//     )

//     advanced_group.add_argument(
//         "--no-cache",
//         action='store_true',
//         help="""[ADVANCED] do not cache how many lines are in a blob""",
//     )

//     advanced_group.add_argument(
//         "--no-progress",
//         action='store_true',
//         help="""[ADVANCED] do not print progress""",
//     )

//     advanced_group.add_argument(
//         "--all-parents", "-a",
//         action='store_true',
//         help="""[ADVANCED] increase pool of candidate commits by following all
//         commit parents, but with the risk of producing inconsistent/jumpy graphs""",
//     )

//     args = parser.parse_args()

//     # pre-parse width and height
//     width_inches, height_inches = args.size_inches.split(',')
//     args.size_inches = (float(width_inches), float(height_inches))

//     # Figure out output file extension
//     # Without an extension, we treat the entire filename as the extension
//     name, ext = os.path.splitext(args.output)
//     if not ext:
//         ext = name
//         name = ""
//     args.output_ext = ext

//     # Do a favor to the user, create the parent dirs if they are missing
//     Path(os.path.dirname(args.output)).mkdir(exist_ok=True, parents=True)

//     return args

// def get_default_output():
//     cwd = os.getcwd()
//     basename = os.path.basename(cwd)

//     return f"{basename}-language-trends.png"

// def positive_int(arg):
//     i = int(arg)
//     if i < 0:
//         raise argparse.ArgumentTypeError("Must not be negative")
//     return i

// def positive_float(arg):
//     i = float(arg)
//     if i < 0:
//         raise argparse.ArgumentTypeError("Must not be negative")
//     return i

// def formatter(prog):
//     return argparse.RawDescriptionHelpFormatter(
//         "git-repo-language-trends",
//         indent_increment=4,
//         max_help_position=38,
//     )

// def get_args():
//     parser = argparse.ArgumentParser(
//         description=desc,
//         formatter_class=formatter,
//     )

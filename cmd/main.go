package main

import (
	"fmt"

	"github.com/jessevdk/go-flags"
)

type Options struct {
	Verbose []bool `short:"v" long:"verbose" description:"Show verbose debug information"`
}

func main() {
	var opts struct {
		// Slice of bool will append 'true' each time the option
		// is encountered (can be set multiple times, like -vvv)
		Verbose []bool `short:"v" long:"verbose" description:"Show verbose debug information"`

		// Example of automatic marshalling to desired type (uint)
		Offset uint `long:"offset" description:"Offset"`

		// Example of a callback, called each time the option is found.
		Call func(string) `short:"c" description:"Call phone number"`

		// Example of a required flag
		Name string `short:"n" long:"name" description:"A name" required:"true"`

		// Example of a flag restricted to a pre-defined set of strings
		Animal string `long:"animal" choice:"cat" choice:"dog"`

		// Example of a value name
		File string `short:"f" long:"file" description:"A file" value-name:"FILE"`

		// Example of a pointer
		Ptr *int `short:"p" description:"A pointer to an integer"`

		// Example of a slice of strings
		StringSlice []string `short:"s" description:"A slice of strings"`

		// Example of a slice of pointers
		PtrSlice []*string `long:"ptrslice" description:"A slice of pointers to string"`

		// Example of a map
		IntMap map[string]int `long:"intmap" description:"A map from string to int"`
	}

	_, err := flags.Parse(&opts)

	if err != nil {
		panic(err)
	}
	fmt.Println("Hej")
}

// import os
// import sys
// import argparse
// import git_repo_language_trends
// from pathlib import Path

// desc = """
// Description:
// ============

// Analyze programming language usage over time in a git repository and produce a
// graphical or textual representation of the result.

// Available output file formats:
// * .svg - Scalable Vector Graphics
// * .png - Portable Network Graphics
// * .csv - Comma-separated values
// * .tsv - Tab-separated values

// Examples:
// =========

// First go to any git repository:

//     cd ~/src/any-git-repository

// Run the tool without arguments to analyze programing language usage of top three
// file extensions, and write the result to an SVG file:

//     git-repo-language-trends

// Analyze Objective-C vs Swift and write the result to a .csv file e.g. so you can
// create a graph yourself in your spreadsheet software of choice:

//     git-repo-language-trends .m+.h .swift --output=output.csv

// Analyze Java vs Kotlin and write the result to a PNG file with a white
// background and a custom size:

//     git-repo-language-trends .java .kt --output=output.png --size-inches=10,6 --style=light

// Arguments:
// ==========
// """

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

//     parser.add_argument(
//         '--version',
//         action='version',
//         version=f"%(prog)s {git_repo_language_trends.__version__}",
//     )

//     parser.add_argument(
//         "--list", "-l",
//         action='store_true',
//         help="list file extensions and their total line count in the first commit",
//     )

//     parser.add_argument(
//         "--min-interval-days",
//         metavar="<int>",
//         type=positive_int,
//         default=7,
//         help="""mimimum interval in days between analyzed commits
//         (default: %(default)s)""",
//     )

//     parser.add_argument(
//         "--max-commits", "-n",
//         metavar="<int>",
//         type=positive_int,
//         default=sys.maxsize,
//         help="""maximum number of commits to analyze
//         (default: %(default)s)"""
//     )

//     parser.add_argument(
//         "--first-commit",
//         metavar="<rev>",
//         default="HEAD",
//         help="""the commit or tag or branch to start from
//         (default: %(default)s)"""
//     )

//     parser.add_argument(
//         "--relative",
//         action='store_true',
//         help="use relative instead of absolute numbers",
//     )

//     parser.add_argument(
//         "--output", "-o",
//         metavar="<out.ext>",
//         default=get_default_output(),
//         help="""output filename and format (via extension .svg .png .csv or .tsv)
//         (default: %(default)s)""",
//     )

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

package main

import (
	"errors"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/go-git/go-git/v5"
	"github.com/go-git/go-git/v5/plumbing/object"
)

func main() {
	args := GetArgs()
	// if args2.List:
	//     list_available_file_extensions(args)
	// else:
	outputs := get_outputs(args)
	process_commits(args, outputs)
}

// func list_available_file_extensions(args) {
//     ext_to_lines = get_data_for_first_commit(args)
//     sorted_exts = get_extensions_sorted_by_popularity(ext_to_lines)
//     fmt.Printf("Available _, extensions := range first commit:")

//     len_of_longest_ext = len(max(sorted_exts, key=len))
//     for _, ext := range sorted_exts:
//         fmt.Println(f"{ext:<{len_of_longest_ext}} - {ext_to_lines[ext]} lines")

func get_outputs(args AppArgs) []Output {
	// It should be pretty easy to add support for having multiple
	// outputs generated at once, but for now we only support one at a time.
	var outputs []Output

	// if args.Output == ".svg" || args.output_ext == ".png":
	//     outputs.append(MatplotlibOutput(args))
	// elif args.output_ext == ".tsv":
	//     outputs.append(SeparatedValuesOutput(args, "\t"))
	// elif args.output_ext == ".csv":
	//     outputs.append(SeparatedValuesOutput(args, ","))
	// else:
	//     sys.exit(f"Output file format '{args.output_ext}' not supported")

	outputs = append(outputs, NewSeparatedValuesOutput(args, ","))

	return outputs
}

func process_commits(args AppArgs, outputs []Output) error {
	columns := []string{".go", ".py"} // args.columns
	// if len(columns) == 0 {
	//     fmt.Printf("No file extensions specified, will use top three.")
	//     data = get_data_for_first_commit(args)
	//     columns = get_top_three_extensions(data)
	//     fmt.Println("Top three extensions were: ", {' '.join(columns)})
	// }
	// if len(columns) == 0 {
	//     sys.exit("No extensions to count lines for")
	// }
	ext_to_column := generate_ext_to_column_dict(columns)

	commits_to_process, err := get_commits_to_process(args)
	if err != nil {
		return nil
	}

	// // Since we analyze many commits, but many commits share the same blobs,
	// // caching how many lines there _, are := range a blob (keyed by git object id) speeds
	// // things up significantly, without a dramatic memory usage increase.
	var file_to_lines_cache map[object.Blob]int
	// if !args.NoCache {
	// 	file_to_lines_cache = make(map[object.Blob]int)
	// }

	// progress_state = Progress(args, len(commits_to_process))

	// Print column headers
	for _, output := range outputs {
		output.start(columns)
	}
	// Print rows
	for _, commit := range commits_to_process {
		date := get_commit_date(commit)
		column_to_lines_dict, err := process_commit(
			commit,
			ext_to_column,
			file_to_lines_cache,
			//progress_state,
		)
		if err != nil {
			return err
		}

		for _, output := range outputs {
			output.add_row(
				columns,
				date,
				column_to_lines_dict, //to_relative_numbers_if_enabled(args, column_to_lines_dict),
			)
		}
		//progress_state.commit_processed()
	}
	// Wrap things up
	for _, output := range outputs {
		output.finish()
	}

	return nil
}

// // Calls process_commit for the first commit (possibly from --first-commit)
// func get_data_for_first_commit(args) {
//     repo = get_repo()
//     r, err := git.PlainOpen(path)
//     rev = repo.revparse_single(args.first_commit)
//     return process_commit(rev.peel(pygit2.Commit), None, None, Progress(args, 1))
// }

func get_commits_to_process(args AppArgs) ([]*object.Commit, error) {
	repo, err := get_repo()
	if err != nil {
		return nil, err
	}

	ref, err := repo.Head()
	if err != nil {
		return nil, err
	}

	var commits_to_process []*object.Commit

	var date_of_last_row *time.Time
	rows_left := args.MaxCommits
	iter, err := repo.Log(&git.LogOptions{From: ref.Hash()})
	if err != nil {
		return nil, err
	}
	iter.ForEach(func(commit *object.Commit) error {
		if rows_left <= 0 {
			return errors.New("done")
		}

		// Make sure --min-interval days has passed since last printed commit before
		// processing and printing the data for another commit
		current_date := &commit.Author.When
		if enough_days_passed(args, date_of_last_row, current_date) {
			date_of_last_row = current_date

			commits_to_process = append(commits_to_process, commit)

			rows_left -= 1
		}

		return nil
	})

	// except KeyError:
	//     // Analyzing a shallow git clone will cause the walker to throw an
	//     // _, exception := range the end. That is not a catastrophe. We already collected
	//     // some data. So just keep going after printing a notice.
	//     fmt.Printf("WARNING: unexpected end of git log, maybe a shallow git repo?")
	//     pass

	// // git log shows most recent first, but for the graph
	// // you want to have from oldest to newest, so reverse
	reverse_commits(commits_to_process)

	return commits_to_process, nil
}

// I can't belive this is not part of standard library in Go, but oh well
// https://github.com/golang/go/wiki/SliceTricks#reversing
func reverse_commits(a []*object.Commit) {
	for i := len(a)/2 - 1; i >= 0; i-- {
		opp := len(a) - 1 - i
		a[i], a[opp] = a[opp], a[i]
	}
}

// Counts lines for files with the given file _, extensions := range a given commit.
func process_commit(
	commit *object.Commit,
	ext_to_column map[string]string,
	file_to_lines_cache map[object.Blob]int, /*, progress_state*/
) (map[string]int, error) {
	files, err := get_files_in_commit(commit)
	if err != nil {
		return nil, err
	}

	column_to_lines := make(map[string]int)
	//len_files := len(files)
	// We don't want to use an iterator here, because that will hold on to the
	// pygit2 Blob object, preventing the libgit2 git_blob_free (or actually;
	// git_object_free) from being called even though we are done counting lines
	index := 0
	for _, file := range files {
		// One based counting since the printed progress is for human consumption
		index += 1

		ext := filepath.Ext(file.Name)
		//progress_state.print_state(index, len_blobs)

		// Figure out if we should count the lines for the file extension this
		// blob has, by figuring out what column the lines should be added to,
		// if any
		var column string
		if ext_to_column != nil {
			column = ext_to_column[ext]
		} else {
			column = ext
		}
		// If no specific columns are requested, we are probably invoked
		// with --list, so count the lines for all extensions

		// If the blob has an extension we care about, count the lines!
		if column != "" {
			lines, err := get_lines_in_file(file, file_to_lines_cache)
			if err != nil {
				return nil, err
			}
			column_to_lines[column] = column_to_lines[column] + lines
		}
	}

	return column_to_lines, nil
}

// func get_all_blobs_in_tree(tree object.Tree) {
// 	// blobs = make([]object.Blob)
// 	// trees_left = [tree]
// 	// // Say no to recursion
// 	// for len(trees_left) > 0 {
// 	//     tree = trees_left.pop()
// 	//     for _, obj := range tree {
// 	//         if isinstance(obj, pygit2.Tree) {
// 	//             trees_left.append(obj)
// 	//         else if isinstance(obj, pygit2.Blob) {
// 	//             blobs.append(obj)
// 	//         }
// 	//     }
// 	// }
// 	// return blobs
// }

func get_files_in_commit(commit *object.Commit) ([]*object.File, error) {
	var files []*object.File

	iter, err := commit.Files()
	if err != nil {
		return nil, err
	}
	defer iter.Close() // TODO: More places?

	iter.ForEach(func(f *object.File) error {
		files = append(files, f)

		return nil
	})

	// for _, obj := range get_all_blobs_in_tree(commit.tree) {
	//     ext = os.path.splitext(obj.name)[1]
	//     if ext {
	//         blobs.append((obj, ext))
	//     }
	// }

	return files, nil
}

func get_lines_in_file(file *object.File, file_to_lines_cache map[object.Blob]int) (int, error) {
	// // Don't use the blob.oid directly, because that will keep the underlying git
	// // blob object alive, preventing freeing of the blob content from
	// // git_blob_get_rawcontent(), which quickly accumulate to hundred of megs of
	// // heap memory when analyzing large git projects such as the linux kernel
	// hex = blob.oid.hex

	// if file_to_lines_cache is not None and _, hex := range file_to_lines_cache {
	//     return file_to_lines_cache[hex]
	// }

	// lines = 0
	// for _, byte := range memoryview(blob) {
	//     if byte == 10 {  // \n
	//         lines += 1
	//     }
	// }

	// if file_to_lines_cache is not None {
	//     file_to_lines_cache[hex] = lines
	// }

	// return lines

	lines, err := file.Lines()
	if err != nil {
		return 0, err
	}

	return len(lines), nil
}

func get_repo() (*git.Repository, error) {
	path, exists := os.LookupEnv("GIT_DIR")
	if !exists {
		path = "."
	}

	return git.PlainOpen(path)
}

// func get_git_log_walker(args AppArgs) {
// 	repo, err := get_repo()
// 	if err == nil {
// 		return nil, err
// 	}

// 	rev = repo.revparse_single(args.first_commit)

// 	walker = repo.walk(rev.peel(pygit2.Commit).oid)

// 	if !args.AllParents {
// 		walker.simplify_first_parent()
// 	}

// 	return walker
// }

// Checks if enough days according to --min-interval has passed, i.e. if it is
// time to process and print another commit.
func enough_days_passed(args AppArgs, date_of_last_row *time.Time, current_date *time.Time) bool {
	if date_of_last_row != nil {
		duration := date_of_last_row.Sub(*current_date)
		days := duration.Seconds() / 60 / 60 / 24
		return days > float64(args.MinIntervalDays)
	}
	return true
}

func generate_ext_to_column_dict(columns []string) map[string]string {
	extension_to_column_dict := make(map[string]string)
	for _, column := range columns {
		for _, ext := range strings.Split(column, "+") {
			extension_to_column_dict[ext] = column
		}
	}
	return extension_to_column_dict
}

func get_commit_date(commit *object.Commit) string {
	return commit.Author.When.Format("2006-01-02")
}

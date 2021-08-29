package main

import (
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strings"
	"time"

	git "github.com/libgit2/git2go/v28"
)

func main() {
	args := GetArgs()
	var err error
	if args.List {
		err = list_available_file_extensions(args)
	} else {
		outputs := get_outputs(args)
		err = process_commits(args, outputs)
	}
	if err != nil {
		log.Fatal(err)
	}
}

func list_available_file_extensions(args AppArgs) error {
	ext_to_lines, err := get_data_for_first_commit(args)
	if err != nil {
		return err
	}
	sorted_exts := get_extensions_sorted_by_popularity(ext_to_lines)
	fmt.Print("Available extensions in first commit:", sorted_exts)

	// len_of_longest_ext = len(max(sorted_exts, key=len))
	// for _, ext := range sorted_exts {
	//     fmt.Println(f"{ext:<{len_of_longest_ext}} - {ext_to_lines[ext]} lines")
	// }

	return nil
}

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
	repo, err := get_repo()
	if err != nil {
		return err
	}

	columns := args.Columns
	if len(columns) == 0 {
		fmt.Printf("No file extensions specified, will use top three.")
		data, err := get_data_for_first_commit(args)
		if err != nil {
			return err
		}
		columns := get_top_three_extensions(data)
		fmt.Println("Top three extensions were: ", strings.Join(columns, " "))
	}
	if len(columns) == 0 {
		log.Fatal("No extensions to count lines for")
	}
	ext_to_column := generate_ext_to_column_dict(columns)

	commits_to_process, err := get_commits_to_process(repo, args)
	if err != nil {
		return nil
	}

	// Since we analyze many commits, but many commits share the same blobs,
	// caching how many lines there _, are := range a blob (keyed by git object id) speeds
	// things up significantly, without a dramatic memory usage increase.
	var file_to_lines_cache map[git.Blob]int
	if !args.NoCache {
		file_to_lines_cache = make(map[git.Blob]int)
	}

	// progress_state = Progress(args, len(commits_to_process))

	// Print column headers
	for _, output := range outputs {
		output.start(columns)
	}
	// Print rows
	for _, commit := range commits_to_process {
		date := get_commit_date(commit)
		column_to_lines_dict, err := process_commit(
			repo,
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

// Calls process_commit for the first commit (possibly from --first-commit)
func get_data_for_first_commit(args AppArgs) (map[string]int, error) {
	repo, err := get_repo()
	if err != nil {
		return nil, err
	}
	object, err := revparse_single_to_object(repo, args.FirstCommit)
	if err != nil {
		return nil, err
	}

	commit, err := object.AsCommit()
	if err != nil {
		return nil, err
	}

	return process_commit(repo, commit, nil, nil, NewProgress(args, 1))
}

func revparse_single_to_object(repo *git.Repository, spec string) (*git.Object, error) {
	rev, err := repo.RevparseSingle(spec)
	if err != nil {
		return nil, err
	}

	return rev.Peel(git.ObjectCommit)
}

/*
func get_commits_to_process(args AppArgs) ([]*git.Commit, error) {
	repo, err := get_repo()
	if err != nil {
		return nil, err
	}

	ref, err := repo.Head()
	if err != nil {
		return nil, err
	}

	var commits_to_process []*git.Commit

	var date_of_last_row *time.Time
	rows_left := args.MaxCommits
	iter, err := repo.Log(&git.LogOptions{From: ref.Hash()})
	if err != nil {
		return nil, err
	}
	iter.ForEach(func(commit *git.Commit) error {
		if rows_left <= 0 {
			return errors.New("done")
		}

		// Make sure --min-interval days has passed since last printed commit before
		// processing and printing the data for another commit
		current_date := &commit.Author().When
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
*/

func get_commits_to_process(repo *git.Repository, args AppArgs) ([]*git.Commit, error) {
	var commits_to_process []*git.Commit

	rows_left := args.MaxCommits

	var date_of_last_row *time.Time
	walker, err := get_git_log_walker(repo, args)
	if err != nil {
		return nil, err
	}
	//try:
	walker.Iterate(func(commit *git.Commit) bool {
		if rows_left == 0 {
			return false // don't continue
		}

		// Make sure --min-interval days has passed since last printed commit before
		// processing and printing the data for another commit
		current_date := &commit.Author().When
		if enough_days_passed(args, date_of_last_row, current_date) {
			date_of_last_row = current_date

			commits_to_process = append(commits_to_process, commit)

			rows_left -= 1
		}

		return true // continue
	})
	// except KeyError:
	//     // Analyzing a shallow git clone will cause the walker to throw an
	//     // exception in the end. That is not a catastrophe. We already collected
	//     // some data. So just keep going after printing a notice.
	//     print("WARNING: unexpected end of git log, maybe a shallow git repo?")
	//     pass

	// git log shows most recent first, but in the graph
	// you want to have from oldest to newest, so reverse
	reverse_commits(commits_to_process)

	return commits_to_process, nil
}

// I can't belive this is not part of standard library in Go, but oh well
// https://github.com/golang/go/wiki/SliceTricks#reversing
func reverse_commits(a []*git.Commit) {
	for i := len(a)/2 - 1; i >= 0; i-- {
		opp := len(a) - 1 - i
		a[i], a[opp] = a[opp], a[i]
	}
}

// Counts lines for files with the given file _, extensions := range a given commit.
func process_commit(
	repo *git.Repository,
	commit *git.Commit,
	ext_to_column map[string]string,
	file_to_lines_cache map[git.Blob]int,
	progress_state *Progress,
) (map[string]int, error) {
	blobs, err := get_blobs_in_commit(commit)
	if err != nil {
		return nil, err
	}

	column_to_lines := make(map[string]int)
	len_blobs := len(blobs)
	// We don't want to use an iterator here, because that will hold on to the
	// pygit2 Blob object, preventing the libgit2 git_blob_free (or actually;
	// git_object_free) from being called even though we are done counting lines
	index := 0
	for _, file := range blobs {
		// One based counting since the printed progress is for human consumption
		index += 1

		ext := filepath.Ext(file.name)
		progress_state.print_state(index, len_blobs)

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
			lines, err := get_lines_in_blob(repo, file, file_to_lines_cache)
			if err != nil {
				return nil, err
			}
			column_to_lines[column] = column_to_lines[column] + lines
		}
	}

	return column_to_lines, nil
}

type BlobNameAndOid struct {
	name string
	oid  *git.Oid
}

func get_all_blobs_in_tree(tree *git.Tree) []*BlobNameAndOid {
	var blobs []*BlobNameAndOid

	tree.Walk(func(s string, te *git.TreeEntry) int {
		if te.Type == git.ObjectBlob {
			blobs = append(blobs, &BlobNameAndOid{
				te.Name,
				te.Id,
			})
		}

		return 0 // < 0 stops walk
	})

	return blobs
}

func get_blobs_in_commit(commit *git.Commit) ([]*BlobNameAndOid, error) {
	tree, err := commit.Tree()
	if err != nil {
		return nil, err
	}

	return get_all_blobs_in_tree(tree), nil
}

func get_lines_in_blob(
	repo *git.Repository,
	blob_name_and_oid *BlobNameAndOid,
	blob_to_lines_cache map[git.Blob]int,
) (int, error) {
	// Don't use the blob.oid directly, because that will keep the underlying git
	// blob object alive, preventing freeing of the blob content from
	// git_blob_get_rawcontent(), which quickly accumulate to hundred of megs of
	// heap memory when analyzing large git projects such as the linux kernel
	// hex = blob.oid.hex

	// if blob_to_lines_cache is not None and hex in blob_to_lines_cache {
	//     return blob_to_lines_cache[hex]
	// }

	blob, err := repo.LookupBlob(blob_name_and_oid.oid)
	if err != nil {
		return 0, err
	}

	lines := 0
	for _, ch := range blob.Contents() {
		if ch == 10 { // \n
			lines += 1
		}
	}

	// if blob_to_lines_cache is not None {
	//     blob_to_lines_cache[hex] = lines
	// }

	return lines, nil
}

// func get_lines_in_blob(file *BlobNameAndOid, file_to_lines_cache map[git.Blob]int) (int, error) {
// 	// Don't use the blob.oid directly, because that will keep the underlying git
// 	// blob object alive, preventing freeing of the blob content from
// 	// git_blob_get_rawcontent(), which quickly accumulate to hundred of megs of
// 	// heap memory when analyzing large git projects such as the linux kernel
// 	hex = blob.oid.hex

// 	if file_to_lines_cache is not None and _, hex := range file_to_lines_cache {
// 	    return file_to_lines_cache[hex]
// 	}

// 	lines = 0
// 	for _, byte := range memoryview(blob) {
// 	    if byte == 10 {  // \n
// 	        lines += 1
// 	    }
// 	}

// 	if file_to_lines_cache is not None {
// 	    file_to_lines_cache[hex] = lines
// 	}

// 	return lines

// 	lines, err := file.Lines()
// 	if err != nil {
// 		return 0, err
// 	}

// 	return 42, nil //len(lines), nil
// }

func get_repo() (*git.Repository, error) {
	path, exists := os.LookupEnv("GIT_DIR")
	if !exists {
		path = "."
	}

	return git.OpenRepository(path)
}

func get_git_log_walker(
	repo *git.Repository,
	args AppArgs,
) (*git.RevWalk, error) {
	commit, err := revparse_single_to_object(repo, args.FirstCommit)
	if err != nil {
		return nil, err
	}

	walker, err := repo.Walk()
	if err != nil {
		return nil, err
	}
	walker.Push(commit.Id())

	// if !args.AllParents {
	// 	walker.SimplifyFirstParent()
	// }

	return walker, nil
}

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

func get_commit_date(commit *git.Commit) string {
	return commit.Author().When.Format("2006-01-02")
}

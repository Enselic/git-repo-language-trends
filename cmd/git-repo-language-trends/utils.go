package main

import (
	"fmt"
	"sort"
)

// Is there not a nicer way to do this?? Go is such a silly language sometimes ...
type StringAndInt struct {
	s string
	i int
}

func get_extensions_sorted_by_popularity(column_to_lines_dict map[string]int) []string {
	var entries []StringAndInt
	for k, v := range column_to_lines_dict {
		entries = append(entries, StringAndInt{k, v})
	}

	sort.Slice(entries, func(i, j int) bool { return entries[i].i < entries[j].i })

	var extensions []string
	for _, entry := range entries {
		extensions = append(extensions, entry.s)
	}
	return extensions
}

// Excludes some extensions very unlikely to be of interest, e.g. '.lock'
func get_top_three_extensions(column_to_lines_dict map[string]int) []string {
	data := get_extensions_sorted_by_popularity(column_to_lines_dict)
	var result []string
	for _, item := range data {
		if item != ".lock" {
			result = append(result, item)
		}
	}

	return result[0:3]
}

// func to_relative_numbers_if_enabled(args, column_to_lines) {
// if not args.relative:
// 	return column_to_lines

// relative_column_to_lines = {}

// total_lines = float(sum(column_to_lines.values()))
// for column in list(column_to_lines.keys()):
// 	relative_column_to_lines[column] = round((column_to_lines[column] / total_lines) * 100, 2)

// return relative_column_to_lines
// }

func print_file_written(filename string) {
	fmt.Println("\n"+
		"Wrote output to file:\n"+
		"\n"+
		"%s\n", filename)
}

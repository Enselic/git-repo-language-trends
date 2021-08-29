package main

import "fmt"

func get_extensions_sorted_by_popularity(column_to_lines_dict map[string]int) []string {
	var keys []string
	for key := range column_to_lines_dict {
		keys = append(keys, key)
	}
	// TODO: r = sorted(column_to_lines_dict, key=column_to_lines_dict.get)
	// r.reverse()
	return keys
}

// Excludes some extensions very unlikely to be of interest, e.g. '.lock'
func get_top_three_extensions(column_to_lines_dict map[string]int) []string {
	data := get_extensions_sorted_by_popularity(column_to_lines_dict)
	// TODO: filtered_data = list(filter(lambda e: e != ".lock", data))
	return data[0:3]
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

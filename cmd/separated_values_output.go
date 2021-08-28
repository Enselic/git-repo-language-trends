// This module adds support for
//
//   Tab-separated values
//
// and
//
//   Comma-separated values
//
// output, i.e. .tsv and .csv file formats.
package main

import (
	"fmt"
	"os"
)

type SeparatedValuesOutput struct {
	args      AppArgs
	separator string
	dest      *os.File
}

func write_header_row(file *os.File, separator string, columns []string) {
	// To get correct tab alignment, pad with spaces in place of a date
	// on the first row
	file.WriteString("          ") // for YYYY-MM-DD

	// Now write the columns
	for _, column := range columns {
		file.WriteString(fmt.Sprintf("%s%s", separator, column))
	}

	// ... and finish with a newline
	file.WriteString("\n")
}

func write_row(file *os.File, separator string, columns []string, date string, column_to_lines_dict map[string]int) {
	// Date
	file.WriteString(date)

	// Line count information
	for _, column := range columns {
		s := fmt.Sprintf("%s%d", separator, column_to_lines_dict[column])
		//fmt.Println("yep", file, s)
		file.WriteString(s)
		//os.Stdout.WriteString(s)
	}

	// ... and finish with a newline
	file.WriteString("\n")
}

func NewSeparatedValuesOutput(args AppArgs, separator string) SeparatedValuesOutput {
	return SeparatedValuesOutput{
		args:      args,
		separator: separator,
		dest:      os.Stdout,
	}
}

func (o SeparatedValuesOutput) start(columns []string) {
	write_header_row(o.dest, o.separator, columns)
}

func (o SeparatedValuesOutput) add_row(columns []string, date string, column_to_lines_dict map[string]int) {
	write_row(o.dest, o.separator, columns, date, column_to_lines_dict)
}

func (o SeparatedValuesOutput) finish() {
	print_file_written(o.args.Output)
	//o.dest.close() TODO: For files
}

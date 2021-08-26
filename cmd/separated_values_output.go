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
	"io"
	"os"
)

type SeparatedValuesOutput struct {
	separator string
	dest      io.Writer
}

func write_header_row(writer io.Writer, separator string, columns []string) {
	// To get correct tab alignment, pad with spaces in place of a date
	// on the first row
	writer.write("          ") // for YYYY-MM-DD

	// Now write the columns
	for _, column := range columns {
		writer.write(fmt.Sprintf("%s%s", separator, column))
	}

	// ... and finish with a newline
	writer.write("\n")
}

func write_row(writer io.Writer, separator string, columns []string, date string, column_to_lines_dict map[string]int) {
	// Date
	writer.write(date)

	// Line count information
	for _, column := range columns {
		writer.write(fmt.Sprintf("%s%s", separator, column_to_lines_dict.get(column, 0)))
	}

	// ... and finish with a newline
	writer.write("\n")
}

func NewSeparatedValuesOutput(separator string) SeparatedValuesOutput {
	return SeparatedValuesOutput{separator: separator, dest: os.Stdout}
}

//	o.dest = open(args.output, 'w')

func (o *SeparatedValuesOutput) start(columns []string) {
	write_header_row(o.dest, o.separator, columns)
}

func (o *SeparatedValuesOutput) add_row(columns []string, date string, column_to_lines_dict map[string]int) {
	write_row(o.dest, o.separator, columns, date, column_to_lines_dict)
}

func (o *SeparatedValuesOutput) finish() {
	print_file_written(o.args.output)
	o.dest.close()
}

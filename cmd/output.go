package main

// Implement this to support formating the collected data in some kind of output format.
// Example output formats could be stdout printouts, a .tsv file, or a .png file with a graph.
type Output interface {

	// Called in the beginning of processing,
	// to announce what args to use for processing.
	start(columns []string)

	// Called each time a commit has been analyzed,
	// and it is time to present the data in the commit.
	add_row(date string, column_to_lines_dict map[string]int)

	// Called when the processing of commits is complete.
	// This is a good time to write output files to disk.
	finish()
}

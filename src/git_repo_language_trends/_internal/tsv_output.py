import sys

def write_header_row(writer, columns):
    # To get correct tab alignment, pad with spaces in place of a date
    # on the first row
    writer.write("          ")  # for YYYY-MM-DD

    # Now write the columns
    for column in columns:
        writer.write(f"\t{column}")

    # ... and finish with a newline
    writer.write("\n")


def write_row(writer, columns, date, column_to_lines_dict):
    # Date
    writer.write(date)

    # Line count information
    for column in columns:
        writer.write(f"\t{column_to_lines_dict.get(column, 0)}")

    # ... and finish with a newline
    writer.write("\n")


class TabSeparatedValuesStdoutOutput:
    """
    Implemented this to support formating the collected data in some kind of output format.
    Example output formats could be stdout printouts, a .tsv file, or a .png file with a graph.
    """

    # Called in the beginning of processing,
    # to announce what columns to use in the output.
    def start(self, columns):
        write_header_row(sys.stdout, columns)

    # Called each time a commit has been analyzed,
    # and it is time to present the data in the commit.
    def add_row(self, columns, date, column_to_lines_dict):
        write_row(sys.stdout, columns, date, column_to_lines_dict)

    # Called when the processing of commits is complete.
    # This is a good time to write output files to disk.
    def finish(self):
        pass

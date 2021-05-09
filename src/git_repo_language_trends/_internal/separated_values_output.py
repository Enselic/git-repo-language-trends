import sys

from .output import Output

"""
This module adds support for

  Tab-separated values

and

  Comma-separated values

output, i.e. .tsv and .csv file formats.
"""


def write_header_row(writer, separator, columns):
    # To get correct tab alignment, pad with spaces in place of a date
    # on the first row
    writer.write("          ")  # for YYYY-MM-DD

    # Now write the columns
    for column in columns:
        writer.write(f"{separator}{column}")

    # ... and finish with a newline
    writer.write("\n")


def write_row(writer, separator, columns, date, column_to_lines_dict):
    # Date
    writer.write(date)

    # Line count information
    for column in columns:
        writer.write(f"{separator}{column_to_lines_dict.get(column, 0)}")

    # ... and finish with a newline
    writer.write("\n")


class SeparatedValuesStdoutOutput(Output):

    def __init__(self, args, separator):
        super().__init__(args)
        self.separator = separator

    def start(self, columns):
        write_header_row(sys.stdout, self.separator, columns)

    def add_row(self, columns, date, column_to_lines_dict):
        write_row(sys.stdout, self.separator, columns, date, column_to_lines_dict)

    def finish(self):
        pass

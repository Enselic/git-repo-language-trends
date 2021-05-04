import sys
import os

import matplotlib.pyplot as plt
import numpy as np

from .output import Output

class SvgOutput(Output):
    columns = None
    rows = []

    def __init__(self, args):
        super().__init__(args)

    # Called in the beginning of processing,
    # to announce what columns to use in the output.
    def start(self, columns):
        self.columns = columns

    # Called each time a commit has been analyzed,
    # and it is time to present the data in the commit.
    def add_row(self, columns, date, column_to_lines_dict):
        self.rows.append((date, column_to_lines_dict))

    # Called when the processing of commits is complete.
    # This is a good time to write output files to disk.
    def finish(self):
        dates = list(map(lambda row: row[0], self.rows))

        line_counts = []
        for column in self.columns:
            line_count = list(map(lambda row: row[1].get(column, 0), self.rows))
            line_counts.append(line_count)

        s = np.vstack(line_counts)

        plt.style.use(self.args.svg_style)
        plt.figure(figsize=(self.args.svg_width_inches, self.args.svg_height_inches))
        plt.stackplot(dates, s, labels=self.columns)
        plt.legend(loc='upper left')
        plt.ylabel("Total (stacked) line count")
        plt.title(f"{os.path.basename(os.getcwd())} language trends")
        plt.tick_params(axis='x', labelrotation=45)
        plt.tight_layout()

        plt.savefig("output.svg")

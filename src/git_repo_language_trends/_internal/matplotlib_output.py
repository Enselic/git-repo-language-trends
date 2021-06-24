from .output import Output


class MatplotlibOutput(Output):
    def __init__(self, args):
        super().__init__(args)
        self.columns = None
        self.rows = []

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
        # Import right before needed to significantly
        # improve startup time of this tool
        import os
        import matplotlib.pyplot as plt
        import numpy as np
        from .utils import print_file_written

        dates = list(map(lambda row: row[0], self.rows))

        line_counts = []
        for column in self.columns:
            line_count = list(
                map(lambda row: row[1].get(column, 0), self.rows))
            line_counts.append(line_count)

        s = np.vstack(line_counts)

        # See https://matplotlib.org/stable/gallery/style_sheets/style_sheets_reference.html
        matplotlib_style = "dark_background"
        if self.args.style == "light":
            matplotlib_style = "default"

        width_inches, height_inches = self.args.size_inches.split(':')
        width_inches = float(width_inches)
        height_inches = float(height_inches)

        plt.style.use(matplotlib_style)
        plt.figure(figsize=(width_inches, height_inches))
        plt.stackplot(dates, s, labels=self.columns)
        plt.legend(loc='upper left')
        if self.args.relative:
            plt.ylabel("Language usage %")
        else:
            plt.ylabel("Total (stacked) line count")
        plt.title(f"{os.path.basename(os.getcwd())} language trends")
        plt.tick_params(axis='x', labelrotation=45)
        plt.tight_layout()

        plt.savefig(self.args.output)
        print_file_written(self.args.output)

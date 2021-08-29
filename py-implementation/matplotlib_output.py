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

        barely_visible_color = "#505050"
        matplotlib_style = "dark_background"
        if self.args.style == "light":
            matplotlib_style = "default"
            barely_visible_color = "#c0c0c0"

        # See https://matplotlib.org/stable/gallery/style_sheets/style_sheets_reference.html
        plt.style.use(matplotlib_style)

        fig, ax = plt.subplots(figsize=self.args.size_inches)

        ax.stackplot(dates, s, labels=self.columns)
        title = f"{os.path.basename(os.getcwd())} language trends"

        # Don't leave spaces to the left and right of the plot
        ax.set_xlim([dates[0], dates[-1]])

        if self.args.relative:
            y_label = "Language usage %"
            ax.set_ylim([0, 100])
        else:
            y_label = "Total line count"
            title = title + ", stacked area plot"
        ax.set_ylabel(y_label, fontsize=15)
        ax.set_title(title, fontsize=20)

        ax.legend(loc='upper left', fontsize=15)
        ax.grid(True, color=barely_visible_color, alpha=0.5)
        ax.tick_params(axis='x', labelrotation=45)
        # Don't use scientific notation for line counts
        ax.ticklabel_format(axis='y', style='plain')

        fig.autofmt_xdate()
        if not self.args.no_watermark:
            fig.text(
                0.5, 0,
                "Created with https://github.com/Enselic/git-repo-language-trends",
                fontsize=9,
                color=barely_visible_color,
                ha='center',
                va='bottom',
            )
        fig.tight_layout()

        fig.savefig(self.args.output)
        print_file_written(self.args.output)

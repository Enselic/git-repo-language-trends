class Output:
    """
    Implement this to support formating the collected data in some kind of output format.
    Example output formats could be stdout printouts, a .tsv file, or a .png file with a graph.
    """

    def __init__(self, args):
        self.args = args

    # Called in the beginning of processing,
    # to announce what args to use for processing.
    def start(self, columns):
        pass

    # Called each time a commit has been analyzed,
    # and it is time to present the data in the commit.
    def add_row(self, date, column_to_lines_dict):
        pass

    # Called when the processing of commits is complete.
    # This is a good time to write output files to disk.
    def finish(self):
        pass

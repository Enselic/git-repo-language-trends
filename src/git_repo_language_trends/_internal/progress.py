import sys
import time

RATE_LIMIT_INTERVAL_SECONDS = 0.1


class Progress:

    def __init__(self, args, total_commits):
        self.args = args

        self.current_commit = 1
        self.total_commits = total_commits

        self.last_print = None

    def print_state(self, current_file, total_files):
        if (self.args.no_progress):
            return

        if (not sys.stderr.isatty):
            return

        # If we recently printed, bail out. Always print if this is the last file we
        # are processig however, since otherwise output seems "incomplete" to a human.
        if self.is_rate_limited() and current_file < total_files:
            return

        if self.total_commits == 1:
            commit_part = ""
        else:
            # "commit  12/345 "
            commit_part = f"commit {padded_progress(self.current_commit, self.total_commits)} "

        # "file  67/890"
        file_part = f"file {padded_progress(current_file, total_files)}"

        # "Counting lines in commit  12/345 file  67/890"
        print(
            f"Counting lines in {commit_part}{file_part}\r",
            file=sys.stderr,
            end='',
        )

    # Avoid writing large amounts of data to stderr, which can slow down execution significantly
    def is_rate_limited(self):
        now = time.time()
        if self.last_print is not None and now < self.last_print + RATE_LIMIT_INTERVAL_SECONDS:
            return True
        self.last_print = now
        return False

    def commit_processed(self):
        self.current_commit += 1


def padded_progress(current_commit, total_commits):
    pad = len(str(total_commits))
    return f"{current_commit:>{pad}}/{total_commits}"

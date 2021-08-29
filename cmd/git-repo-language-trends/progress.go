package main

import (
	"os"
	"strconv"
)

const RATE_LIMIT_INTERVAL_SECONDS = 0.1


type Progress struct {
	args AppArgs,
	current_commit int,
	total_commits int,
	last_print *time.Time,
}

func NewProgress(args AppArgs, total_commits int) *Progress {
	return &Progress {
			args,
			1,
			total_commits,
			nil,
			}
}

func (self *Progress) print_state(current_file int, total_files int) {
	if (self.args.NoProgress) {
		return
	}

	// TODO:
	// if (os.Stderr.Istty) {
	// 	return
	// }

	// If we recently printed, bail out. Always print if this is the last file we
	// are processig however, since otherwise output seems "incomplete" to a human.
	if self.is_rate_limited() && current_file < total_files {
		return
	}

	if self.total_commits == 1 {
		commit_part = ""
	} else {
		// "commit  12/345 "
		commit_part = fmt.Sprintf("commit %s ", padded_progress(self.current_commit,self.total_commits))
	}

	// "file  67/890"
	file_part = fmt.Sprints("file %s", padded_progress(current_file,total_files))

	// "Counting lines in commit  12/345 file  67/890"
	os.Stderr.WriteString(fmt.Sprintf("Counting lines in %s%s\r"), commit_part, file_part)
}

// Avoid writing large amounts of data to stderr, which can slow down execution significantly
func (self *Progress) is_rate_limited() bool {
	now = time.Now()
	if self.last_print != nil && now.Sub(self.last_print).Seconds() < RATE_LIMIT_INTERVAL_SECONDS {
		return true
	}
	self.last_print = now
	return false
}

func (self *Progress) commit_processed(self) {
	self.current_commit += 1
}

func padded_progress(current_commit int, total_commits int) string {
    pad = len(strconv.FormatInt(total_commits))
	return fmt.Sprintf("%*d/%d", pad, current_commit, total_commits)
}

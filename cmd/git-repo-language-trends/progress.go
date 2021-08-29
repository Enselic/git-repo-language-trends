package main

import (
	"fmt"
	"os"
	"strconv"
	"time"
)

const RATE_LIMIT_INTERVAL_SECONDS = 0.1

type Progress struct {
	args           AppArgs
	current_commit int
	total_commits  int
	last_print     *time.Time
}

func NewProgress(args AppArgs, total_commits int) *Progress {
	return &Progress{
		args,
		1,
		total_commits,
		nil,
	}
}

func (p *Progress) print_state(current_file int, total_files int) {
	if p.args.NoProgress {
		return
	}

	// TODO:
	// if (os.Stderr.Istty) {
	// 	return
	// }

	// If we recently printed, bail out. Always print if this is the last file we
	// are processig however, since otherwise output seems "incomplete" to a human.
	if p.is_rate_limited() && current_file < total_files {
		return
	}

	var commit_part string
	if p.total_commits == 1 {
		commit_part = ""
	} else {
		// "commit  12/345 "
		commit_part = fmt.Sprintf("commit %s ", padded_progress(p.current_commit, p.total_commits))
	}

	// "file  67/890"
	var file_part = fmt.Sprintf("file %s", padded_progress(current_file, total_files))

	// "Counting lines in commit  12/345 file  67/890"
	os.Stderr.WriteString(fmt.Sprintf("Counting lines in %s%s\r", commit_part, file_part))
}

// Avoid writing large amounts of data to stderr, which can slow down execution significantly
func (p *Progress) is_rate_limited() bool {
	now := time.Now()
	if p.last_print != nil && now.Sub(*p.last_print).Seconds() < RATE_LIMIT_INTERVAL_SECONDS {
		return true
	}
	p.last_print = &now
	return false
}

func (p *Progress) clear() {
	if p.args.NoProgress {
		return
	}

	os.Stderr.WriteString("                                                                              \r")
}

func (p *Progress) commit_processed() {
	p.current_commit += 1
}

func padded_progress(current_commit int, total_commits int) string {
	pad := len(strconv.FormatInt(int64(total_commits), 10))
	return fmt.Sprintf("%*d/%d", pad, current_commit, total_commits)
}

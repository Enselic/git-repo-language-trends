# git-repo-language-trends
Prints tabulated data about programming language usage over time in a git
repository for a given set of file extensions.

Copy-paste the output into e.g. Google Sheets or Microsoft Excel to easily make
a graph. Stacked area chart is recommended.

# Example

Simply pass the file extensions of the languages you want the trend for. In the output, `2021w02` means "2021, week 02".
```
% git-repo-language-trends cpp rs
        cpp     rs
2021w03 0       245
2021w02 143     198
2021w01 386     27
```

# To Do

Performance is OK as is, but would probably be a lot faster with the (git2
crate)[https://github.com/rust-lang/git2-rs] to avoid spawning thousands of
child processes.

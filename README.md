# THIS IS AN OLD IMPLEMENTATION OF THIS TOOL BASED ON RUST

I started using Rust but am now using Python. Please use the 'main' branch
instead.

# git-repo-language-trends
Prints tabulated data about programming language usage over time in a git
repository.

Copy-paste the output into your favourite spreadsheet software to easily make a
graph. Stacked area chart is recommended.

# Example
Simply pass the file extensions of the languages you want the trend for.
```
% cd ~/src/your-project
% git-repo-language-trends .cpp .rs
                .cpp    .rs
2021-01-23      0       121
2021-01-22      120     107
2021-01-19      243     66
```
Then copy-paste the output into your favourite spreadsheet software and make a
graph.

# Installation
## Pre-built binaries
You can download pre-built binaries for **Linux**, **Mac** and **Windows** for the latest release [here](https://github.com/Enselic/git-repo-language-trends/releases).

## cargo install
If you have Rust and Cargo installed, all you need to do to fetch, build and
install the self-contained `git-repo-language-trends` crate is:
```
cargo install git-repo-language-trends
```

## From source
You can of course also clone this repo and then simply `cargo build` it if you have Rust and Cargo installed on your system.

# Performance
This program is fast. It counts ~5 000 000 lines / second on a high-end 2018
laptop on a large repository (with `--disable-progress-bar`). (For smaller
repositories, the number is lower.)

This is because the inner loop uses the Rust `libgit2`
[bindings](https://github.com/rust-lang/git2-rs). A regular shell script on a
fast 2018 laptop that uses `git show $COMMIT:$FILE` in the inner loop counts
only ~20 000 lines / second.

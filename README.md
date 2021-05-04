# git-repo-language-trends
Programming language usage over time in a git repository is plotted to an SVG
file, based on total line count for given file extensions.

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
Then open `output.svg`.

# Installation

Simply do
```
python3 -m pip install git-repo-language-trends
```

# Performance
This program is pretty fast, because it uses the pygit2 wrapper for the C
library libgit2. On a low end computer (with an **Intel(R) Celeron(R) J4005 CPU
@ 2.00GHz**) it counts ~400 000 lines per qsecond.

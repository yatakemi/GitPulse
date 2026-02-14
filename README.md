# GitPulse 💓

**GitPulse** is a lightweight, blazing fast CLI tool to measure and visualize git repository productivity. Built with Rust, it analyzes your commit history and generates interactive reports without needing heavy database dependencies.

![GitPulse Report Preview](https://github.com/yatakemi/GitPulse/raw/main/docs/preview.png?raw=true)

## Features

- 🚀 **Pure Rust**: Single binary, minimal dependencies, and extremely fast.
- 📊 **Interactive Dashboard**: Generates beautiful HTML reports with Chart.js.
    - **Timeline**: Visualize code changes (Added/Deleted) or commit counts over time.
    - **User Activity**: Deep dive into contributor metrics: **Total Changes**, **Branch Lead Time**, and **Review Metrics**.
    - **GitHub Reviews**: Comprehensive review analysis: **Reviews Assigned**, **Comments Given (Points made)**, and **Review Lead Time**.
    - **Drill-down Investigation**: Click on any user to see their full commit history, messages, and affected files (NEW!).
    - **User Filtering**: Toggle specific users on/off to analyze subsets of the team. State preserved in URL.
    - **User Grouping**: Define logical groups (e.g., Teams) for collective selection/deselection (NEW!).
    - **Trends & Forecasts**: 7-day moving average trend lines and **ML-based Velocity Forecasting** (NEW!).
    - **Distribution Analysis**: Analyze **Commit Size**, **Response Time**, and **Branch Lead Time** frequencies (NEW!).
    - **Context Switching**: Track **Directory Diversity** and **Inter-commit Intervals** to gauge fragmentation (NEW!).
    - **Team Health**: Track "Code Churn" (Refactoring/Rework), "Work Duration", and "Merge Reciprocity".
- 🎯 **Flexible Metrics**: Switch between Added, Deleted, Total lines, and Commit Count.
- 🔀 **Merge Commits**: Tracks merge commits (as 0 changes) and calculates **Branch Lead Time** (first commit to merge).
- 🤝 **User Unification**: Merge duplicate users (e.g., personal vs work email) and automatically handle GitHub noreply addresses.
- 🐳 **Portable**: Database-free (uses JSON for intermediate storage), making it easy to version control your stats.

## Installation

### Download Binary (Recommended)

Download the latest binary for your OS (macOS, Linux, Windows) from the [Releases page](https://github.com/yatakemi/GitPulse/releases).

### Build from Source

Requirements: Rust (cargo) installed.

```bash
git clone https://github.com/yatakemi/GitPulse.git
cd GitPulse
cargo build --release
```

The binary will be located at `target/release/gitpulse`.

## Usage

### 1. Collect Data
Analyze a git repository and save the raw statistics to a JSON file.

```bash
# Analyze the current directory
gitpulse collect --out stats.json

# Analyze a specific repository
gitpulse collect --repo /path/to/repo --out stats.json

# Analyze only merge commits
gitpulse collect --merges-only --out merge_stats.json

# Analyze with GitHub Review activity
# Requires 'gh' CLI authenticated or GITHUB_TOKEN env var
gitpulse collect --github --out stats_with_reviews.json

# Force fresh fetch from GitHub (ignore local cache)
gitpulse collect --github --no-cache --out stats_with_reviews.json
```

### 2. Visualize Data
Generate a report from the collected JSON data.

**HTML Report (Interactive Dashboard)**
```bash
gitpulse visualize --data stats.json --out report.html --format html
```
Then open `report.html` in your browser.

**CSV Export**
```bash
gitpulse visualize --data stats.json --out report.csv --format csv
```

## Configuration

You can customize GitPulse behavior by creating a `gitpulse.toml` file in your working directory.

### User Unification (Alias)

Unify multiple author names/emails into a single user.

```toml
[alias]
# Format: "email_or_name" = "Canonical Name"
"alice.personal@gmail.com" = "Alice"

# Base branches for filtering noise (NEW!)
# Merges FROM these branches INTO feature branches will be ignored
# to prevent noise from upstream changes.
base_branches = ["main", "master", "develop"]

# Commit filtering to exclude noise (NEW!)
[filter.commits]
max_lines = 2000        # Exclude commits changing > 2000 lines
max_files = 50          # Exclude commits changing > 50 files
ignore_messages = ["refactor", "cleanup", "formatting"] # Exclude if message contains

### User Grouping (NEW!)

Define logical groups of users for collective selection in the dashboard.

```toml
[groups]
"Backend Team" = ["Alice", "Bob"]
"Frontend Team" = ["Charlie", "Dave"]
```

**Note:** GitHub noreply emails are **automatically merged** to `username` by default.

### Initiative Tracking (Events)

Mark specific dates when you started a new initiative to assess its impact on productivity.

```toml
[[events]]
date = "2026-02-01"
name = "New Code Review Policy"

[[events]]
date = "2026-02-15"
name = "CI Pipeline Optimization"
```

### File & Directory Exclusion

Exclude specific files or directories (like lock files or dependencies) from the statistics and line counts.

```toml
# Files and directories to exclude from stats
exclude = [
    "/README.md",       # Matches only root README.md (NEW!)
    "pnpm-lock.yaml",   # Matches root pnpm-lock.yaml
    "secret.txt",       # Matches any secret.txt in any directory
    "target/",          # Matches everything inside target/ directory
    "vendor/*.c",       # Matches files ending with .c in vendor/
    "*.lock",           # Matches any file ending with .lock
]
```

Exclusion supports:
- **Root-relative match**: `/README.md` (If the pattern starts with `/`, it matches exactly from the repository root)
- **Filename match**: `secret.txt` (If the pattern has no `/`, it matches the filename in any directory)
- **Exact match**: `path/to/file.txt` (Matches the specific path from the repository root)
- **Directory prefix**: `node_modules/` (Excludes everything inside the directory)
- **Suffix wildcard**: `*.lock` (Excludes files ending with .lock)
- **Prefix wildcard**: `vendor/*` (Excludes paths starting with vendor/)

**Note:** Binary files (images, executables, etc.) are automatically excluded from line counts.

## Advanced Analysis (v0.6+)

GitPulse now includes advanced analysis features to help you understand your team's health and codebase quality:

### 🔥 File Hotspots
Identifies the top 20 most frequently modified files. Frequent changes to the same file often indicate:
- **God Class**: A class that does too much.
- **Instability**: Code that is fragile and requires constant fixing.
- **Architectural Bottleneck**: Core logic that everyone touches concurrently.

### 🔍 Commit Drill-down (v0.14+)
If you see a spike in changes or commits, simply click on the user's name in the activity table. GitPulse will display a detailed list of all commits for that period, including:
- **Commit Messages**: Understand the *why* behind the changes.
- **Affected Files**: See which modules were touched.
- **Granular Stats**: View the exact lines added/deleted for each individual commit.

### 🐙 GitHub Review Performance (v0.14+)
When using the `--github` flag, GitPulse uses GraphQL to provide high-resolution review metrics:
- **Reviews (Assigned)**: Count of Pull Requests where the user was requested as a reviewer.
- **Review Comments**: Count of points/comments made by the user.
- **Review Lead Time**: Calculates the time from the first review comment to the final merge.
- **Rework Rate**: Percentage of PRs that received a "Request Changes" review. High rates may indicate scope creep or communication gaps.
- **Review Depth**: Average comments per PR. Measures the thoroughness of the review process.
- **Time to First Response**: Average time from PR creation to the first review or comment.
- **Review Reciprocity Matrix**: A heatmap showing "who reviews whom" to identify silos and team dependencies.
- **PR Size vs Lead Time Correlation**: A scatter plot analyzing if larger PRs take significantly longer to merge, helping teams optimize PR size.

### 🔮 Velocity Forecasting (BETA)
GitPulse analyzes the last 4 weeks of commit velocity to predict future throughput.
- **Current Velocity**: Average commits per week.
- **Target Goal Estimation**: Set a target commit goal to see an estimated completion date based on your team's current pace.
- **Trend Analysis**: See if your team is accelerating or slowing down.

### 🧠 Context Switching & Fragmentation (NEW!)
Understand how multi-tasking and focus time evolve.
- **Unrelated Switches**: Detects when a developer jumps between unrelated modules in consecutive commits.
- **Time Fragmentation**: Measures the interval between commits. Short intervals suggest fragmented work (interruptions/meetings).
- **Deep Work**: Periods of low switches and healthy inter-commit intervals indicate successful deep focus.

### 📉 Initiative Impact Assessment
Evaluate the ROI of your process changes (e.g., CI/CD introduction, new review rules). Define an "event" in `gitpulse.toml` to see a before/after comparison.
- **Throughput Change**: Merged PRs/Week (Volume of work).
- **Process Stability**: Lead Time P50 (Median) vs P90 (Worst case).
- **Rework Impact**: Changes in Churn and Rework rates.
- **Visual Annotation**: Initiatives are marked with vertical dashed lines on the timeline charts.

## Architecture

GitPulse adopts a "Collector-Visualizer" pattern:
1.  **Collector**: Scans `git` history using `libgit2` and dumps raw commit stats to a JSON file.
2.  **Visualizer**: Reads the JSON, applies user unification rules, aggregates data, and renders it to the desired format (HTML/CSV).

## Development & Verification

To ensure the quality of changes, you can use the following methods for verification:

### 1. Automated Verification
Run the verification script to build the project and perform an end-to-end check using a minimal test repository.
```bash
./verify.sh
```

### 2. Comprehensive Data Test
Generate a sample repository with 30 days of activity and multiple users to verify advanced features like forecasting and filtering.
```bash
python3 generate_sample_repo.py
./target/release/gitpulse collect --repo sample_repo --out sample_stats.json
./target/release/gitpulse visualize --data sample_stats.json --out sample_report.html --format html
```

### 3. Unit Tests
Run standard Rust unit tests.
```bash
cargo test
```

## License

MIT

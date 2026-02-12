# GitPulse 💓

**GitPulse** is a lightweight, blazing fast CLI tool to measure and visualize git repository productivity. Built with Rust, it analyzes your commit history and generates interactive reports without needing heavy database dependencies.

![GitPulse Report Preview](https://github.com/yatakemi/GitPulse/raw/main/docs/preview.png?raw=true)

## Features

- 🚀 **Pure Rust**: Single binary, minimal dependencies, and extremely fast.
- 📊 **Interactive Dashboard**: Generates beautiful HTML reports with Chart.js.
    - **Timeline**: Visualize code changes (Added/Deleted) or commit counts over time.
    - **User Activity**: Deep dive into contributor metrics: **Total Changes**, **Avg Lead Time**, and **Top Directories** (AI-detected expertise).
    - **User Filtering**: Toggle specific users on/off to analyze subsets of the team. State preserved in URL.
    - **Trends**: 7-day moving average trend lines to see the big picture.
    - **Heatmaps**: "Punch card" style activity analysis (Hour vs Day).
    - **Team Health**: Track "Code Churn" (Refactoring/Rework) and "Work Duration" trends.
    - **File Hotspots**: Identify frequently modified files that might need architectural attention.
- 🎯 **Flexible Metrics**: Switch between Added, Deleted, Total lines, and Commit Count.
- 🔀 **Merge Commits**: Tracks merge commits (as 0 changes) and displays them separately in the dashboard.
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

# Merge duplicate users
"alice.personal@gmail.com" = "Alice"
"Bob_Work" = "Bob"

# Fix typos
"Charly" = "Charlie"
```

**Note:** GitHub noreply emails (e.g., `123456+username@users.noreply.github.com`) are **automatically merged** to `username` by default.

### File & Directory Exclusion

Exclude specific files or directories (like lock files or dependencies) from the statistics and line counts.

```toml
# Files and directories to exclude from stats
exclude = [
    "pnpm-lock.yaml",
    "package-lock.json",
    "target/",
    "node_modules/",
    "vendor/*.c",
]
```

Exclusion supports:
- **Exact match**: `pnpm-lock.yaml`
- **Directory prefix**: `node_modules/` (excludes everything inside)
- **Wildcards**: Basic support for `*` at the beginning or end (e.g., `*.lock`, `vendor/*`).

## Advanced Analysis (v0.6+)

GitPulse now includes advanced analysis features to help you understand your team's health and codebase quality:

### 🔥 File Hotspots
Identifies the top 20 most frequently modified files. Frequent changes to the same file often indicate:
- **God Class**: A class that does too much.
- **Instability**: Code that is fragile and requires constant fixing.
- **Architectural Bottleneck**: Core logic that everyone touches concurrently.

### 🧘 Work Habits & Team Health
- **Est. Daily Work Duration**: Calculates the time span between the first and last commit of the day for each user. Helps identify potential burnout or unhealthy working hours.
- **Code Churn (Volatility)**: Measures the ratio of "Rework" (edits to existing code) vs "New Work". High churn might indicate unclear requirements or technical debt.
- **Health Trends**: A dedicated chart tracking Churn Rate and Work Duration over time to spot negative trends early.

### ℹ️ Interactive Explanations
The dashboard includes tooltips (info icons) for each chart, explaining what to look for and how to interpret the data, making it easier for non-technical stakeholders to understand.

## Architecture

GitPulse adopts a "Collector-Visualizer" pattern:
1.  **Collector**: Scans `git` history using `libgit2` and dumps raw commit stats to a JSON file.
2.  **Visualizer**: Reads the JSON, applies user unification rules, aggregates data, and renders it to the desired format (HTML/CSV).

## License

MIT

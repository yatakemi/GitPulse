# GitPulse 💓

**GitPulse** is a lightweight, blazing fast CLI tool to measure and visualize git repository productivity. Built with Rust, it analyzes your commit history and generates interactive reports without needing heavy database dependencies.

![GitPulse Report Preview](https://github.com/yatakemi/GitPulse/raw/main/docs/preview.png?raw=true)

## Features

- 🚀 **Pure Rust**: Single binary, minimal dependencies, and extremely fast.
- 📊 **Interactive Dashboard**: Generates beautiful HTML reports with Chart.js.
    - **Timeline**: Visualize code changes (Added/Deleted) or commit counts over time.
    - **Trends**: 7-day moving average trend lines to see the big picture.
    - **Heatmaps**: "Punch card" style activity analysis (Hour vs Day).
    - **Productivity**: Day of Week analysis and Commit Size distribution.
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

## Configuration (User Unification)

You can unify multiple author names/emails into a single user by creating a `gitpulse.toml` file in your working directory.

```toml
[alias]
# Format: "email_or_name" = "Canonical Name"

# Merge duplicate users
"alice.personal@gmail.com" = "Alice"
"Bob_Work" = "Bob"

# Fix typos
"Charly" = "Charlie"
```

**Note:** GitHub noreply emails (e.g., `123456+username@users.noreply.github.com`) are **automatically merged** to `username` by default, so you don't need to define them manually unless you want to rename them.

## Architecture

GitPulse adopts a "Collector-Visualizer" pattern:
1.  **Collector**: Scans `git` history using `libgit2` and dumps raw commit stats to a JSON file.
2.  **Visualizer**: Reads the JSON, applies user unification rules, aggregates data, and renders it to the desired format (HTML/CSV).

## License

MIT

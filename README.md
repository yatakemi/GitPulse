# GitPulse 💓

**GitPulse** is a lightweight, blazing fast CLI tool to measure and visualize git repository productivity. Built with Rust, it analyzes your commit history and generates interactive reports without needing heavy database dependencies.

## Features

- 🚀 **Pure Rust**: Single binary, minimal dependencies, and extremely fast.
- 📊 **Interactive Charts**: Generates beautiful HTML reports with Chart.js.
- 💾 **Data Lake Architecture**: Separates collection (`collect`) from visualization (`visualize`) using JSON.
- 🎯 **Flexible Metrics**: Switch between Added, Deleted, and Total lines of code changes.
- 🐳 **Portable**: Database-free (uses JSON for intermediate storage), making it easy to version control your stats.

## Installation

### Build from Source

Requirements: Rust (cargo) installed.

```bash
git clone <this-repo-url>
cd gitpulse
cargo build --release
```

The binary will be located at `target/release/gitpulse`.

## Usage

### 1. Collect Data
Analyze a git repository and save the raw statistics to a JSON file.

```bash
# Analyze the current directory
./target/release/gitpulse collect --out stats.json

# Analyze a specific repository
./target/release/gitpulse collect --repo /path/to/repo --out stats.json
```

### 2. Visualize Data
Generate a report from the collected JSON data.

**HTML Report (Interactive Graph)**
```bash
./target/release/gitpulse visualize --data stats.json --out report.html --format html
```
Then open `report.html` in your browser.

**CSV Export**
```bash
./target/release/gitpulse visualize --data stats.json --out report.csv --format csv
```

## Architecture

GitPulse adopts a "Collector-Visualizer" pattern:
1.  **Collector**: Scans `git` history using `libgit2` and dumps raw commit stats to a JSON file.
2.  **Visualizer**: Reads the JSON, aggregates user data (daily/weekly), and renders it to the desired format.

## License

MIT

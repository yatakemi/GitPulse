#!/bin/bash
set -e

# Build the project
cargo build --release

# Create a test directory
mkdir -p test_repo
cd test_repo
git init
git config user.name "Tester"
git config user.email "tester@example.com"

# Create some commits
echo "Hello" > file.txt
git add file.txt
git commit -m "Initial commit"

echo "World" >> file.txt
git add file.txt
git commit -m "Second commit"

# Run collect
../target/release/git-product-perf collect --repo . --out ../stats.json

# Run visualize
../target/release/git-product-perf visualize --data ../stats.json --out ../report.html --format html
../target/release/git-product-perf visualize --data ../stats.json --out ../report.csv --format csv

# Check outputs
if [ -f "../stats.json" ]; then
    echo "SUCCESS: stats.json created"
else
    echo "FAILURE: stats.json not found"
    exit 1
fi

if [ -f "../report.html" ]; then
    echo "SUCCESS: report.html created"
else
    echo "FAILURE: report.html not found"
    exit 1
fi

if [ -f "../report.csv" ]; then
    echo "SUCCESS: report.csv created"
else
    echo "FAILURE: report.csv not found"
    exit 1
fi

echo "All tests passed!"

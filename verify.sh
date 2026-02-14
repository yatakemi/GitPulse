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
../target/release/gitpulse collect --repo . --out ../stats.json

# Run visualize
../target/release/gitpulse visualize --data ../stats.json --out ../report.html --format html
../target/release/gitpulse visualize --data ../stats.json --out ../report.csv --format csv

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

# JavaScript Syntax Check
echo "🔍 Validating JavaScript syntax..."
if command -v node > /dev/null; then
    # Check all JS files
    for js_file in ../src/*.js; do
        if [ -f "$js_file" ]; then
            echo "Checking $js_file..."
            # Create a temporary version without Tera templates
            sed 's/{{.*}}/null/g' "$js_file" > "${js_file}.tmp.js"
            node --check "${js_file}.tmp.js"
            rm "${js_file}.tmp.js"
        fi
    done
    echo "SUCCESS: JavaScript syntax is valid for all files"
else
    echo "SKIP: node not found, skipping JS syntax check"
fi

# JavaScript Integrity Check
echo "🔍 Checking JavaScript integrity in report.html..."
CHART_CONTEXTS=$(grep -oE "new Chart\([^,)]+" "../report.html" | cut -d'(' -f2)
for ctx in $CHART_CONTEXTS; do
    if ! grep -q "const $ctx =" "../report.html"; then
        echo "FAILURE: JavaScript error detected! Variable '$ctx' is used in 'new Chart()' but not defined with 'const $ctx ='."
        exit 1
    fi
done
echo "SUCCESS: JavaScript integrity check passed"

echo "All tests passed!"

#!/bin/bash
set -e # Exit immediately if a command exits with a non-zero status.

echo "🚀 Building GitPulse..."
if cargo build --release; then
    echo "✅ Build successful!"
else
    echo "❌ Build failed!"
    exit 1
fi

echo "📊 Generating report..."
./target/release/gitpulse visualize --data sample_stats.json --out sample_report.html --format html

echo "🌍 Opening report..."
open sample_report.html

#!/bin/bash
set -e

# Usage: ./release.sh v0.13.x "commit message"

if [ -z "$1" ] || [ -z "$2" ]; then
    echo "Usage: ./release.sh <tag_name> <commit_message>"
    echo "Example: ./release.sh v0.13.12 'feat: add new feature'"
    exit 1
fi

TAG_NAME=$1
COMMIT_MSG=$2

echo "🧪 Running unit tests..."
cargo test

echo "🔍 Running verification script..."
./verify.sh

echo "✅ All tests passed! Proceeding to commit..."

# Stage all changes
git add .

# Commit if there are changes
if ! git diff-index --quiet HEAD --; then
    git commit -m "$COMMIT_MSG"
else
    echo "No changes to commit, checking if tag already exists..."
fi

# Create tag
if git rev-parse "$TAG_NAME" >/dev/null 2>&1; then
    echo "⚠️ Tag $TAG_NAME already exists. Skipping tag creation."
else
    echo "🏷️ Creating tag $TAG_NAME..."
    git tag "$TAG_NAME"
fi

echo "🚀 Pushing to remote..."
git push origin main
git push origin "$TAG_NAME"

echo "🎉 Release $TAG_NAME complete!"

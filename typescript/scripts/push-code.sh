#!/bin/bash

# Git Push Script
# This script runs all checks, commits changes, and pushes to repository

set -e  # Exit on any error

# Check if there are any changes to commit
if [ -z "$(git status --porcelain)" ]; then
    echo "📝 No changes to commit"
    echo "🚀 Pushing existing commits to repository..."
    git push
    echo "✅ Push completed successfully!"
    exit 0
fi

echo "🔍 Running build and type checks..."
./scripts/build-check.sh

echo "📝 Adding all changes to git..."
git add .

echo "💬 Enter commit message:"
read -r commit_message

if [ -z "$commit_message" ]; then
    echo "❌ Commit message cannot be empty"
    exit 1
fi

echo "📦 Committing changes..."
git commit -m "$commit_message"

echo "🚀 Pushing to repository..."
git push

echo "✅ Code successfully pushed to repository!"
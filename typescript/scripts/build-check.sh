#!/bin/bash

# Build and Type Check Script
# This script runs linting, type checking, and builds the project

set -e  # Exit on any error

echo "🔍 Running ESLint..."
npm run lint

echo "🔧 Running TypeScript type check..."
npm run type-check

echo "🏗️  Building the project..."
npm run build

echo "✅ All checks passed! Build completed successfully."
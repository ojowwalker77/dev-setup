#!/bin/bash

# Automated Setup Script for Development Template
# Usage: curl -sSL https://raw.githubusercontent.com/ojowwalker77/dev-setup/main/typescript/setup.sh | bash

set -e

REPO_BASE="https://raw.githubusercontent.com/ojowwalker77/dev-setup/main/typescript"

echo "Setting up development environment..."

# Check if we're in a git repository
if [ ! -d ".git" ]; then
    echo "Initializing git repository..."
    git init
fi

# Create necessary directories
mkdir -p .husky scripts

# Download configuration files
echo "Downloading configuration files..."
curl -sSL "${REPO_BASE}/configs/eslint.config.js" -o eslint.config.js
curl -sSL "${REPO_BASE}/configs/vite.config.ts" -o vite.config.ts

# Download TypeScript configurations (with backup)
echo "Setting up TypeScript configurations..."
if [ -f "tsconfig.json" ]; then
    echo "Backing up existing tsconfig.json to tsconfig.json.backup"
    mv tsconfig.json tsconfig.json.backup
fi
curl -sSL "${REPO_BASE}/templates/tsconfig.json" -o tsconfig.json
curl -sSL "${REPO_BASE}/templates/tsconfig.app.json" -o tsconfig.app.json
curl -sSL "${REPO_BASE}/templates/tsconfig.node.json" -o tsconfig.node.json

# Download package.json template
echo "Downloading package.json template..."
if [ -f "package.json" ]; then
    echo "Backing up existing package.json to package.json.backup"
    mv package.json package.json.backup
fi
curl -sSL "${REPO_BASE}/templates/package.json" -o package.json

# Download dev script
echo "Downloading dev script..."
curl -sSL "${REPO_BASE}/dev" -o dev
chmod +x dev

# Download other scripts
echo "Downloading utility scripts..."
mkdir -p scripts
curl -sSL "${REPO_BASE}/scripts/build.sh" -o scripts/build.sh 2>/dev/null || true
curl -sSL "${REPO_BASE}/scripts/lint.sh" -o scripts/lint.sh 2>/dev/null || true
chmod +x scripts/*.sh 2>/dev/null || true

# Install dependencies
echo "Installing dependencies..."
npm install

# Initialize Husky
echo "Setting up Husky git hooks..."
npx husky init

# Download and setup pre-commit hook
curl -sSL "${REPO_BASE}/.husky/pre-commit" -o .husky/pre-commit
chmod +x .husky/pre-commit

echo ""
echo "Development environment setup completed!"
echo ""
echo "Next steps:"
echo "1. Run 'npm run lint' to test linting"
echo "2. Run 'npm run type-check' to test TypeScript"
echo "3. Run './dev' to start the interactive menu"
echo "4. Add alias to ~/.zshrc: alias dev='./dev'"
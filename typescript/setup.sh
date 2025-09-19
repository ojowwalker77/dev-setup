#!/bin/bash

# Automated Setup Script for Development Template
# Run this script in your new project directory

set -e

echo "🚀 Setting up development environment..."

# Check if we're in a git repository
if [ ! -d ".git" ]; then
    echo "📝 Initializing git repository..."
    git init
fi

# Copy configuration files
echo "📋 Copying configuration files..."
cp configs/eslint.config.js ./
cp configs/vite.config.ts ./

# Copy TypeScript configurations (with backup)
echo "🔧 Setting up TypeScript configurations..."
if [ -f "tsconfig.json" ]; then
    echo "⚠️  Backing up existing tsconfig.json to tsconfig.json.backup"
    mv tsconfig.json tsconfig.json.backup
fi
cp templates/tsconfig*.json ./

# Install dependencies
echo "📦 Installing development dependencies..."
npm install --save-dev \
    @eslint/js@^9.9.0 \
    @types/node@^22.5.5 \
    @types/react@^18.3.0 \
    @types/react-dom@^18.3.0 \
    @vitejs/plugin-react-swc@^3.5.0 \
    eslint@^9.9.0 \
    eslint-plugin-react-hooks@^5.1.0-rc.0 \
    eslint-plugin-react-refresh@^0.4.9 \
    globals@^15.9.0 \
    husky@^9.1.7 \
    lint-staged@^16.1.6 \
    typescript@^5.5.3 \
    typescript-eslint@^8.0.1 \
    vite@^5.4.1

# Initialize Husky
echo "🐕 Setting up Husky git hooks..."
npx husky init

# Copy pre-commit hook
cp .husky/pre-commit .husky/
chmod +x .husky/pre-commit

# Make scripts executable
echo "🔨 Making scripts executable..."
chmod +x dev
chmod +x scripts/*.sh

# Update package.json with scripts (this would need manual intervention)
echo "📝 Please manually add the following to your package.json:"
echo ""
echo "Scripts section:"
cat << 'EOF'
{
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "build:dev": "vite build --mode development",
    "lint": "eslint .",
    "lint:fix": "eslint . --fix",
    "type-check": "tsc --noEmit",
    "preview": "vite preview",
    "prepare": "husky"
  },
  "lint-staged": {
    "*.{ts,tsx,js,jsx}": [
      "eslint --fix"
    ]
  }
}
EOF

echo ""
echo "✅ Development environment setup completed!"
echo ""
echo "Next steps:"
echo "1. Update your package.json with the scripts shown above"
echo "2. Run 'npm run lint' to test linting"
echo "3. Run 'npm run type-check' to test TypeScript"
echo "4. Run './dev' to start the interactive menu"
echo "5. Add alias to ~/.zshrc: alias dev='./dev'"
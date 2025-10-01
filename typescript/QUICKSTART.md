# Quick Start Guide

## One-Command Setup

```bash
# In your new project directory
curl -sSL https://raw.githubusercontent.com/your-username/dev-setup-template/main/setup.sh | bash
```

Or manually:

```bash
# 1. Clone or copy this template
git clone <your-repo-url> temp-setup
cp -r temp-setup/* your-project/
cp temp-setup/.husky your-project/
rm -rf temp-setup

# 2. Run setup script
cd your-project
./setup.sh
```

## Verify Setup

```bash
# Test linting
npm run lint

# Test type checking
npm run type-check

# Test interactive menu
./dev
```

## Daily Usage

```bash
# Start development
./dev

# Or use individual commands
npm run dev          # Start dev server
npm run lint:fix     # Fix linting issues
npm run build        # Build project
./scripts/push-code.sh  # Commit and push
```

## One-Time Configuration

Add to `~/.zshrc` or `~/.bashrc`:
```bash
alias dev="./dev"
```

Then: `source ~/.zshrc`

Now you can just type `dev` anywhere in your project!

# Reusable Development Setup Template

A complete development workflow template with linting, type checking, git hooks, and a modern Rust-based CLI for React/TypeScript projects.

## 🚀 Quick Setup

### 1. Install Rust

```bash
# Install rustup (the Rust toolchain installer)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Build the dev-cli

```bash
# Navigate to the dev-cli directory
cd ../dev-cli

# Build the CLI
cargo build
```

### 3. Run the dev-cli

```bash
# Run the CLI from the typescript directory
./dev
```

## 📋 What's Included

### Scripts

- **`./dev`** - A wrapper script that runs the Rust-based `dev-cli`.

### dev-cli (Rust)

The new `dev-cli` is a modern, fast, and interactive CLI for development tasks.

- **Interactive TUI**: A beautiful and intuitive terminal user interface for selecting commands.
- **AI Error Analysis**: Integrated with Gemini for intelligent error analysis.
- **Commands**:
    - `start`: Start the development server.
    - `build`: Run a build and type check.
    - `push`: Commit and push changes.
    - `lint`: Run linting only.
    - `typecheck`: Run type check only.
    - `analyze`: Analyze errors with AI.
    - `setup`: Configure the AI integration.

### Features

- ✅ **Modern Rust-based CLI**
- ✅ **Interactive TUI**
- ✅ **AI-powered error analysis** with Gemini
- ✅ **Strict linting** with auto-fix
- ✅ **TypeScript type checking**
- ✅ **Pre-commit hooks** with lint-staged
- ✅ **Automated build validation**
- ✅ **Path aliases** (`@/` imports)
- ✅ **Git workflow automation**

## 🛠️ Usage

### Development Menu

```bash
./dev
```

This will launch the interactive TUI. Use the arrow keys to navigate, `Enter` to select, and `q` to quit.

### 🤖 AI Error Fixing Features

When Gemini AI is configured, the CLI provides intelligent error analysis:

- **Automatic Error Detection**: Captures errors from build, lint, and type-checking commands.
- **Smart AI Analysis**: Provides solution suggestions for errors.

#### AI Menu Options
- **Setup AI Error Fixing**: Configure your Gemini API key.
- **AI Debug Mode**: Analyze the most recent errors.

## 📝 License

This template is free to use across all your projects.

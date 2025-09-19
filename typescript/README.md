# Reusable Development Setup Template

A complete development workflow template with linting, type checking, git hooks, and automated scripts for React/TypeScript projects.

## 🚀 Quick Setup

### 1. Copy Template Files to Your Project

```bash
# Copy all files to your project root
cp -r dev-setup-template/* your-project/
cp dev-setup-template/.husky your-project/
```

### 2. Install Dependencies

```bash
npm install --save-dev @eslint/js@^9.9.0 @types/node@^22.5.5 @types/react@^18.3.0 @types/react-dom@^18.3.0 @vitejs/plugin-react-swc@^3.5.0 eslint@^9.9.0 eslint-plugin-react-hooks@^5.1.0-rc.0 eslint-plugin-react-refresh@^0.4.9 globals@^15.9.0 husky@^9.1.7 lint-staged@^16.1.6 typescript@^5.5.3 typescript-eslint@^8.0.1 vite@^5.4.1
```

### 3. Copy Configuration Files

```bash
# Copy config files to your project root
cp configs/eslint.config.js ./
cp configs/vite.config.ts ./

# Copy TypeScript configs (merge with existing if needed)
cp templates/tsconfig*.json ./
```

### 4. Update Package.json

Add these scripts and configuration to your `package.json`:

```json
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
```

### 5. Initialize Husky and Git Hooks

```bash
# Initialize git (if not already done)
git init

# Initialize Husky
npx husky init

# Copy pre-commit hook
cp .husky/pre-commit .husky/
chmod +x .husky/pre-commit
```

### 6. Make Scripts Executable

```bash
chmod +x dev
chmod +x scripts/*.sh
```

### 7. Setup Shell Alias (Optional)

Add to your `~/.zshrc` or `~/.bashrc`:

```bash
alias dev="./dev"
```

Then run `source ~/.zshrc` to activate.

### 8. Setup AI Error Fixing (Optional)

```bash
# Run the dev script and choose option 6 to setup AI
./dev

# Or manually create the config file
echo 'GEMINI_API_KEY="your-api-key-here"' > ~/.dev-config
chmod 600 ~/.dev-config
```

Get your free Gemini API key from: https://aistudio.google.com/app/apikey

## 📋 What's Included

### Scripts

- **`./dev`** - Interactive development menu with AI error fixing
- **`./scripts/build-check.sh`** - Complete build validation
- **`./scripts/push-code.sh`** - Automated git workflow

### Configuration Files

- **`eslint.config.js`** - Strict ESLint rules for React/TypeScript
- **`vite.config.ts`** - Vite configuration with path aliases
- **`tsconfig*.json`** - TypeScript configurations
- **`.husky/pre-commit`** - Git pre-commit hooks

### Features

- ✅ **Strict linting** with auto-fix
- ✅ **TypeScript type checking**
- ✅ **Pre-commit hooks** with lint-staged
- ✅ **Interactive dev menu**
- ✅ **Automated build validation**
- ✅ **Path aliases** (`@/` imports)
- ✅ **Git workflow automation**
- 🤖 **AI-powered error analysis** with Gemini 2.5 Flash
- 🔧 **Smart error suggestions** for build/lint/type errors
- 📊 **Project context awareness** for better AI analysis

## 🛠️ Usage

### Development Menu

```bash
./dev
```

Provides interactive menu with options:
1. Start development server
2. Run build and type check
3. Commit and push changes
4. Run linting only
5. Run type check only
6. Setup AI Error Fixing (Gemini) *[or AI Debug Mode if configured]*
7. Reconfigure AI settings *[only when AI is enabled]*
8. Exit *[7 if AI not enabled]*

### 🤖 AI Error Fixing Features

When Gemini AI is configured, the script provides intelligent error analysis:

#### Automatic Error Detection
- **Build errors**: Automatically captures TypeScript compilation errors
- **Lint errors**: Captures ESLint rule violations and suggestions
- **Type errors**: Analyzes TypeScript type checking issues

#### Smart AI Analysis
- **Context-aware**: Sends project dependencies, TypeScript config, and recent commits
- **Error categorization**: Identifies error types (build, lint, type)
- **Solution suggestions**: Provides step-by-step fixes and code examples
- **Best practices**: Suggests prevention strategies

#### AI Menu Options
- **Option 6**: AI Debug Mode - Analyze most recent errors
- **Option 7**: Reconfigure API key or setup

#### Example AI Workflow
```bash
./dev
# Choose option 4 (Run linting only)
# If errors occur: "❗ Linting errors detected. Would you like AI analysis? (Y/n)"
# AI provides detailed analysis with:
#   - Error explanation
#   - Step-by-step solution
#   - Exact code fixes
#   - Prevention tips
```

### Manual Commands

```bash
# Build validation
./scripts/build-check.sh

# Git workflow
./scripts/push-code.sh

# Individual checks
npm run lint
npm run type-check
npm run build
```

## 🔧 Customization

### ESLint Rules

Edit `eslint.config.js` to modify linting rules:

```javascript
rules: {
  // Add your custom rules here
  "no-console": "warn",
  "@typescript-eslint/no-unused-vars": "error",
}
```

### TypeScript Configuration

Modify `tsconfig.app.json` for your project needs:

```json
{
  "compilerOptions": {
    "strict": true,  // Enable for stricter type checking
    "noUnusedLocals": true,  // Enable to catch unused variables
  }
}
```

### Vite Configuration

Update `vite.config.ts` for your setup:

```typescript
export default defineConfig({
  server: {
    port: 3000,  // Change default port
  },
  // Add your custom config
});
```

### Git Hooks

Modify `.husky/pre-commit` to customize pre-commit checks:

```bash
npx lint-staged
npm run type-check
npm run test  # Add if you have tests
```

## 📦 Project Structure

```
your-project/
├── dev                     # Interactive dev menu
├── scripts/
│   ├── build-check.sh     # Build validation script
│   └── push-code.sh       # Git workflow script
├── .husky/
│   └── pre-commit         # Git pre-commit hook
├── eslint.config.js       # ESLint configuration
├── vite.config.ts         # Vite configuration
├── tsconfig.json          # Main TypeScript config
├── tsconfig.app.json      # App TypeScript config
├── tsconfig.node.json     # Node TypeScript config
└── package.json           # Updated with scripts
```

## 🚨 Troubleshooting

### ESLint Issues

```bash
# Fix most issues automatically
npm run lint:fix

# Check specific files
npx eslint src/specific-file.ts
```

### TypeScript Issues

```bash
# Run type checking
npm run type-check

# Check specific files
npx tsc --noEmit src/specific-file.ts
```

### Git Hook Issues

```bash
# Make sure hooks are executable
chmod +x .husky/pre-commit

# Test pre-commit hook manually
./.husky/pre-commit
```

### Path Alias Issues

Ensure your `tsconfig.json` and `vite.config.ts` both have:

```typescript
// vite.config.ts
resolve: {
  alias: {
    "@": path.resolve(__dirname, "./src"),
  },
}

// tsconfig.json
"paths": {
  "@/*": ["./src/*"]
}
```

## 🔄 Updates

To update this template in your projects:

1. Pull latest changes from this repository
2. Compare and merge any new configurations
3. Update dependencies as needed
4. Test all functionality before committing

## 📝 License

This template is free to use across all your projects.
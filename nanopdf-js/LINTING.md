# Code Quality & Linting Setup

This project uses **ESLint** and **Prettier** with a comprehensive suite of professional plugins to ensure maximum code quality and consistency.

## 🛠️ Tools Installed

### Core Tools
- **ESLint 9.x** - Latest ESLint with flat config format
- **Prettier 3.x** - Opinionated code formatter
- **TypeScript ESLint** - TypeScript-specific linting rules

### Professional Plugins (9 Total)

1. **@typescript-eslint** - TypeScript-specific rules and type checking
2. **eslint-plugin-import** - Import/export organization and validation
3. **eslint-plugin-jsdoc** - JSDoc comment validation
4. **eslint-plugin-node** - Node.js best practices
5. **eslint-plugin-prettier** - Prettier integration with ESLint
6. **eslint-plugin-promise** - Promise best practices
7. **eslint-plugin-security** - Security vulnerability detection
8. **eslint-plugin-sonarjs** - SonarJS code quality rules
9. **eslint-plugin-unicorn** - Awesome ESLint rules for better code

## 📝 Scripts Available

```bash
# Lint all TypeScript/JavaScript files
pnpm lint

# Auto-fix linting issues
pnpm lint:fix

# Format all files with Prettier
pnpm format

# Check formatting without modifying files
pnpm format:check

# Run all quality checks (lint + format + build + test)
pnpm quality
```

## 🎯 What's Being Checked

### TypeScript Rules
- ✅ Type safety and correctness
- ✅ Unused variables and imports
- ✅ Floating promises and async/await
- ✅ Naming conventions (PascalCase for classes/interfaces)
- ✅ Prefer nullish coalescing and optional chaining

### Import Organization
- ✅ Automatic import sorting
- ✅ No duplicate imports
- ✅ No circular dependencies
- ✅ No self-imports

### Code Quality (SonarJS)
- ✅ Cognitive complexity limits
- ✅ Duplicate code detection
- ✅ Identical function detection
- ✅ Redundant boolean operations
- ✅ Unused collections
- ✅ Prefer object literals

### Best Practices (Unicorn)
- ✅ Better regex patterns
- ✅ Consistent error handling
- ✅ Filename conventions (kebab-case)
- ✅ Modern JavaScript patterns
- ✅ Prefer built-in methods
- ✅ Custom error definitions

### Security
- ✅ Detect unsafe regex patterns
- ✅ Detect insecure random numbers
- ✅ Detect unsafe child process usage
- ✅ Detect eval usage

### Promise Handling
- ✅ No unhandled promises
- ✅ Proper promise parameter names
- ✅ Catch or return patterns
- ✅ No promise nesting

### General Best Practices
- ✅ No console.log (only warn/error allowed)
- ✅ No debugger statements
- ✅ Use const over let
- ✅ Prefer arrow functions
- ✅ Strict equality (===)
- ✅ No nested ternaries

## 🎨 Prettier Configuration

```json
{
  "semi": true,
  "trailingComma": "es5",
  "singleQuote": true,
  "printWidth": 100,
  "tabWidth": 2,
  "useTabs": false,
  "arrowParens": "always"
}
```

## 🔧 IDE Integration

### VS Code
The project includes `.vscode/settings.json` with:
- ✅ Format on save
- ✅ Auto-fix ESLint issues on save
- ✅ Organize imports on save
- ✅ 100-character ruler
- ✅ Proper TypeScript SDK configuration

### Cursor / Other IDEs
Install the following extensions:
- ESLint
- Prettier - Code formatter
- EditorConfig for VS Code

The project includes `.editorconfig` for consistent settings across all editors.

## 📦 File Ignores

Both ESLint and Prettier ignore:
- `dist/` - Build output
- `build/` - Native build output
- `node_modules/` - Dependencies
- `coverage/` - Test coverage
- `native/**/*.{cpp,h}` - Native C++ code
- `.cursor/` - Cursor IDE files

## 🚀 Pre-commit Workflow

Recommended workflow before committing:

```bash
# 1. Format all files
pnpm format

# 2. Fix auto-fixable linting issues
pnpm lint:fix

# 3. Run full quality check
pnpm quality
```

## 🔍 Disabled Rules & Why

Some rules are intentionally disabled:

- `@typescript-eslint/explicit-function-return-type` - TypeScript infers return types
- `@typescript-eslint/no-explicit-any` - Set to 'warn' instead of 'error' for flexibility
- `unicorn/no-null` - We use null in APIs for compatibility
- `unicorn/prevent-abbreviations` - Common abbreviations are acceptable
- `security/detect-object-injection` - Too many false positives

## 📊 Rule Statistics

- **Total ESLint Rules**: 100+
- **TypeScript Rules**: 30+
- **Security Rules**: 10+
- **Code Quality Rules**: 20+
- **Best Practice Rules**: 40+

## 🎓 Learning Resources

- [ESLint Rules](https://eslint.org/docs/latest/rules/)
- [TypeScript ESLint](https://typescript-eslint.io/rules/)
- [Prettier Options](https://prettier.io/docs/en/options.html)
- [Unicorn Plugin](https://github.com/sindresorhus/eslint-plugin-unicorn)
- [SonarJS Plugin](https://github.com/SonarSource/eslint-plugin-sonarjs)

## 🐛 Troubleshooting

### ESLint is slow
Run with `--debug` flag to see what's taking time:
```bash
pnpm eslint --debug src/
```

### Prettier conflicts with ESLint
The `eslint-config-prettier` package is installed to disable conflicting rules.

### Type checking errors
Make sure `tsconfig.json` is properly configured and TypeScript can find all files:
```bash
pnpm tsc --noEmit
```

## 📈 Metrics

Current code quality metrics:
- ESLint Errors: Target 0
- ESLint Warnings: Minimize
- Prettier Issues: Target 0
- Test Coverage: Target 85%+
- TypeScript Strict: Enabled

---

**Remember**: The goal is to write **professional, maintainable, and secure code**! 🎯


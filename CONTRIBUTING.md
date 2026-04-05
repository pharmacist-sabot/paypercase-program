# Contributing to PayPerCase

Thank you for your interest in contributing! Here's how to get started.

## Prerequisites

- Node.js 22+
- Rust (stable toolchain)
- Python 3 (for exporting default config)

## Getting Started

```bash
git clone https://github.com/pharmacist-sabot/paypercase-program.git
cd paypercase-program
npm install
npm run tauri dev
```

## Workflow

1. **Fork** the repository and create a branch from `main`
2. **Make your changes** — keep commits focused and descriptive
3. **Test** your changes locally with `npm run tauri dev`
4. **Open a Pull Request** against `main` with a clear description

## Branch Naming

| Type | Pattern |
|---|---|
| Feature | `feat/short-description` |
| Bug fix | `fix/short-description` |
| Chore / docs | `chore/short-description` |

## Commit Style

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add export button to settings page
fix: correct payout calculation for SSS pt-type
chore: update default config for new procedures
```

## Releasing (maintainers only)

1. Update `default_config.json` if needed: `npm run export-defaults`
2. Bump version in `package.json` and `src-tauri/Cargo.toml`
3. Commit, tag, and push:

   ```bash
   git commit -m "chore: release vX.Y.Z"
   git tag vX.Y.Z
   git push origin main --tags
   ```

4. CI will automatically build and publish the Windows installer to GitHub Releases.

## Security

- **Never** commit HOSxP credentials or any patient data
- `default_config.json` must not contain connection passwords or PII

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).

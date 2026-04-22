# PayPerCase

[![Release](https://img.shields.io/github/v/release/suradet-ps/paypercase-program?style=flat-square)](https://github.com/suradet-ps/paypercase-program/releases)
[![CI](https://img.shields.io/github/actions/workflow/status/suradet-ps/paypercase-program/release-windows-installer.yml?style=flat-square&label=build)](https://github.com/pharmacist-sabot/paypercase-program/actions)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)
[![Tauri](https://img.shields.io/badge/built%20with-Tauri%20v2-24C8D8?style=flat-square)](https://tauri.app)

> A desktop application for managing therapist compensation at Thai traditional medicine clinics, integrated with HOSxP hospital information system.

## Features

- **HOSxP Integration** — Fetches patient visit data directly from HOSxP via MySQL
- **Compensation Tracking** — Calculates and records therapist payout per visit
- **Configurable Rules** — Manage pt-types, procedures, drugs, and providers through a UI
- **PDF Export** — Generates payment summary reports
- **Offline-first** — All data stored locally in SQLite; HOSxP connection is optional per session

## Download

Get the latest Windows installer from the [Releases](https://github.com/suradet-ps/paypercase-program/releases) page.

## Tech Stack

| Layer | Technology |
|---|---|
| Frontend | Vue 3 + TypeScript + Vite |
| Backend | Rust (Tauri v2) |
| Local DB | SQLite via rusqlite |
| HOSxP DB | MySQL via mysql crate |

## Development

**Prerequisites:** Node.js 22+, Rust (stable), Python 3

```bash
# Install dependencies
npm install

# Start dev server
npm run tauri dev
```

## Updating Default Configuration

Default settings (pt-types, procedures, drugs, providers) are baked into the binary at compile time.

```bash
# 1. Configure everything via the UI in dev mode
npm run tauri dev

# 2. Export current settings to default_config.json
npm run export-defaults

# 3. Commit and tag to trigger a release build
git add src-tauri/default_config.json
git commit -m "chore: update default config"
git tag v5.x.x && git push origin main --tags
```

> HOSxP connection credentials are intentionally excluded from the default config for security.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

[MIT](LICENSE) © PayPerCase

# Contributing to MimicServer

Thank you for your interest in contributing to MimicServer — a [PhoticLabs](https://github.com/Photic-Labs/) project.

MimicServer is free software under the **GNU General Public License v3**.  
By contributing, you agree that your contributions will be licensed under GPL v3.

---

## Table of Contents

- [License](#license)
- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Pull Request Guidelines](#pull-request-guidelines)
- [Coding Standards](#coding-standards)
- [Commit Messages](#commit-messages)
- [Reporting Issues](#reporting-issues)

---

## License

MimicServer is released under the [GNU General Public License v3](LICENSE).  
All contributions must be compatible with this license. By submitting a pull request, you affirm that:

1. You have the right to license your contribution under GPL v3.
2. Your contribution does not include code you do not have permission to share.
3. You understand that your contribution will be publicly available forever under GPL v3.

---

## Code of Conduct

Please read our [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).  
All contributors must follow it. We do not tolerate harassment or exclusionary behavior.

---

## Getting Started

1. Fork the repository on GitHub.
2. Clone your fork:
   ```bash
   git clone https://github.com/<your-username>/mimic-server.git
   cd mimic-server
   ```
3. Ensure you have Rust 1.75+ installed:
   ```bash
   rustup update
   ```
4. Verify the project compiles:
   ```bash
   cargo check --workspace
   ```
5. Read [DEV_GUIDE.md](DEV_GUIDE.md) for a full walkthrough of the codebase.

---

## Development Workflow

```bash
# Run the app
cargo run

# Build release
cargo build --release

# Lint
cargo clippy --workspace

# Check formatting
cargo fmt --check
```

Keep changes focused. A single pull request should address one concern.

---

## Pull Request Guidelines

1. Create a branch from `main`:
   ```bash
   git checkout -b feature/your-feature
   ```
2. Make your changes. Keep commits small and logical.
3. Before pushing, run:
   ```bash
   cargo check --workspace
   cargo clippy --workspace
   cargo fmt --check
   ```
4. Push and open a pull request against `main`.
5. In the PR description, explain **what** changed and **why**.
6. Link any related issues.

Your PR will be reviewed. Expect feedback — it is not personal.

---

## Coding Standards

- Follow the existing code style (run `cargo fmt`).
- Use `cargo clippy` — resolve all warnings before submitting.
- Use `anyhow::Result` for fallible functions unless a specific error type is needed.
- Use `eprintln!` for diagnostics (not `println!` — that goes to stdout).
- Never hardcode hex colors — use `Color::*` from `pl-components`.
- Use the workspace dependency table for shared crates.
- Write clear, self-documenting code. Comments explain *why*, not *what*.

---

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add path-parameter support to route matching
fix: crash when response file is missing
refactor: extract log writer into its own module
docs: update schema in DEV_GUIDE.md
```

Use the imperative mood ("add", "fix", "refactor", not "added", "fixed").

---

## Reporting Issues

Open an issue on [GitHub](https://github.com/Photic-Labs/mimic-server/issues).

Include:
- MimicServer version (from Settings footer or `Cargo.toml`)
- Your OS and Rust version (`rustc --version`)
- Steps to reproduce
- Expected vs actual behavior

For security issues, do **not** open a public issue. Email `photiclabs@proton.me`.

---

*Thank you for helping make MimicServer better.*
*Built with Rust. Designed with intention.*

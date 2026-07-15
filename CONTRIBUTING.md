# Contributing to minifetch

Thanks for wanting to help out.

## Prerequisites

- Rust (stable + nightly for security checks)
- [just](https://github.com/casey/just) command runner

## Getting started

```sh
just all
```

This runs formatting checks, clippy, tests, and security audits. If it passes, you're in good shape.

## What we expect

- **Code compiles, lints, and tests pass.** Clippy is set to `deny` across `all`, `pedantic`, `nursery`, and `cargo` groups. No warnings allowed.
- **Rust edition 2024.** Make sure your toolchain is recent enough.
- **No unsafe code.** Literally `forbid`den in `Cargo.toml`.
- **Keep it concise.** This is a fetch tool, not a framework. Prefer simple, readable code and avoid unnecessary complexity.

## Useful commands

| Command | What it does |
|---------|-------------|
| `just lint` | Format check + clippy |
| `just test` | Run all tests |
| `just sec` | Security audit (needs nightly) |
| `just cov` | Generate coverage report |
| `just open` | Open coverage report |
| `just all` | All of the above |

## Reporting issues

Open an issue on GitHub. Include your OS, distro, and what you expected to happen vs. what actually happened.

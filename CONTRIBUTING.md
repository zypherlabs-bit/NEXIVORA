# Contributing to Nexivora

Thank you for your interest in contributing to Nexivora! We welcome contributions from everyone.

## Code of Conduct

This project adheres to the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). Please be respectful and inclusive in all interactions.

## How to Contribute

### Reporting Issues

- Use the GitHub issue tracker to report bugs or suggest features
- Provide clear, reproducible steps for bugs
- Include relevant information (OS, Rust version, etc.)

### Submitting Pull Requests

1. Fork the repository and create a feature branch
2. Make your changes with clear, focused commits
3. Ensure tests pass (`cargo test`)
4. Update documentation as needed
5. Submit a PR with a descriptive title and explanation

### Development Setup

```bash
# Clone the repository
git clone https://github.com/zypherlabs-bit/NEXIVORA.git
cd nexivora

# Build the project
cargo build --workspace

# Run tests
cargo test --workspace
```

### Code Style

- Follow Rust conventions (rustfmt)
- Use descriptive names and comments
- Keep functions focused and small
- Write tests for new functionality

### Testing

- Run the full test suite before submitting: `cargo test --workspace`
- Add tests for new features
- Ensure existing tests continue to pass

## License

By contributing to Nexivora, you agree that your contributions will be licensed under the AGPL-3.0-or-later license.
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.1.0] - 2024-MM-DD

### Added

- Initial public release of Nexivora
- Core formula engine with parser and evaluator
- Spreadsheet engine with basic cell operations
- Document engine with rich text support
- Presentation engine foundation
- Database engine foundation
- Cross-platform Tauri desktop shell
- Comprehensive CI/CD pipeline
- Formula functions: SUM, AVERAGE, MIN, MAX, COUNT, ABS, ROUND, IF, AND, OR, NOT, TEXT functions, LEN, UPPER, LOWER, CONCAT, TRIM, LEFT, RIGHT, MID, FIND, SUBSTITUTE, REPLACE, VALUE, ISNUMBER, ISTEXT, ISBLANK, ISERROR, IFERROR, DATE, TIME, YEAR, MONTH, DAY, HOUR, MINUTE, SECOND, DAYS, EDATE, EOMONTH
- Cell reference support (A1 notation)
- Absolute and relative cell references
- Range references
- Basic error handling
- Unit test suite with 100+ tests

### Changed

- Improved formula parsing for complex expressions
- Enhanced error messages for debugging
- Optimized evaluation performance

### Fixed

- Absolute cell reference parsing ($B$2 format)
- SUM function handling of ranges
- Various edge cases in formula evaluation

## [0.0.1] - 2024-01-01

### Added

- Initial project setup
- Workspace configuration
- Basic crate structure
- Initial formula engine prototype
- Initial spreadsheet engine prototype

[Unreleased]: https://github.com/zypherlabs-bit/NEXIVORA/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/zypherlabs-bit/NEXIVORA/releases/tag/v0.1.0
[0.0.1]: https://github.com/zypherlabs-bit/NEXIVORA/releases/tag/v0.0.1
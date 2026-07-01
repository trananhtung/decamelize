# Changelog

All notable changes to this project are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2026-07-01

### Fixed

- Apply `rustfmt` formatting to the integration test suite so `cargo fmt --all --check`
  passes in CI. No changes to the library API or behavior.

## [0.1.0]

### Added

- Initial release: `decamelize` and `decamelize_with` — a faithful, zero-dependency,
  `no_std` port of the `decamelize` npm package, including the custom separator and
  `preserveConsecutiveUppercase` options.

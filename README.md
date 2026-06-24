# decamelize

[![crates.io](https://img.shields.io/crates/v/decamelize.svg)](https://crates.io/crates/decamelize)
[![docs.rs](https://docs.rs/decamelize/badge.svg)](https://docs.rs/decamelize)
[![CI](https://github.com/trananhtung/decamelize/actions/workflows/ci.yml/badge.svg)](https://github.com/trananhtung/decamelize/actions/workflows/ci.yml)
[![license](https://img.shields.io/crates/l/decamelize.svg)](#license)

**Convert a `camelCase` string to a separated lower-case string.**

`fooBar` → `foo_bar`, `unicornsAndRainbows` → `unicorns_and_rainbows`,
`XMLHttpRequest` → `xml_http_request`. A faithful Rust port of the widely-used
[`decamelize`](https://www.npmjs.com/package/decamelize) npm package.

- **Zero dependencies**, **`#![no_std]`**
- Custom separator and a `preserveConsecutiveUppercase` mode
- Differential-tested against the reference `decamelize` implementation

## Install

```toml
[dependencies]
decamelize = "0.1"
```

## Usage

```rust
use decamelize::{decamelize, decamelize_with};

assert_eq!(decamelize("unicornsAndRainbows"), "unicorns_and_rainbows");
assert_eq!(decamelize("testGUILabel"), "test_gui_label");
assert_eq!(decamelize("p2pNetwork"), "p2p_network");

// Custom separator:
assert_eq!(decamelize_with("fooBar", "-", false), "foo-bar");
assert_eq!(decamelize_with("unicornsAndRainbows", " ", false), "unicorns and rainbows");

// Preserve consecutive uppercase runs:
assert_eq!(decamelize_with("testGUILabel", "_", true), "test_GUI_label");
assert_eq!(decamelize_with("XMLHttpRequest", "_", true), "XML_http_request");
```

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.

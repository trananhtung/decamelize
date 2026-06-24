# decamelize

[![All Contributors](https://img.shields.io/badge/all_contributors-1-orange.svg?style=flat-square)](#contributors-)

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

## Contributors ✨

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind are welcome — code, docs, bug reports, ideas, reviews! See the [emoji key](https://allcontributors.org/docs/en/emoji-key) for how each contribution is recognized, and open a PR or issue to get involved.

Thanks goes to these wonderful people:

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/trananhtung"><img src="https://avatars.githubusercontent.com/u/30992229?v=4?s=100" width="100px;" alt="Tung Tran"/><br /><sub><b>Tung Tran</b></sub></a><br /><a href="https://github.com/trananhtung/./commits?author=trananhtung" title="Code">💻</a> <a href="#maintenance-trananhtung" title="Maintenance">🚧</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.

//! # decamelize — convert camelCase to a separated lower-case string
//!
//! Turn `camelCase` (or `PascalCase`) into a separated, lower-case string — `fooBar` →
//! `foo_bar`, `unicornsAndRainbows` → `unicorns_and_rainbows`. A faithful Rust port of the
//! widely-used [`decamelize`](https://www.npmjs.com/package/decamelize) npm package.
//!
//! ```
//! use decamelize::decamelize;
//!
//! assert_eq!(decamelize("unicornsAndRainbows"), "unicorns_and_rainbows");
//! assert_eq!(decamelize("XMLHttpRequest"), "xml_http_request");
//! assert_eq!(decamelize("testGUILabel"), "test_gui_label");
//! ```
//!
//! Use [`decamelize_with`] for a custom separator or to preserve consecutive uppercase
//! runs:
//!
//! ```
//! use decamelize::decamelize_with;
//!
//! assert_eq!(decamelize_with("fooBar", "-", false), "foo-bar");
//! assert_eq!(decamelize_with("testGUILabel", "_", true), "test_GUI_label");
//! ```
//!
//! **Zero dependencies** and `#![no_std]`.

#![no_std]
#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/decamelize/0.1.0")]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

// Compile-test the README's examples as part of `cargo test`.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

/// Convert `text` from camelCase to a `_`-separated lower-case string.
///
/// Equivalent to [`decamelize_with(text, "_", false)`](decamelize_with).
///
/// ```
/// # use decamelize::decamelize;
/// assert_eq!(decamelize("fooBar"), "foo_bar");
/// ```
#[must_use]
pub fn decamelize(text: &str) -> String {
    decamelize_with(text, "_", false)
}

/// Convert `text` from camelCase using a custom `separator`.
///
/// When `preserve_consecutive_uppercase` is `true`, runs of consecutive uppercase letters
/// keep their case (`testGUILabel` → `test_GUI_label`); otherwise the whole result is
/// lower-cased.
///
/// ```
/// # use decamelize::decamelize_with;
/// assert_eq!(decamelize_with("fooBarBaz", ".", false), "foo.bar.baz");
/// ```
#[must_use]
pub fn decamelize_with(
    text: &str,
    separator: &str,
    preserve_consecutive_uppercase: bool,
) -> String {
    // Matches the reference's UTF-16 length check.
    if text.encode_utf16().take(2).count() < 2 {
        return if preserve_consecutive_uppercase {
            text.into()
        } else {
            text.to_lowercase()
        };
    }

    // Split a lowercase letter or digit followed by an uppercase letter.
    let split = split_lower_then_upper(text, separator);

    if preserve_consecutive_uppercase {
        let lowered = lowercase_isolated(&split);
        split_upper_run(&lowered, separator, true)
    } else {
        split_upper_run(&split, separator, false).to_lowercase()
    }
}

fn is_upper(c: char) -> bool {
    c.is_uppercase()
}

fn is_lower(c: char) -> bool {
    c.is_lowercase()
}

/// `\d` (ASCII digits, as in JavaScript regular expressions).
fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_upper_or_digit(c: char) -> bool {
    is_upper(c) || is_digit(c)
}

/// `([\p{Ll}\d])(\p{Lu})` → insert `separator` between each lowercase/digit → uppercase
/// transition.
fn split_lower_then_upper(text: &str, separator: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut prev: Option<char> = None;
    for c in text.chars() {
        if let Some(p) = prev {
            if (is_lower(p) || is_digit(p)) && is_upper(c) {
                out.push_str(separator);
            }
        }
        out.push(c);
        prev = Some(c);
    }
    out
}

/// `((?<![\p{Lu}\d])[\p{Lu}\d](?![\p{Lu}\d]))` → lowercase any single uppercase/digit that
/// is not adjacent to another uppercase/digit.
fn lowercase_isolated(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    for (i, &c) in chars.iter().enumerate() {
        let isolated = is_upper_or_digit(c)
            && (i == 0 || !is_upper_or_digit(chars[i - 1]))
            && (i + 1 >= chars.len() || !is_upper_or_digit(chars[i + 1]));
        if isolated {
            out.extend(c.to_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

/// Split the boundary between a run of uppercase letters and a following lowercase word.
///
/// This implements both `(\p{Lu})(\p{Lu}\p{Ll}+)` (non-preserve) and
/// `(?<!\p{Lu})(\p{Lu}+)(\p{Lu}\p{Ll}+)` (preserve): in either case the separator is
/// inserted before the **last** uppercase letter of a run (length ≥ 2) that is immediately
/// followed by a lowercase letter. When `lowercase_moved` is set, that moved last letter is
/// lower-cased (the rest of the run keeps its case).
fn split_upper_run(text: &str, separator: &str, lowercase_moved: bool) -> String {
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();
    let mut out = String::with_capacity(text.len() + separator.len());
    let mut i = 0;
    while i < n {
        if is_upper(chars[i]) && (i == 0 || !is_upper(chars[i - 1])) {
            // Maximal uppercase run [i, j).
            let mut j = i;
            while j < n && is_upper(chars[j]) {
                j += 1;
            }
            let run_len = j - i;
            if run_len >= 2 && j < n && is_lower(chars[j]) {
                for &c in &chars[i..j - 1] {
                    out.push(c);
                }
                out.push_str(separator);
                if lowercase_moved {
                    out.extend(chars[j - 1].to_lowercase());
                } else {
                    out.push(chars[j - 1]);
                }
            } else {
                for &c in &chars[i..j] {
                    out.push(c);
                }
            }
            i = j;
        } else {
            out.push(chars[i]);
            i += 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(decamelize("unicornsAndRainbows"), "unicorns_and_rainbows");
        assert_eq!(decamelize("fooBar"), "foo_bar");
        assert_eq!(decamelize("p2pNetwork"), "p2p_network");
    }

    #[test]
    fn acronyms() {
        assert_eq!(decamelize("XMLHttpRequest"), "xml_http_request");
        assert_eq!(decamelize("testGUILabel"), "test_gui_label");
        assert_eq!(decamelize("testID"), "test_id");
        assert_eq!(decamelize("ABCdef"), "ab_cdef");
        assert_eq!(decamelize("fooBAR"), "foo_bar");
    }

    #[test]
    fn digits_and_short() {
        assert_eq!(decamelize("foo2bar"), "foo2bar");
        assert_eq!(decamelize("test123Number"), "test123_number");
        assert_eq!(decamelize("ID"), "id");
        assert_eq!(decamelize("a"), "a");
        assert_eq!(decamelize("A"), "a");
        assert_eq!(decamelize(""), "");
        assert_eq!(decamelize("__foo__bar__"), "__foo__bar__");
    }

    #[test]
    fn separators() {
        assert_eq!(decamelize_with("fooBar", "-", false), "foo-bar");
        assert_eq!(
            decamelize_with("unicornsAndRainbows", " ", false),
            "unicorns and rainbows"
        );
        assert_eq!(
            decamelize_with("XMLHttpRequest", "-", false),
            "xml-http-request"
        );
    }

    #[test]
    fn preserve_consecutive_uppercase() {
        assert_eq!(decamelize_with("testGUILabel", "_", true), "test_GUI_label");
        assert_eq!(
            decamelize_with("XMLHttpRequest", "_", true),
            "XML_http_request"
        );
        assert_eq!(decamelize_with("testID", "_", true), "test_ID");
        assert_eq!(decamelize_with("fooBAR", "_", true), "foo_BAR");
        assert_eq!(decamelize_with("ABCdef", "_", true), "AB_cdef");
        assert_eq!(decamelize_with("A", "_", true), "A");
    }
}

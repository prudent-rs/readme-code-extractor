#![doc = include_str!("../README.md")]

pub use mce_proc::{all, all_by_file, mce_tag, mce_tag_by_file, nth, nth_by_file};

/// We can't report the actual version(s), because [panic] macro is not eager, and passing in
/// (formatting) variables doesn't work in `const` context. See also
/// <https://rustc-dev-guide.rust-lang.org/macro-expansion.html#eager-expansion>.
const _: () = {
    let proc_version = mce_proc::version!();

    if !is_exact_version(proc_version) {
        panic!(
            "prudent-rs/mce-proc is of different version than \
                prudent-rs/mce. Please report this as an issue, along with both \
                versions."
        );
    }
};

const fn is_exact_version(expected_version: &'static str) -> bool {
    // We can't use a comparison operator ==, because trait PartialEq is not const (in April 2026).
    matches!(expected_version.as_bytes(), b"0.0.6")
}

const _: () = {
    if !is_exact_version(env!("CARGO_PKG_VERSION")) {
        panic!("prudent-rs/mce has its function is_exact_version() out of date.");
    }
};

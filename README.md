[![GitHub_Actions](https://github.com/prudent-rs/mce/actions/workflows/main.yml/badge.svg)](https://github.com/prudent-rs/mce/actions)

# mce: Markdown Code Extractor

Rust macros to extract part(s) of `README.md` (or a similar Markdown file) and to use those parts in
tests, doctests or elsewhere.

## Why?

As per

- [stackoverflow#35080160](https://stackoverflow.com/questions/35080160/github-include-md-files-in-readme-md),
- [github/markup#346](https://github.com/github/markup/issues/346) and
- [github/markup#172](https://github.com/github/markup/issues/172)

GitHub does not support/allow `README.md` (or `README.rst`) to include other file(s).

## Alternatives

Rust supports the other way: including `README.md` (or any file) as a part of rustdoc, usually done
in `src/lib.rs` with:
```rust,ignore
#![doc = include_str!("../README.md")]
```

However, that doesn't let you pick just specific parts of `README.md` (or another file). That would
be beneficial, for example when re-using

- specific code sections in both `README.md` and negative/positive doctests, or
- specific code sections with specific cargo features or configuration.

There **are** several crates and methods that generate `README.md` based on other file(s). But

- a consumer crate has to use `build.rs`, which is a burden. Or
- you need CI setup, GIT pre-commit hook or manual steps
  - such a setup needs to be manually checked, which is error prone and time-consuming.
  - they don't get automatically invoked by `cargo check` and friends.
- With any of these methods, you still need to commit the generated/modified `README.md` to GIT.

Instead, `mce` extracts the code section(s) with no side effects. It's stateless.

## Examples

For now we don't have a separate/detailed documentation. See examples in
[`positive_tests/src/bin/`](https://github.com/prudent-rs/mce/tree/main/positive_tests/src/bin).
<!-- \\\--- We use a FULL GitHub link. Otherwise
1. https://crates.io would auto-generate link as
   https://github.com/REPO/PROJECT/blob/HEAD/path-here/, which auto-converts to a
   commit link!
2. https://docs.rs would generate the link as
   https://docs.rs/CRATE/latest/CRATE/path-here/, which doesn't exist, because as per
   https://doc.rust-lang.org/nightly/cargo/reference/manifest.html#the-exclude-and-include-fields
   > "Regardless of whether exclude or include..."
   > "Any sub-packages will be skipped (any subdirectory that contains a Cargo.toml file)."
-->

## Configuration in TOML only

Configuration is only in TOML, deserialized with [`toml-rs/toml`](https://github.com/toml-rs/toml).
No other formats (JSON, [`eternal-io/keon`](https://github.com/eternal-io/keon),
[`ron-rs/ron`](https://github.com/ron-rs/ron)... ). Why? Because TOML is

- simple and readable
- already used by Rust community
- both clean and expressive enough for simple Rust values, see `toml-rs/toml` ->
  - [`crates/toml/examples/enum_external.rs`](https://github.com/toml-rs/toml/blob/main/crates/toml/examples/enum_external.rs)
  - [`crates/toml/tests/serde/de_enum.rs`](https://github.com/toml-rs/toml/blob/main/crates/toml/tests/serde/de_enum.rs)
    -> `fn value_from_str()`
- syntax highlighted by ["Extended **Embedded**
  Languages"](https://marketplace.visualstudio.com/items?itemName=ruschaaf.extended-embedded-languages)
  in VS Code. That also works **in raw strings** passed to
  - `#![doc = r#"..."#]` or
  - `#[doc  = r#"..."#]` (and other attributes).
- yes, TOML is _not_ ideal for deep structures, but that's not our case.

## MSRV

MSRV (minimum supported Rust version), as indicated with `rust-version` in
[`Cargo.toml`](Cargo.toml), is 1.88.0. However, it's tested only with Rust 1.89.0+. You will get
incorrect (false positive) `dead_code` warnings, unless you use Rust 1.89.0+. (See
[rust-lang/rust#142208](https://github.com/rust-lang/rust/pull/142208).)

## Blockers and related issues

Please give thumbs up (and contribute, if you can) to

- [SergioBenitez/proc-macro2-diagnostics#13](https://github.com/SergioBenitez/proc-macro2-diagnostics/issues/13)
  defect: Error message and details missing, when macro fails to generate main() on STABLE
- [dtolnay/no-panic#28](https://github.com/dtolnay/no-panic/issues/28) `no-panic` currently cannot
  be used in doctests
- [dtolnay/no-panic#78](https://github.com/dtolnay/no-panic/issues/78) Not working (false negative)
  in integration tests

## Side fruit

- [dtolnay/proc-macro2#536](https://github.com/dtolnay/proc-macro2/pull/536) `Span::file` and
  `Span::local_file` suggest to use Rust 1.88 when using stable

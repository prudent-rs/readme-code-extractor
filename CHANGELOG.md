# Changelog

This includes changes from crate `mce-proc`.

## 2026-June-24

## Version 0.0.6

- using `dis` version `0.0.4`

## 2026-June-21

## Version 0.0.5 (mce-lib only)

- minor/internal

## 2026-May-28 (2026-May-29 in UTC)

## Version 0.0.4

- not using `proc-macro2-diagnostic` for error reporting for now (in the meantime using `panic!`).
- internal: `selected_by_config_content_and_span now(...)` now takes a closure for validation stage.
- updated `mce-proc` and `mce-lib` to `0.0.4`: renamed `.spanned(span)` --> `.span_err(span)`

## 2026-May-27

## Version 0.0.3

Initial published version. Macros `all, all_by_file, mce_tag, mce_tag_by_file, nth, nth_by_file`.

## Unpublished (only more important decision points listed)

### May 3, 2026

Renamed from readme-code-extractor to mce (Markdown Code Extractor).

### May 1, 2026

Error handling:

- `public::ext` module (extension traits) for
  - errors implementing `Debug`, `ToString` or `Into<String>`
  - error indicators
    - containers holding those errors: `Result` and `Option`
    - `bool`
- `public::assert` module with more intentional function names

### April 30, 2026

Error handling: `proc_macro2_diagnostics` crate +`DeepDiagnostic`.

### April 27, 2026

- `all_by_file` macro

### April 25, 2026

- stopped using `trybuild` crate in [negative_tests_runner/`](negative_tests_runner/). See
  [`prudent-rs/mce -> negative_tests_runner/src/lib.rs commit
  #ff8935f`](https://github.com/prudent-rs/mce/blob/ff8935ff314133ddc432c32d0ec89c41f4dd0dd0/negative_tests_runner/src/lib.rs).

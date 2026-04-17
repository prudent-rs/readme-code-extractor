# readme-code-extractor

Rust macros to extract part(s) of README.md (or a similar file) and to use them in
tests/doctests/elsewhere.

## MSRV
MSRV (minimum supported Rust version), as indicated with `rust-version` in
[`Cargo.toml`](Cargo.toml), is 1.88.0. However, you will get incorrect (false positive) `dead_code`
warnings, unless you use Rust 1.89.0+.

<!--
## Related issues

Please give thumbs up (and contribute, if you can) to
-->
#[cfg(debug_assertions)]
compile_error!("Build in release mode only.");

#[cfg(test)]
compile_error!("Don't test. Instead, build (in release, that is, with `cargo build --release`.");

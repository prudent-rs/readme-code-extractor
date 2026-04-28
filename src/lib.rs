// @TODO
//
//#![doc = top_level!( )]
//all_by_file!(top_level, "some-file.toml");

pub use readme_code_extractor_proc::{all, all_by_file, nth };

/// We can't report the actual version(s), because [panic] macro is not eager, and passing in
/// (formatting) variables doesn't work in const context. See also
/// <https://rustc-dev-guide.rust-lang.org/macro-expansion.html#eager-expansion>.
#[doc = "Testing simple but multiline string literal in doc attribute. It works, BUT NOT FOR TOML
         - new lines are removed, so we'd need to add them manually: \n\
        ```\n\
           // hrhr \n\
           // hhr \n\
        ```"]
const _: () = {
    let proc_version = readme_code_extractor_proc::version!();

    if !is_exact_version(proc_version) {
        panic!(
            "prudent-rs/readme-code-extractor-proc is of different version than \
                prudent-rs/readme-code-extractor. Please report this as an issue, along with both \
                versions."
        );
    }
};

const fn is_exact_version(expected_version: &'static str) -> bool {
    // We can't use a comparison operator ==, because trait PartialEq is not const (in April 2026).
    matches!(expected_version.as_bytes(), b"0.0.1")
}

const _: () = {
    if !crate::is_exact_version(env!("CARGO_PKG_VERSION")) {
        panic!("prudent-rs/readme-code-extractor has its function is_exact_version() out of date.");
    }
};

/*
/// Like [all], but load the configuration from a (TOML) file.
#[macro_export]
macro_rules! all_by_file {
    ( $name_of_new_extractor_macro:ident, $config_file_path:literal ) => {
        const _: () = {
            // Let's assure the literal is a string literal, to simplify error triage.
            const _: &str = $config_file_path;
            // We have to use `include_str!`, so that any change to the config file gets noticed by
            // rustc/cargo, so that on next `cargo check` amd similar they rebuild the crate.
            ::core::hint::black_box(::core::include_str!($config_file_path));
        };

        // Create an extractor macro. In case of `all_by_file` we don't strictly need it, but then
        // the user would have to guarantee that she would
        // `::core::hint::black_box(::core::include_str!($config_file_path));` on her own. And that
        // allows for human error.
        //
        // (In case of `nth_by_file` we do need to create an extractor macro either way, because of
        // the extra parameter `n`. See `nth_by_file`.)
        //
        // We do HAVE TO `macro_export` it. Otherwise it can't be used in `#[doc = ... ]` or `#![doc
        // = ... ]` (which are processed in a crate separate to the crate that called `all_by_file`
        // macro).
        #[macro_export]
        macro_rules! $name_of_new_extractor_macro {
            () => {
                // Unfortunately, Rust macros are lazy (and not eager). So here we can't
                // "just tell" or `all!` macro to "evaluate"/receive/get result of
                // `::core::include_str!($config_file_path)`.
                //
                // Therefore we call a proc macro, which loads the file and passes its
                // content to `all` macro
                ::readme_code_extractor_proc::all_by_file!($config_file_path)
            };
        }
    };
}*/

/// Like [nth], but load the configuration from a (TOML) file. See also [all_by_file].
#[macro_export]
macro_rules! nth_by_file {
    ( $name_of_new_extractor_macro:ident, $config_file_path:literal ) => {
        const _: () = {
            const _: &str = $config_file_path;
            ::core::hint::black_box(::core::include_str!($config_file_path));
        };

        // Unlike `all_by_file`, here we can't simply create an accessor/applier macro using
        // `macro_rules`. This accessor/applier macro needs to accept a parameter (position/index of
        // the code block to return).
        //
        // But, `macro_rules` can't create a macro that itself creates uses `macro_rules` to create
        // another macro that accepts parameters, because the dollar prefix would be resolved as if
        // it were a capturing variable from the outer macro. (And `$$` is not stable yet
        // (https://github.com/rust-lang/rust/issues/83527)).
        /*
        #[macro_export]
        macro_rules! $name_of_new_extractor_macro {
            ($$a:literal) => {
                ::readme_code_extractor_proc::nth!($config_file_path);
                let _ = $$a;
            };
        }*/
    };
}

#[macro_export]
macro_rules! generate {
    //    () => { "Gen" };
    () => {{
        const _: () = {};
        "Hi"
    }};
}

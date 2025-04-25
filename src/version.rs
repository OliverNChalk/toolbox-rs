#[doc(hidden)]
pub use const_format::formatcp as __formatcp;

#[doc(hidden)]
pub const DEFAULT_GIT_SHA: &str = "0000000";
#[doc(hidden)]
pub const DEFAULT_GIT_COMMIT_TIMESTAMP: &str = "0000-00-00T00:00:00.000000000Z";

#[doc(hidden)]
#[macro_export]
macro_rules! unwrap_or {
    ($option:ident, $default:expr) => {
        match $option {
            Some(val) => val,
            None => $default,
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! unwrap_or_default {
    ($option:ident) => {
        $crate::unwrap_or!($option, $crate::version::__formatcp!("{} MISSING", stringify!($option)))
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! dynamic_unwrap_or_default {
    ($option:ident) => {
        std::env::var(stringify!($option))
            .ok()
            .or_else(|| ($option).map(::std::string::ToString::to_string))
            .unwrap_or_else(|| format!("{} MISSING", stringify!($option)))
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __load_build_info {
    () => {
        const CARGO_PKG_VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
        const GIT_SHA: Option<&str> = option_env!("VERGEN_GIT_SHA");
        const GIT_DIRTY: Option<&str> = option_env!("VERGEN_GIT_DIRTY");
        const GIT_COMMIT_TIMESTAMP: Option<&str> = option_env!("VERGEN_GIT_COMMIT_TIMESTAMP");
        const GIT_COMMIT_MESSAGE: Option<&str> = option_env!("VERGEN_GIT_COMMIT_MESSAGE");
        const RUSTC_SEMVER: Option<&str> = option_env!("VERGEN_RUSTC_SEMVER");
        const RUSTC_HOST_TRIPLE: Option<&str> = option_env!("VERGEN_RUSTC_HOST_TRIPLE");
        const CARGO_TARGET_TRIPLE: Option<&str> = option_env!("VERGEN_CARGO_TARGET_TRIPLE");
    };
}

/// Version info for the current program.
///
/// # Example
/// `my-crate 2.0.0 (b142e42 2024-09-09T12:52:12.000000000Z)`
#[macro_export]
macro_rules! version {
    () => {{
        #[allow(dead_code)]
        {
            $crate::__load_build_info!();

            $crate::version::__formatcp!(
                "{} ({}{} {})",
                $crate::unwrap_or_default!(CARGO_PKG_VERSION),
                $crate::unwrap_or!(GIT_SHA, $crate::version::DEFAULT_GIT_SHA),
                match GIT_DIRTY {
                    Some(val) => {
                        let val = val.as_bytes();
                        if val[0] == b'f'
                            && val[1] == b'a'
                            && val[2] == b'l'
                            && val[3] == b's'
                            && val[4] == b'e'
                        {
                            ""
                        } else {
                            "-dirty"
                        }
                    }
                    None => "",
                },
                $crate::unwrap_or!(
                    GIT_COMMIT_TIMESTAMP,
                    $crate::version::DEFAULT_GIT_COMMIT_TIMESTAMP
                ),
            )
        }
    }};
}

/// Version info for the current program, can be overridden at runtime with
/// environment variables.
///
/// # Example
/// `my-crate 2.0.0 (b142e42 2024-09-09T12:52:12.000000000Z)`
#[macro_export]
macro_rules! version_dynamic {
    () => {{
        #[allow(dead_code)]
        {
            $crate::__load_build_info!();

            format!(
                "{} ({}{} {})",
                $crate::dynamic_unwrap_or_default!(CARGO_PKG_VERSION),
                std::env::var("GIT_SHA").ok().unwrap_or_else(|| GIT_SHA
                    .unwrap_or($crate::version::DEFAULT_GIT_SHA)
                    .to_string()),
                match std::env::var("GIT_DIRTY")
                    .ok()
                    .unwrap_or_else(|| GIT_DIRTY.unwrap_or("false").to_string())
                    .parse()
                    .unwrap_or(false)
                {
                    true => "-dirty",
                    false => "",
                },
                std::env::var("GIT_COMMIT_TIMESTAMP")
                    .ok()
                    .unwrap_or_else(|| GIT_COMMIT_TIMESTAMP
                        .unwrap_or($crate::version::DEFAULT_GIT_COMMIT_TIMESTAMP)
                        .to_string()),
            )
        }
    }};
}

/// Detailed version info for the current program.
///
/// # Example
/// ```notrust
/// my-crate
/// Version:       2.0.0 (b142e42 2024-09-09T12:52:12.000000000Z)
/// Rustc Version: 1.86.0
/// Rustc Host:    x86_64-unknown-linux-gnu
/// Cargo Target:  x86_64-unknown-linux-gnu
///
/// feat: support message-pack encoding
/// ```
#[macro_export]
macro_rules! long_version {
    () => {{
        #[allow(dead_code)]
        {
            $crate::__load_build_info!();

            $crate::version::__formatcp!(
                r#"
Version:       {}
Rustc Version: {}
Rustc Host:    {}
Cargo Target:  {}

{}"#,
                $crate::version!(),
                $crate::unwrap_or_default!(RUSTC_SEMVER),
                $crate::unwrap_or_default!(RUSTC_HOST_TRIPLE),
                $crate::unwrap_or_default!(CARGO_TARGET_TRIPLE),
                $crate::unwrap_or_default!(GIT_COMMIT_MESSAGE),
            )
        }
    }};
}

/// Detailed version info for the current program, can be overridden at runtime
/// with environment variables.
///
/// # Example
/// ```notrust
/// my-crate
/// Version:       2.0.0 (b142e42 2024-09-09T12:52:12.000000000Z)
/// Rustc Version: 1.86.0
/// Rustc Host:    x86_64-unknown-linux-gnu
/// Cargo Target:  x86_64-unknown-linux-gnu
///
/// feat: support message-pack encoding
/// ```
#[macro_export]
macro_rules! long_version_dynamic {
    () => {{
        #[allow(dead_code)]
        {
            $crate::__load_build_info!();

            format!(
                r#"
Version:       {}
Rustc Version: {}
Rustc Host:    {}
Cargo Target:  {}

{}"#,
                $crate::version_dynamic!(),
                $crate::dynamic_unwrap_or_default!(RUSTC_SEMVER),
                $crate::dynamic_unwrap_or_default!(RUSTC_HOST_TRIPLE),
                $crate::dynamic_unwrap_or_default!(CARGO_TARGET_TRIPLE),
                $crate::dynamic_unwrap_or_default!(GIT_COMMIT_MESSAGE),
            )
        }
    }};
}

/// Log build info using [`tracing`].
///
/// The following values are logged if `vergen` has been run as part of the
/// project's build.rs:
///
/// - `CARGO_PKG_VERSION`
/// - `GIT_SHA`
/// - `GIT_COMMIT_TIMESTAMP`
/// - `GIT_COMMIT_MESSAGE`
/// - `RUSTC_SEMVER`
/// - `RUSTC_HOST_TRIPLE`
/// - `CARGO_TARGET_TRIPLE`
#[cfg(feature = "tracing")]
#[macro_export]
macro_rules! log_build_info {
    () => {{
        $crate::__load_build_info!();

        $crate::version::_log_build_info(
            CARGO_PKG_VERSION.unwrap_or(""),
            GIT_SHA.unwrap_or($crate::version::DEFAULT_GIT_SHA),
            GIT_DIRTY.unwrap_or("false"),
            GIT_COMMIT_TIMESTAMP.unwrap_or(""),
            GIT_COMMIT_MESSAGE.unwrap_or(""),
            RUSTC_SEMVER.unwrap_or(""),
            RUSTC_HOST_TRIPLE.unwrap_or(""),
            CARGO_TARGET_TRIPLE.unwrap_or(""),
        )
    }};
}

#[doc(hidden)]
#[allow(clippy::too_many_arguments)]
#[cfg(feature = "tracing")]
pub fn _log_build_info(
    cargo_pkg_version: &str,
    git_sha: &str,
    git_dirty: &str,
    git_commit_timestamp: &str,
    git_commit_message: &str,
    rustc_semver: &str,
    rustc_host_triple: &str,
    cargo_target_triple: &str,
) {
    ::tracing::info!(
        cargo_pkg_version,
        git_sha,
        git_dirty,
        git_commit_timestamp,
        git_commit_message,
        rustc_semver,
        rustc_host_triple,
        cargo_target_triple,
        "Build information"
    )
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    #[test]
    fn version_const() {
        expect!["0.2.1 (0000000 0000-00-00T00:00:00.000000000Z)"].assert_eq(&crate::version!());
    }

    #[test]
    fn version_dynamic() {
        expect!["0.2.1 (0000000 0000-00-00T00:00:00.000000000Z)"]
            .assert_eq(&crate::version_dynamic!());
    }

    #[test]
    fn long_version_const() {
        expect![[r#"

            Version:       0.2.1 (0000000 0000-00-00T00:00:00.000000000Z)
            Rustc Version: RUSTC_SEMVER MISSING
            Rustc Host:    RUSTC_HOST_TRIPLE MISSING
            Cargo Target:  CARGO_TARGET_TRIPLE MISSING

            GIT_COMMIT_MESSAGE MISSING"#]]
        .assert_eq(&crate::long_version!());
    }

    #[test]
    fn long_version_dynamic() {
        expect![[r#"

            Version:       0.2.1 (0000000 0000-00-00T00:00:00.000000000Z)
            Rustc Version: RUSTC_SEMVER MISSING
            Rustc Host:    RUSTC_HOST_TRIPLE MISSING
            Cargo Target:  CARGO_TARGET_TRIPLE MISSING

            GIT_COMMIT_MESSAGE MISSING"#]]
        .assert_eq(&crate::long_version_dynamic!());
    }

    #[test]
    fn log_build_info() {
        crate::log_build_info!();
    }
}

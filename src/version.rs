#[doc(hidden)]
pub use const_format::formatcp as __formatcp;

// TODO: Can this just be rewritten as const functions?

#[doc(hidden)]
#[macro_export]
macro_rules! __load_build_info {
    () => {
        const CARGO_PKG_VERSION: &str = match option_env!("CARGO_PKG_VERSION") {
            Some(val) => val,
            None => "",
        };
        const GIT_SHA: &str = match option_env!("VERGEN_GIT_SHA") {
            Some(val) => val,
            None => "",
        };
        const GIT_COMMIT_TIMESTAMP: &str = match option_env!("VERGEN_GIT_COMMIT_TIMESTAMP") {
            Some(val) => val,
            None => "",
        };
        const GIT_COMMIT_MESSAGE: &str = match option_env!("VERGEN_GIT_COMMIT_MESSAGE") {
            Some(val) => val,
            None => "",
        };
        const RUSTC_SEMVER: &str = match option_env!("VERGEN_RUSTC_SEMVER") {
            Some(val) => val,
            None => "",
        };
        const RUSTC_HOST_TRIPLE: &str = match option_env!("VERGEN_RUSTC_HOST_TRIPLE") {
            Some(val) => val,
            None => "",
        };
        const CARGO_TARGET_TRIPLE: &str = match option_env!("VERGEN_CARGO_TARGET_TRIPLE") {
            Some(val) => val,
            None => "",
        };
    };
}

/// Version info for the current program.
///
/// # Example
/// `my-crate 2.0.0 (b142e42 2024-09-09T12:52:12.000000000Z)`
#[macro_export]
macro_rules! version {
    () => {{
        $crate::__load_build_info!();

        $crate::version::__formatcp!("{} ({} {})", CARGO_PKG_VERSION, GIT_SHA, GIT_COMMIT_TIMESTAMP)
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
        $crate::__load_build_info!();

        $crate::version::__formatcp!(
            r#"
Version:       {}
Rustc Version: {RUSTC_SEMVER}
Rustc Host:    {RUSTC_HOST_TRIPLE}
Cargo Target:  {CARGO_TARGET_TRIPLE}

{GIT_COMMIT_MESSAGE}"#,
            $crate::version!(),
        )
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
#[macro_export]
macro_rules! log_build_info {
    () => {{
        $crate::__load_build_info!();

        $crate::version::_log_build_info(
            CARGO_PKG_VERSION,
            GIT_SHA,
            GIT_COMMIT_TIMESTAMP,
            GIT_COMMIT_MESSAGE,
            RUSTC_SEMVER,
            RUSTC_HOST_TRIPLE,
            CARGO_TARGET_TRIPLE,
        )
    }};
}

#[doc(hidden)]
pub fn _log_build_info(
    cargo_pkg_version: &str,
    git_sha: &str,
    git_commit_timestamp: &str,
    git_commit_message: &str,
    rustc_semver: &str,
    rustc_host_triple: &str,
    cargo_target_triple: &str,
) {
    tracing::info!(
        cargo_pkg_version,
        git_sha,
        git_commit_timestamp,
        git_commit_message,
        rustc_semver,
        rustc_host_triple,
        cargo_target_triple,
        "Build information"
    )
}

#[cfg(feature = "pkg_data")]
pub const IMPORTING_PROJECT_NAME: &str = env!("CARGO_PKG_NAME");

#[cfg(feature = "pkg_data")]
pub const IMPORTING_PROJECT_VERSION: &str = env!("CARGO_PKG_VERSION");

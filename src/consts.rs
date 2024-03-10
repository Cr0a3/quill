pub static DOMAIN: &str = "localhost";

#[cfg(windows)]
pub static BINARY_EXT: &str = "exe";
#[cfg(unix)]
pub static BINARY_EXT: &str = "out";

#[cfg(windows)]
pub static LIBARY_EXT: &str = "dll";
#[cfg(unix)]
pub static LIBARY_EXT: &str = "lib";

#[cfg(windows)]
pub static LIBARY_LD_FLAG: &str = "--dll";
#[cfg(unix)]
pub static LIBARY_LD_FLAG: &str = "--lib";
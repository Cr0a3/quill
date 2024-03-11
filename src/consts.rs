pub static DOMAIN: &str = "localhost";

#[cfg(windows)]
pub static BINARY_EXT: &str = "exe";
#[cfg(unix)]
pub static BINARY_EXT: &str = "out";

#[cfg(windows)]
pub static LIBARY_EXT: &str = "dll";
#[cfg(unix)]
pub static LIBARY_EXT: &str = "so";

#[cfg(windows)]
pub static LIBARY_LD_FLAG: &str = "--dll";
#[cfg(unix)]
pub static LIBARY_LD_FLAG: &str = "--shared";

#[cfg(windows)]
pb static LIBARY_LINK_LD_OPT: &str = "-l";
#[cfg(windows)]
pb static LIBARY_LINK_LD_OPT: &str = "-l:";

#[cfg(windows)]
pb static LIBARY_LINK_LD_OPTI: &str = "";
#[cfg(windows)]
pb static LIBARY_LINK_LD_OPTI: &str = ".so";
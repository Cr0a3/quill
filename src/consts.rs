<<<<<<< HEAD
pub static DOMAIN: &str = "http://localhost:8000/api";
=======
pub const DOMAIN: &str = "localhost";
>>>>>>> 4fb64176c6bcd6d4eef5a28430231d15b0091ce4

#[cfg(windows)]
pub const BINARY_EXT: &str = "exe";
#[cfg(unix)]
pub const BINARY_EXT: &str = "out";

#[cfg(windows)]
pub const LIBARY_EXT: &str = "dll";
#[cfg(unix)]
pub const LIBARY_EXT: &str = "so";

#[cfg(windows)]
pub const LIBARY_LD_FLAG: &str = "--dll";
#[cfg(unix)]
pub const LIBARY_LD_FLAG: &str = "--shared";

#[cfg(windows)]
pub const LIBARY_LINK_LD_OPT: &str = "-l";
#[cfg(unix)]
pub const LIBARY_LINK_LD_OPT: &str = "-l:";

#[cfg(windows)]
pub const LIBARY_LINK_LD_OPTI: &str = "";
#[cfg(unix)]
pub const LIBARY_LINK_LD_OPTI: &str = ".so";
//! A thin wrapper around anyhow for easy use

#[doc(no_inline)]
pub use anyhow::{Context, Result, Error};

/// Logging message based on [tracing](https://github.com/tokio-rs/tracing) library
///
/// If you want to use it in application or unit test, you must install a global log collector at
/// the beginning of your main or test function such as
///
///     tracing_subscriber::fmt::init();
///
/// see [tracing-subscriber](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/index.html) for detail.
///
/// To log in specific trace level, do something such as
///
///     logmsg!(TRACE, "some msg");
///     logmsg!(TRACE, "some {}", msg);
///
/// Other alternative log types are `DEBUG`, `INFO`, `WARN`, `ERROR`.
///
/// As a side note, you can override above log level through `RUST_LOG` env var.
///
#[macro_export]
macro_rules! logmsg {
    (TRACE, $msg:literal) => {
        tracing::trace!("{}", $msg);
    };
    (TRACE, $fmt:expr, $($arg:tt)*) => {
        tracing::trace!("{}", format!($fmt, $($arg)*));
    };

    (DEBUG, $msg:literal) => {
        tracing::debug!("{}", $msg);
    };
    (DEBUG, $fmt:expr, $($arg:tt)*) => {
        tracing::debug!("{}", format!($fmt, $($arg)*));
    };

    (INFO, $msg:literal) => {
        tracing::info!("{}", $msg);
    };
    (INFO, $fmt:expr, $($arg:tt)*) => {
        tracing::info!("{}", format!($fmt, $($arg)*));
    };

    (WARN, $msg:literal) => {
        tracing::warn!("{}", $msg);
    };
    (WARN, $fmt:expr, $($arg:tt)*) => {
        tracing::warn!("{}", format!($fmt, $($arg)*));
    };

    (ERROR, $msg:literal) => {
        tracing::error!("{}", $msg);
    };
    (ERROR, $fmt:expr, $($arg:tt)*) => {
        tracing::error!("{}", format!($fmt, $($arg)*));
    };
}

/// Error wrapper of anyhow which provides you error message with line number and ergonomic experiences
///
/// This error wrapper can only be used in function that returns anyhow's `Result`, for example
///
///     use errlog::{wraperr, logmsg, Result};
///     use std::fs::File;
///     use std::io::Read;
///
///     fn test() -> Result<String> {
///         let filepath = "/path/to/file";
///         let mut f = wraperr!(File::open(filepath), "failed to open file {}", filepath)?;
///         let mut content = String::new();
///         wraperr!(f.read_to_string(&mut content), "failed to read content from {}", filepath)?;
///         Ok(content)
///     }
///
/// In default, wraperr log error in `ERROR` level, you can specify the level explicitly, for
/// example
///
///     wraperr!(TRACE, f.read_to_string(&mut content), "failed to read content from {}", filepath)?;
///     wraperr!(DEBUG, f.read_to_string(&mut content), "failed to read content from {}", filepath)?;
///
/// If you want to get backtrace error from anyhow, you can do as followings
///
///     for err in errlog::backtrace_anyhow(main()) {
///         logmsg!(ERROR, "{err}");
///     }
///
#[macro_export]
macro_rules! wraperr {
    (TRACE, $expr:expr) => {
        wraperr!(__anyhow "TRACE", $expr)
    };
    (TRACE, $expr:expr, $msg:literal $(,)?) => {
        wraperr!(__anyhow "TRACE", $expr, $msg)
    };
    (TRACE, $expr:expr, $fmt:expr, $($arg:tt)*) => {
        wraperr!(__anyhow "TRACE", $expr, $fmt, $($arg)*)
    };

    (DEBUG, $expr:expr) => {
        wraperr!(__anyhow "DEBUG", $expr)
    };
    (DEBUG, $expr:expr, $msg:literal $(,)?) => {
        wraperr!(__anyhow "DEBUG", $expr, $msg)
    };
    (DEBUG, $expr:expr, $fmt:expr, $($arg:tt)*) => {
        wraperr!(__anyhow "DEBUG", $expr, $fmt, $($arg)*)
    };

    (INFO, $expr:expr) => {
        wraperr!(__anyhow "INFO", $expr)
    };
    (INFO, $expr:expr, $msg:literal $(,)?) => {
        wraperr!(__anyhow "INFO", $expr, $msg)
    };
    (INFO, $expr:expr, $fmt:expr, $($arg:tt)*) => {
        wraperr!(__anyhow "INFO", $expr, $fmt, $($arg)*)
    };

    (WARN, $expr:expr) => {
        wraperr!(__anyhow "WARN", $expr)
    };
    (WARN, $expr:expr, $msg:literal $(,)?) => {
        wraperr!(__anyhow "WARN", $expr, $msg)
    };
    (WARN, $expr:expr, $fmt:expr, $($arg:tt)*) => {
        wraperr!(__anyhow "WARN", $expr, $fmt, $($arg)*)
    };

    (ERROR, $expr:expr) => {
        wraperr!(__anyhow "ERROR", $expr)
    };
    (ERROR, $expr:expr, $msg:literal $(,)?) => {
        wraperr!(__anyhow "ERROR", $expr, $msg)
    };
    (ERROR, $expr:expr, $fmt:expr, $($arg:tt)*) => {
        wraperr!(__anyhow "ERROR", $expr, $fmt, $($arg)*)
    };
    ($expr:expr) => {
        wraperr!(__anyhow "ERROR", $expr)
    };
    ($expr:expr, $msg:literal $(,)?) => {
        wraperr!(__anyhow "ERROR", $expr, $msg)
    };
    ($expr:expr, $fmt:expr, $($arg:tt)*) => {
        wraperr!(__anyhow "ERROR", $expr, $fmt, $($arg)*)
    };

    (__anyhowmsg $typ:literal, $msg:expr) => {
        match $typ {
            "TRACE" => {
                tracing::trace!("{}", $msg);
            }
            "DEBUG" => {
                tracing::debug!("{}", $msg);
            }
            "INFO" => {
                tracing::info!("{}", $msg);
            }
            "WARN" => {
                tracing::warn!("{}", $msg);
            }
            "ERROR" => {
                tracing::error!("{}", $msg);
            }
            _ => {}
        }
    };

    (__anyhow $typ:literal, $expr:expr) => {
        {
            use $crate::Context;
            $expr.with_context(|| {
                let msg = format!("{}:{}", file!(), line!());
                wraperr!(__anyhowmsg $typ, msg);
                "".to_string()
            })
        }
    };
    (__anyhow $typ:literal, $expr:expr, $msg:literal $(,)?) => {
        {
            use $crate::Context;
            $expr.with_context(|| {
                let msg = format!("{}:{} => {}", file!(), line!(), $msg);
                wraperr!(__anyhowmsg $typ, msg);
                "".to_string()
            })
        }
    };
    (__anyhow $typ:literal, $expr:expr, $fmt:expr, $($arg:tt)*) => {
        {
            use $crate::Context;
            $expr.with_context(|| {
                let msg = format!("{}:{} => {}", file!(), line!(), format!($fmt, $($arg)*));
                wraperr!(__anyhowmsg $typ, msg);
                "".to_string()
            })
        }
    };
}

/// Convert anyhow::Result into a list of string if the result is Error
pub fn backtrace_anyhow<T>(err: Result<T>) -> Vec<String> {
    let mut errmsg = vec![];
    if let Err(err) = err {
        err.chain().skip(1).for_each(|cause| {
            let cause = cause.to_string();
            if cause.len() > 0 {
                errmsg.push(cause);
            }
        });
    }
    return errmsg;
}

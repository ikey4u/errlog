pub use anyhow::{self as Anyhow, Context as AnyContext, Result as AnyResult};

#[macro_export]
macro_rules! elog {
    ($msg:literal $(,)?) => {
        {
            use errlog::Anyhow;
            Anyhow::anyhow!(format!("[{}].[{}]: {}", file!(), line!(), $msg))
        }
    };
    ($fmt:expr, $($arg:tt)*) => {
        {
            use errlog::Anyhow;
            Anyhow::anyhow!(format!("[{}].[{}]: {}", file!(), line!(), format!($fmt, $($arg)*)))
        }
    };
}

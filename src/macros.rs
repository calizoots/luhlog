/// The base macro for all logging in **`luhlog`**.
///
/// This macro is normally not used directly. Instead, you’ll use one of the
/// shorthand macros like [`trace!`], [`debug!`], [`info!`], [`warn!`], or [`error!`].
///
/// However, `log!` gives you full control — you can:
/// - specify a **target** (useful for namespacing or filtering logs),
/// - set a custom [`Level`],
/// - or pass a fully constructed [`LogRecord`].
///
/// ## Provided Forms
/// ----------------------------------------------------------------
/// ```ignore
/// log!(target: "net", Level::Warn, "disconnected: {}", peer);
/// log!(Level::Info, "application started");
/// log!(record); // directly log a LogRecord
/// ```
///
/// ## Example
/// ----------------------------------------------------------------
/// ```
/// use luhlog::{set_logger, log, Level};
/// fn main() {
///     set_logger!(level: Level::Trace);
///     log!(Level::Info, "custom log call <3");
///     log!(target: "network", Level::Warn, "ping timeout!");
/// }
/// ```
/// ----------------------------------------------------------------
#[macro_export]
macro_rules! log {
    (target: $target:expr, $level:expr, $($arg:tt)*) => {{
        use std::fmt::Write as _;
        let mut s = String::new();
        let _ = write!(&mut s, $($arg)*);
        let record = $crate::LogRecord::new($level, s)
            .with_target($target)
            .with_location(file!(), line!());
        let logger = $crate::get_logger();
        if logger.enabled($level) {
            logger.log(record);
        }
    }};

    ($level:expr, $($arg:tt)*) => {{
        use std::fmt::Write as _;
        let mut s = String::new();
        let _ = write!(&mut s, $($arg)*);
        let record = $crate::LogRecord::new($level, s)
            .with_location(file!(), line!());
        let logger = $crate::get_logger();
        if logger.enabled($level) {
            logger.log(record);
        }
    }};

    ($record:expr) => {{
        let logger = $crate::get_logger();
        if logger.enabled($record.level) {
            logger.log($record);
        }
    }};
}

/// Logs a message at [`Level::Trace`].
///
/// This is the lowest verbosity level, intended for tracing things
/// like println! debugging ;)
///
/// ## Provided Forms
/// ----------------------------------------------------------------
/// ```ignore
/// trace!("a simple message");
/// trace!(target: "math", "x = {}, y = {}", 10, 20);
/// ```
///
/// ## Example
/// ----------------------------------------------------------------
/// ```
/// use luhlog::{set_logger, trace, Level};
/// fn main() {
///     set_logger!(level: Level::Trace);
///     trace!(target: "math", "x = {}, y = {}", 10, 20);
/// }
/// ```
/// ----------------------------------------------------------------
#[macro_export]
macro_rules! trace {
    (target: $target:expr, $($arg:tt)*) => {
        $crate::log!(target: $target, $crate::Level::Trace, $($arg)*)
    };
    ($($arg:tt)*) => {
        $crate::log!($crate::Level::Trace, $($arg)*)
    };
}

/// Logs a message at [`Level::Debug`].
///
/// Use this for development logs.
///
/// ## Provided Forms
/// ----------------------------------------------------------------
/// ```ignore
/// debug!("a simple message");
/// debug!(target: "auth", "user {:?} logged in", user_id);
/// ```
///
/// ## Example
/// ----------------------------------------------------------------
/// ```
/// use luhlog::{set_logger, debug, Level};
/// fn main() {
///     set_logger!(level: Level::Debug);
///     debug!("created user {:?}", "luh_log");
/// }
/// ```
/// ----------------------------------------------------------------
#[macro_export]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)*) => {
        $crate::log!(target: $target, $crate::Level::Debug, $($arg)*)
    };
    ($($arg:tt)*) => {
        $crate::log!($crate::Level::Debug, $($arg)*)
    };
}

/// Logs a message at [`Level::Info`].
///
/// Use this for general purpose logs.
///
/// ## Provided Forms
/// ----------------------------------------------------------------
/// ```ignore
/// info!("a simple message");
/// info!(target: "server", "server started on port {}", 8080);
/// ```
///
/// ## Example
/// ----------------------------------------------------------------
/// ```
/// use luhlog::{set_logger, info, Level};
/// fn main() {
///     set_logger!(level: Level::Info);
///     info!("server started on port {}", 8080);
/// }
/// ```
/// ----------------------------------------------------------------
#[macro_export]
macro_rules! info {
    (target: $target:expr, $($arg:tt)*) => {
        $crate::log!(target: $target, $crate::Level::Info, $($arg)*)
    };
    ($($arg:tt)*) => {
        $crate::log!($crate::Level::Info, $($arg)*)
    };
}

/// Logs a message at [`Level::Warn`].
///
/// Used for events that aren’t errors but might need attention.
///
/// ## Provided Forms
/// ----------------------------------------------------------------
/// ```ignore
/// warn!("a simple message");
/// warn!(target: "storage", "disk space running low!");
/// ```
///
/// ## Example
/// ----------------------------------------------------------------
/// ```
/// use luhlog::{set_logger, warn, Level};
/// fn main() {
///     set_logger!(level: Level::Warn);
///     warn!("disk space running low!");
/// }
/// ```
/// ----------------------------------------------------------------
#[macro_export]
macro_rules! warn {
    (target: $target:expr, $($arg:tt)*) => {
        $crate::log!(target: $target, $crate::Level::Warn, $($arg)*)
    };
    ($($arg:tt)*) => {
        $crate::log!($crate::Level::Warn, $($arg)*)
    };
}

/// Logs a message at [`Level::Error`].
///
/// Use for any errors!!
///
/// ## Provided Forms
/// ----------------------------------------------------------------
/// ```ignore
/// error!("a simple message");
/// error!(target: "config", "failed to open config file: {}", "settings.toml");
/// ```
///
/// ## Example
/// ----------------------------------------------------------------
/// ```
/// use luhlog::{set_logger, error, Level};
/// fn main() {
///     set_logger!(level: Level::Error);
///     error!("failed to open config file: {}", "settings.toml");
/// }
/// ```
/// ----------------------------------------------------------------
#[macro_export]
macro_rules! error {
    (target: $target:expr, $($arg:tt)*) => {
        $crate::log!(target: $target, $crate::Level::Error, $($arg)*)
    };
    ($($arg:tt)*) => {
        $crate::log!($crate::Level::Error, $($arg)*)
    };
}

/// A convenient macro for setting up the global logger.
///
/// You can use it to:
/// - directly pass a custom [`Logger`] or any type implementing [`Log`]
/// - or configure a default [`Logger`] with a specific [`Level`] and optional formatter.
///
/// This macro wraps the given logger in an [`Arc`] and registers it globally.
///
/// ## Provided Forms
/// ----------------------------------------------------------------
/// ```ignore
/// set_logger!(logger_instance);
/// set_logger!(level: Level::Info);
/// set_logger!(level: Level::Debug, formatter: CompactFormatter);
/// ```
///
/// ## Examples
/// ----------------------------------------------------------------
/// ```
/// use luhlog::{set_logger, Level, CompactFormatter};
/// fn main() {
///     // simplest form
///     set_logger!(level: Level::Info);
///     // with a custom formatter
///     set_logger!(level: Level::Trace, formatter: CompactFormatter);
/// }
/// ```
/// ----------------------------------------------------------------
/// ```
/// use luhlog::{set_logger, Logger, Level};
/// use std::sync::Arc;
/// fn main() {
///     // using a manually constructed logger
///     let my_logger = Logger::new(Level::Debug);
///     set_logger!(my_logger);
/// }
/// ```
/// ----------------------------------------------------------------
#[macro_export]
macro_rules! set_logger {
    ($logger:expr) => {{
        use std::sync::Arc;
        let logger: Arc<dyn $crate::Log> = Arc::new($logger);
        $crate::set_logger(logger);
    }};

    (level: $level:expr $(, formatter: $formatter:expr)?) => {{
        use std::sync::Arc;
        let logger = {
            #[allow(unused_mut)]
            let mut logger = $crate::Logger::new($level);
            $(
                logger.set_formatter(Arc::new($formatter));
            )?
            logger
        };
        let logger: Arc<dyn $crate::Log> = Arc::new(logger);
        $crate::set_logger(logger);
    }};
}

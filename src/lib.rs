//! [![github]](https://github.com/calizoots/luhlog)&ensp;[![crates-io]](https://crates.io/crates/luhlog)&ensp;[![docs-rs]](https://docs.rs/luhlog)
//!
//! [github]: https://img.shields.io/badge/github-calizoots/luhlog-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/crates/v/luhlog.svg?style=for-the-badge&color=fc8d62&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-luhlog-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! # LuhLog
//! ---------------------------------------------------------------------
//! A horrible way to log your messages. You would have to be sick
//! to use this. Please take care of your mental health >.<
//! > It is made with love though s.c <3 2025. LKK FREE BINE
//!
//! So as a library we expose `luhlog::Logger` and trait `luhlog::Log`
//! for making custom log and also exposing `luhlog::LogFormatter` for
//! formatting.
//!
//! `luhlog::Level` has 6 levels similar to the log crate with Trace 
//! being the lowest and Critical being the heighest in terms of precedence 
//! we have corresponding macros for those levels for now they only 
//! correspond to the global logger instance through get_logger().
//! 
//! > This is still in dev stages I will ammend that soon.
//! > Thank you for your patience.
//! 
//! We also provide `luhlog::GlobalLogger` for making your own global
//! logger instance. Look at examples below for usage...
//!
//!
//! ## Examples
//! ---------------------------------------------------------------------
//! ```
//! use luhlog::{set_logger, Level, CompactFormatter, info, warn, trace};
//! fn main() {
//!     // level sets the requirement... logs below this wont be logged
//!     // formatter sets the formatter needs to implement LogFormatter
//!     set_logger!(level: Level::Info, formatter: CompactFormatter);
//!     info!("hello world");
//!     warn!(target: "main", "targeting main <3");
//!     // wont be printed
//!     trace!("bine");
//! }
//! ```
//! ---------------------------------------------------------------------
//! ```
//! use luhlog::{set_logger, get_logger, Level, LogBuilder, Log, Logger, CompactFormatter};
//! fn main() {
//!     set_logger!(level: Level::Trace);
//!     let logger = get_logger();
//!     let other_logger = Logger::with_formatter(
//!         Level::Info,
//!         std::sync::Arc::new(CompactFormatter)
//!     ).no_stdout().file("test.log").expect("failed to open test.log");
//!     // this allows for finer control of what your printing
//!     logger.log(
//!         // this has a default level of Info
//!         LogBuilder::new("hello guys <3")
//!             .level(Level::Trace)
//!             .build()
//!     );
//!     other_logger.log(
//!         LogBuilder::new("in the file >.<")
//!             .target("main")
//!             .level(Level::Warn)
//!             .location(file!(), line!())
//!             .build()
//!     );
//! }
//! ```
//! ---------------------------------------------------------------------
//! ```
//! use luhlog::{GlobalLogger, Log, Level, LogBuilder};
//! static LOG: GlobalLogger = GlobalLogger::new();
//! fn main() {
//!     LOG.get().log(
//!         LogBuilder::new("test for GlobalLogger :3")
//!             .target("LOG")
//!             .level(Level::Info)
//!             .location(file!(), line!())
//!             .build()
//!     );
//! }
//! ```
//! ---------------------------------------------------------------------
//! Hope you guys enjoy... any features or issues please message me
//! * Thank you for reading :)

pub mod macros;

#[cfg(test)]
mod tests;

use std::fmt;
use std::io::{Write, self};
use std::path::PathBuf;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::sync::{Arc, Mutex, RwLock};

use chrono::{DateTime, Local};

/// **`luhlog::Level`** is an enum representing the verbosity of a given message
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Level {
    /// "Trace" level typically for very verbose logs or "Tracing"
    Trace = 0,
    /// "Debug" level pretty self explanatory. Used for things more
    /// permanent than `Level::Tracing` but still Debug logs.
    Debug = 1,
    /// Info level... can be used for anything that isn't a
    /// warning but should remain for production
    Info = 2,
    /// "Warning" level
    Warn = 3,
    /// "Error" level
    /// note: no classification between errors for now.
    Error = 4,
    /// "Critical" level no macro for this level
    /// Only added as an edge case
    Critical = 5
}

impl Level {
    /// returns the string version of a given `luhlog::Level`
    pub fn name(&self) -> &str {
        match self {
            Level::Trace => "trace",
            Level::Debug => "debug",
            Level::Info => "info",
            Level::Warn => "warn",
            Level::Error => "error",
            Level::Critical => "critical"
        }
    }

    // (We all built not to change... I just cocked one in and I'm
    // rolling. Out the drop-top I'm thugging like... who want it?)

    // #[cfg(not(feature = "color"))]
    // pub fn to_string(&self) -> String {
    //     return self.name().to_string()
    // }

    pub fn to_string(&self) -> String {
        use luhcore::color::Colorise;
        // kmt this is a bit long would like a better solution
        // would like it to be customisable sumn like that

        let name = self.name().to_string();
        
        let text = match self {
            Level::Trace => name.green(),
            Level::Debug => name.magenta(),
            Level::Info  => name.blue(),
            Level::Warn  => name.yellow(),
            Level::Error => name.red(),
            Level::Critical => name.red().bold()
        };

        return text.to_string()
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// `luhlog::LogRecord` carries all appropriate metadata for a given
/// log entry. There are two ways this is useful to you.
///
/// ## Provided Methods
/// ----------------------------------------------------------------
///
/// ### `new(level: Level, msg: impl Into<String>) -> Self`
/// Creates a new log record with the provided [`Level`] and message.
///
/// ### `with_target(target: impl Into<String>) -> Self`
/// Sets the record’s target (e.g., subsystem, module, etc).
///
/// ### `with_location(file: impl Into<String>, line: u32) -> Self`
/// Attaches file and line information to the record.
///
/// ### `with_metadata(key: impl Into<String>, value: impl Into<String>) -> Self`
/// Adds a custom metadata key–value pair to the record.
///
///
/// ## Examples
/// ----------------------------------------------------------------
/// > First example here is using `LogRecord::new` directly.
/// ```
/// use luhlog::{set_logger, get_logger, LogRecord, Log, Level};
/// fn main() {
///     set_logger!(level: Level::Trace);
///     let logger = get_logger();
///     logger.log(
///         LogRecord::new(Level::Info, "bonjour people :3")
///             .with_target("main")
///             .with_location(file!(), line!())
///     );
/// }
/// ```
/// ----------------------------------------------------------------
/// > Second example here is using `LogBuilder`.
/// ```
/// use luhlog::{set_logger, get_logger, LogBuilder, Log, Level};
/// fn main() {
///     set_logger!(level: Level::Trace);
///     let logger = get_logger();
///     logger.log(
///         // this has a default level of Info
///         LogBuilder::new("hello guys <3")
///             .level(Level::Info)
///             .target("main")
///             .location(file!(), line!())
///             .build()
///     );
/// }
/// ```
/// ----------------------------------------------------------------
#[derive(Clone)]
pub struct LogRecord {
    /// The record's message
    pub msg: String,
    /// The record's log presidence level
    pub level: Level,
    /// An optional abritary name I am calling a "target"
    pub target: Option<String>,
    /// The record's time (using `chrono` chrate)
    pub timestamp: DateTime<Local>,
    /// An optional file name which the log message came from
    pub file: Option<String>,
    /// An optional line number which the log message came from
    pub line: Option<u32>,
    /// Arbitrary Key-Value data
    pub metadata: HashMap<String, String>,
}

impl LogRecord {
    pub fn new(level: Level, msg: impl Into<String>) -> Self {
        Self {
            msg: msg.into(),
            level,
            target: None,
            timestamp: Local::now(),
            file: None,
            line: None,
            metadata: HashMap::new(),
        }
    }

    /// Changes the record's target
    pub fn with_target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    /// Changes the record's location
    pub fn with_location(mut self, file: impl Into<String>, line: u32) -> Self {
        self.file = Some(file.into());
        self.line = Some(line);
        self
    }

    /// Adds to the record's metadata
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// `luhlog::LogBuilder` is an easy way to build a `LogRecord`.
/// Doesn't really do much apart from shorter combinators and
/// a predefined level for the record.
///
/// ## Provided Methods
/// ----------------------------------------------------------------
///
/// ### `new(msg: impl Into<String>) -> Self`
/// Creates a new builder with a default [`Level::Info`] and message.
///
/// ### `level(level: Level) -> Self`
/// Sets the log level for this record.
///
/// ### `target(target: impl Into<String>) -> Self`
/// Sets the target (abritary name).
///
/// ### `location(file: impl Into<String>, line: u32) -> Self`
/// Attaches file and line info to the record.
///
/// ### `metadata(key: impl Into<String>, value: impl Into<String>) -> Self`
/// Adds an arbitrary key–value pair to the record’s metadata.
///
/// ### `build() -> LogRecord`
/// Finalises and returns the constructed [`LogRecord`].
///
/// ## Example
/// ----------------------------------------------------------------
/// ```
/// use luhlog::{set_logger, get_logger, LogBuilder, Log, Level};
/// fn main() {
///     set_logger!(level: Level::Trace);
///     let logger = get_logger();
///     logger.log(
///         // this has a default level of Info
///         LogBuilder::new("hello guys <3")
///             .level(Level::Info)
///             .target("main")
///             .location(file!(), line!())
///             .build()
///     );
/// }
/// ```
/// ----------------------------------------------------------------
pub struct LogBuilder {
    record: LogRecord,
}

impl LogBuilder {
    pub fn new(msg: impl Into<String>) -> Self {
        Self {
            record: LogRecord::new(Level::Info, msg)
        }
    }

    /// Changes the record's level.
    pub fn level(mut self, level: Level) -> Self {
        self.record.level = level;
        self
    }

    /// Changes the record's target.
    pub fn target(mut self, target: impl Into<String>) -> Self {
        self.record.target = Some(target.into());
        self
    }

    /// Changes the record's location.
    pub fn location(mut self, file: impl Into<String>, line: u32) -> Self {
        self.record.file = Some(file.into());
        self.record.line = Some(line);
        self
    }

    /// Adds to the record's metadata
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.record.metadata.insert(key.into(), value.into());
        self
    }

    /// Return the built record.
    pub fn build(self) -> LogRecord {
        self.record
    }
}

/// `luhlog::LogFormatter` decides **how your logs look**.
///
/// This trait is what `luhlog::Logger` uses internally to turn a raw
/// [`LogRecord`](crate::LogRecord) into something actually readable.
///
/// By default, we ship a few built-ins like:
/// - [`DefaultFormatter`](crate::DefaultFormatter) → timestamp, level, file, line, etc  
/// - [`CompactFormatter`](crate::CompactFormatter) → small, just level + msg  
/// - [`JsonFormatter`](crate::JsonFormatter) → *“please don’t use this in prod” edition*
///
/// But of course, you can make your own formatter that fits your needs/wants.
///
/// ## Provided Methods
/// ----------------------------------------------------------------
///
/// ### `time_format(&self) -> &str`
///
/// Returns a strftime-style format string used for timestamps.
/// Default is `"%H:%M:%S"`.
///
/// ```ignore
/// fn time_format(&self) -> &str {
///     "%Y-%m-%d %H:%M:%S"
/// }
/// ```
///
/// ### `format(&self, record: &LogRecord) -> String`
///
/// Called for every log record to generate the final output string.
///
/// ```ignore
/// fn format(&self, record: &LogRecord) -> String {
///     format!("[{}] >> {}", record.level, record.msg)
/// }
/// ```
///
/// ## Example
/// ----------------------------------------------------------------
/// ```
/// use luhlog::{LogFormatter, Logger, set_logger, Level, LogRecord, info};
///
/// #[derive(Debug)]
/// struct MyFormatter;
///
/// impl LogFormatter for MyFormatter {
///     fn format(&self, record: &LogRecord) -> String {
///         format!(">>> {} <<<", record.msg)
///     }
/// }
///
/// fn main() {
///     set_logger!(level: Level::Trace, formatter: MyFormatter);
///     info!("new custom format");
/// }
/// ```
/// ----------------------------------------------------------------
pub trait LogFormatter: Send + Sync {
    fn time_format(&self) -> &str {
        return "%H:%M:%S";
    }
    fn format(&self, record: &LogRecord) -> String;
}

/// In the name a default formatter <3
#[derive(Debug, Clone)]
pub struct DefaultFormatter;

impl LogFormatter for DefaultFormatter {
    fn time_format(&self) -> &str {
        return "%H:%M:%S";
    }

    fn format(&self, record: &LogRecord) -> String {
        let target = record.target.as_deref();
        let timestamp = record.timestamp.format(self.time_format());

        if let (Some(file), Some(line), Some(target)) = (&record.file, record.line, target) {
            format!(
                "[{}] [{:5}] ({}) {}:{} -> {}",
                timestamp, record.level, target, file, line, record.msg
            )
        } else {
            format!(
                "[{}] [{:5}] {}",
                timestamp, record.level, record.msg
            )
        }
    }
}

/// Small basic formatter
#[derive(Debug, Clone)]
pub struct CompactFormatter;

impl LogFormatter for CompactFormatter {
    fn format(&self, record: &LogRecord) -> String {
        format!("[{}] {}", record.level, record.msg)
    }
}

/// `luhlog::JsonFormatter` **barely works at all if you are serious
/// about doing this write your own implementation using serde escaping
/// this will not work in prod!!!**
#[derive(Debug, Clone)]
pub struct JsonFormatter;

impl LogFormatter for JsonFormatter {
    fn format(&self, record: &LogRecord) -> String {
        let timestamp = record.timestamp.format("%Y-%m-%d %H:%M:%S");
        
        let escaped_msg = record.msg.replace('"', "\\\"");

        format!(
            r#"{{"timestamp":"{}","level":"{}","target":"{}","message":"{}" }}"#,
            timestamp,
            record.level.name(),
            record.target.as_deref().unwrap_or("app"),
            escaped_msg,
        )
    }
}

/// `luhlog::Log` is the core trait that powers all logging backends.
///
/// Implementing this allows you to define how your logs are handled —
/// whether you want to print them, write them to disk, send them over
/// a socket, or whatever you want g.
///
/// The default provided [`Logger`](crate::Logger) struct already implements
/// this trait, but if you want complete control, make your own.
///
/// ## Required Methods
/// ----------------------------------------------------------------
///
/// ### `enabled(&self, level: Level) -> bool`
/// Used to determine if a given log level should be processed.
/// This is where you can implement filtering logic, e.g.:
/// ```ignore
/// fn enabled(&self, level: Level) -> bool {
///     level >= self.min_level
/// }
/// ```
///
/// ### `log(&self, record: LogRecord)`
/// This is called when an event is ready to be logged.  
/// Handle it however you like — format it, send it, print it, etc.
///
/// ```ignore
/// fn log(&self, record: LogRecord) {
///     let formatted = self.formatter.format(&record);
///     println!("{}", formatted);
/// }
/// ```
///
/// ### `flush(&self)`
/// Optional cleanup method. Called when the logger is dropped or
/// when [`flush()`](crate::flush) is manually invoked.  
/// Perfect for file-based or buffered loggers.
///
/// ## Example
/// ----------------------------------------------------------------
/// ```
/// use luhlog::{Log, LogRecord, Level};
/// struct MyLogger;
/// impl Log for MyLogger {
///     fn enabled(&self, level: Level) -> bool {
///         level >= Level::Info
///     }
///     fn log(&self, record: LogRecord) {
///         println!("[{}] {}", record.level, record.msg);
///     }
///     fn flush(&self) {
///         println!("flushing logs :)");
///     }
/// }
/// ```
/// ----------------------------------------------------------------
///
/// > ✨ tip: you almost never need to implement this manually unless
/// you’re doing something different — like remote log streaming or sumn.
/// Otherwise, [`Logger`](crate::Logger) is plenty.
pub trait Log: Send + Sync + 'static {
    fn enabled(&self, level: Level) -> bool;
    fn log(&self, record: LogRecord);
    fn flush(&self) {}
}

/// `luhlog::Logger` is the main logging backend implementation.
/// It handles log filtering, formatting, writing to files, and optional
/// output to stdout.
///
/// The `Logger` implements [`luhlog::Log`], so it can be used
/// anywhere a generic `Log` trait object is expected.  
///
/// ## Provided Methods
/// ----------------------------------------------------------------
///
/// ### `new(min_level: Level) -> Self`  
/// Creates a new logger with the given minimum level and a
/// [`DefaultFormatter`] attached.
///
/// ### `with_formatter(min_level: Level, formatter: Arc<dyn LogFormatter>) -> Self`  
/// Creates a logger with a custom formatter instead of the default one.
///
/// ### `set_formatter(&self, new_fmt: Arc<dyn LogFormatter>)`  
/// Replaces the formatter of the current logger instance.
///
/// ### `file(path: impl Into<PathBuf>) -> Result<Self, io::Error>`  
/// Enables file logging. Creates (or appends to) a file at the given path.
/// Returns a `Result<Self>` for error handling.
///
/// ### `level(&self) -> Level`  
/// Returns the current minimum log level for this logger.
///
/// ### `no_stdout(self) -> Self`  
/// Disables logging to stdout entirely (useful for file-only logging).
///
/// ### `with_stdout(self) -> Self`  
/// Re-enables logging to stdout if previously disabled.
///
/// ### `get_records(&self) -> Vec<LogRecord>`  
/// Returns a **cloned** vector of all log records stored in memory.
/// This can be expensive for large logs—use carefully.
///
/// ### `clear_records(&self)`  
/// Clears all stored log records from memory.
///
///
/// ## Examples
/// ----------------------------------------------------------------
///
/// ```
/// use luhlog::{Logger, Log, Level, DefaultFormatter, LogBuilder};
/// use std::sync::Arc;
/// fn main() {
///     // create a logger with a default formatter
///     let logger = Logger::new(Level::Info);
///     // log something
///     logger.log(
///         LogBuilder::new("Hello world from the logger!")
///             .level(Level::Info)
///             .build()
///     );
///     // switch to a compact formatter mid-run
///     use luhlog::CompactFormatter;
///     logger.set_formatter(Arc::new(CompactFormatter));
///     logger.log(
///         LogBuilder::new("Compact mode enabled!")
///             .level(Level::Warn)
///             .build()
///     );
/// }
/// ```
/// ----------------------------------------------------------------
///
/// ```
/// use luhlog::{Logger, Log, Level, LogBuilder};
/// fn main() {
///     // logging to a file instead of stdout
///     let logger = Logger::new(Level::Info)
///         .file("example.log")
///         .expect("failed to open file");
///     logger.log(
///         LogBuilder::new("This will go into the log file!")
///             .level(Level::Info)
///             .build()
///     );
/// }
/// ```
/// ----------------------------------------------------------------
pub struct Logger {
    /// The minimum logging level that will be recorded.
    pub min_level: Level,
    /// The active formatter instance used by this logger.
    pub formatter: RwLock<Arc<dyn LogFormatter>>,
    /// In-memory store for recent log records.
    pub records: Mutex<Vec<LogRecord>>,
    /// Optional file handle for persistent logging.
    file: Option<Mutex<File>>,
    /// Whether logs are printed to stdout.
    print_to_stdout: bool,
}


impl Logger {
    pub fn new(min_level: Level) -> Self {
        Self::with_formatter(min_level, Arc::new(DefaultFormatter))
    }

    pub fn set_formatter(&self, new_fmt: Arc<dyn LogFormatter>) {
        let mut f = self.formatter.write().unwrap();
        *f = new_fmt;
    }

    pub fn with_formatter(min_level: Level, formatter: Arc<dyn LogFormatter>) -> Self {
        Logger {
            min_level,
            formatter: RwLock::new(formatter),
            records: Mutex::new(vec![]),
            file: None,
            print_to_stdout: true,
        }
    }

    pub fn file(mut self, path: impl Into<PathBuf>) -> Result<Self, io::Error> {
        self.file = Some(Mutex::new(
            OpenOptions::new()
                .append(true)
                .create(true)
                .open(path.into())?
        ));
        Ok(self)
    }

    pub fn level(&self) -> Level {
        self.min_level
    }

    pub fn no_stdout(mut self) -> Self {
        self.print_to_stdout = false;
        self
    }

    pub fn with_stdout(mut self) -> Self {
        self.print_to_stdout = true;
        self
    }

    /// THIS DOES CLONE
    /// So use it carefully if you care about performance
    pub fn get_records(&self) -> Vec<LogRecord> {
        self.records.lock().unwrap().clone()
    }

    pub fn clear_records(&self) {
        self.records.lock().unwrap().clear();
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.flush();
    }
}

impl Log for Logger {
    fn enabled(&self, level: Level) -> bool {
        level >= self.min_level
    }

    fn log(&self, record: LogRecord) {
        if !self.enabled(record.level) {
            return;
        }

        let formatted = self.formatter.read().unwrap().format(&record);

        if self.print_to_stdout {
            println!("{}", formatted);
        }

        if let Some(file_mutex) = &self.file {
            let mut file = file_mutex.lock().unwrap();
            let _ = writeln!(file, "{}", formatted);
        }

        self.records.lock().unwrap().push(record);
    }

    fn flush(&self) {
        if let Some(file_mutex) = &self.file {
            let mut file = file_mutex.lock().unwrap();
            let _ = file.flush();
        }
    }
}

/// `luhlog::GlobalLogger` provides a simple thread-safe global logging
/// container. It allows you to store and retrieve a single shared
/// logger instance across your entire program.
///
/// This is mainly used behind the global functions like
/// [`luhlog::set_logger`], [`luhlog::get_logger`], and
/// [`luhlog::clear_logger`], but you can also use it directly if you
/// need a custom global logging instance.
///
/// ## Provided Methods
/// ----------------------------------------------------------------
///
/// ### `new() -> Self`
/// Creates a new, empty `GlobalLogger` with no active logger set.
///
/// ### `set(&self, logger: Arc<dyn Log>)`
/// Stores a logger inside the global container.  
/// Overwrites any existing logger if one is already set.
///
/// ### `clear(&self)`
/// Removes the currently stored logger, returning the global state
/// to `None`.
///
/// ### `get(&self) -> Arc<dyn Log>`
/// Retrieves the currently stored logger instance.
/// If no logger has been set, it will automatically return a new
/// default [`Logger`] with `Level::Info`.
///
///
/// ## Examples
/// ----------------------------------------------------------------
///
/// ```
/// use luhlog::{GlobalLogger, Logger, Level, Log, LogBuilder};
/// use std::sync::Arc;
/// static GLOBAL: GlobalLogger = GlobalLogger::new();
/// fn main() {
///     // create and assign a global logger
///     let logger = Arc::new(Logger::new(Level::Trace));
///     GLOBAL.set(logger);
///     // use the global logger directly
///     GLOBAL.get().log(
///         LogBuilder::new("Hello from the global logger!")
///             .level(Level::Info)
///             .build()
///     );
/// }
/// ```
/// ----------------------------------------------------------------
///
/// ```
/// use luhlog::{GlobalLogger, Logger, Level, Log};
/// use std::sync::Arc;
/// static LOG: GlobalLogger = GlobalLogger::new();
/// fn main() {
///     // clear global logger to reset state
///     LOG.clear();
///     // retrieve will still return a default logger
///     let default_logger = LOG.get();
///     default_logger.log(
///         luhlog::LogBuilder::new("Using the default global logger")
///             .level(Level::Warn)
///             .build()
///     );
/// }
/// ```
/// ----------------------------------------------------------------
pub struct GlobalLogger {
    inner: RwLock<Option<Arc<dyn Log>>>,
}

impl GlobalLogger {
    pub const fn new() -> Self {
        Self {
            inner: RwLock::new(None),
        }
    }

    // (What it is Herm got a hundred in his choppa. He can act like
    // we can't stop him. It won't be long till we pop him. Go to
    // dumping on the runway.)

    pub fn set(&self, logger: Arc<dyn Log>) {
        let mut w = self.inner.write().unwrap();
        *w = Some(logger);
    }

    pub fn clear(&self) {
        *self.inner.write().unwrap() = None;
    }

    pub fn get(&self) -> Arc<dyn Log> {
        self.inner
            .read()
            .unwrap()
            .clone()
            .unwrap_or_else(|| Arc::new(Logger::new(Level::Info)))
    }
}

/// Global logger instance for luhlog.
///
/// This is a [`GlobalLogger`] that stores a single, shared [`Log`]
/// used by all logging macros like [`info!`], [`warn!`], and [`error!`].
///
/// By default, it’s empty and falls back to a basic `Logger` with
/// `Level::Info` until explicitly set using [`set_logger`].
static LOGGER: GlobalLogger = GlobalLogger::new();

/// Sets the global logger for this library instance.
///
/// Once set, all global logging macros (e.g. [`info!`], [`debug!`])
/// will write through this logger.
///
/// ## Example
/// ----------------------------------------------------------------
/// ```
/// use luhlog::{set_logger, Logger, Level};
/// use std::sync::Arc;
/// fn main() {
///     set_logger(Arc::new(Logger::new(Level::Trace)));
/// }
/// ```
/// ----------------------------------------------------------------
pub fn set_logger(logger: Arc<dyn Log>) {
    LOGGER.set(logger)
}

/// Clears the currently active global logger.
///
/// After calling this, any call to [`get_logger`] will automatically
/// return a new default [`Logger`] instance instead.
pub fn clear_logger() {
    LOGGER.clear()
}

/// Retrieves the active global logger instance.
///
/// If no logger has been set with [`set_logger`], a default
/// [`Logger`] with `Level::Info` will be returned automatically.
///
/// ## Example
/// ----------------------------------------------------------------
/// ```
/// use luhlog::{get_logger, LogBuilder, Level, Log};
/// fn main() {
///     let logger = get_logger();
///     logger.log(
///         LogBuilder::new("using the global logger directly")
///             .level(Level::Info)
///             .build()
///     );
/// }
/// ```
/// ----------------------------------------------------------------
pub fn get_logger() -> Arc<dyn Log> {
    LOGGER.get()
}

/// Flushes the global logger’s internal records.
///
/// This simply calls [`Log::flush()`] on the current global logger.
/// If no logger is set, it flushes the default fallback logger.
pub fn flush() {
    get_logger().flush();
}


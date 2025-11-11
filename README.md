# LuhLog

[<img alt="github" src="https://img.shields.io/badge/github-calizoots/luhlog-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/calizoots/luhlog)
[<img alt="crates.io" src="https://img.shields.io/crates/v/luhlog.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/luhlog)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-luhlog-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/luhlog)

A horrible way to log your messages. You would have to be sick to use this.
<br>
Please take care of your mental health >.<

> Made with love though s.c <3 2025. LKK FREE BINE

As a library, we expose:

- `luhtwin::Logger` and trait `luhtwin::Log` for creating custom loggers.
- `luhtwin::LogFormatter` for formatting logs.
- `luhtwin::Level` with 5 levels (Trace, Debug, Info, Warn, Error), similar to the `log` crate.
- Corresponding macros (`trace!`, `debug!`, `info!`, `warn!`, `error!`) for the global logger instance through `get_logger()`.

> This is still in development stages. Thank you for your patience.

We also provide `luhtwin::GlobalLogger` for creating your own global logger instance.

---

## Examples

### 1. Basic usage

```rust
use luhlog::{set_logger, Level, CompactFormatter, info, warn, trace};

fn main() {
    // Level sets the requirement: logs below this won't be logged
    // Formatter must implement LogFormatter
    set_logger!(level: Level::Info, formatter: CompactFormatter);

    info!("hello world");
    warn!(target: "main", "targeting main <3");

    // Won't be printed because it's below the Level::Info threshold
    trace!("bine");
}
```

### 2. Using a logger directly

```rust
use luhlog::{set_logger, get_logger, Level, LogBuilder, Logger, CompactFormatter};

fn main() {
    set_logger!(level: Level::Trace);

    let logger = get_logger();

    let other_logger = Logger::with_formatter(
        Level::Info,
        std::sync::Arc::new(CompactFormatter)
    ).no_stdout().file("test.log").expect("failed to open test.log");

    // log directly with a custom LogBuilder
    logger.log(
        LogBuilder::new("hello guys <3")
            .level(Level::Trace)
            .build()
    );

    other_logger.log(
        LogBuilder::new("in the file >.<")
            .target("main")
            .level(Level::Warn)
            .location(file!(), line!())
            .build()
    );
}

```

### 3. Using GlobalLogger

```rust
use luhlog::{GlobalLogger, Log, Level, LogBuilder};

static LOG: GlobalLogger = GlobalLogger::new();

fn main() {
    LOG.get().log(
        LogBuilder::new("test for GlobalLogger :3")
            .target("LOG")
            .level(Level::Info)
            .location(file!(), line!())
            .build()
    );
}
```

### 4. Custom formatters

```rust
use luhlog::{set_logger, Level, Logger, LogBuilder};
use std::sync::Arc;

#[derive(Debug)]
struct MyFormatter;

impl luhtwin::LogFormatter for MyFormatter {
    fn format(&self, record: &luhtwin::LogRecord) -> String {
        format!(">>> {} <<<", record.msg)
    }
}

fn main() {
    let logger = Logger::with_formatter(Level::Info, Arc::new(MyFormatter));
    set_logger!(logger);

    luhtwin::info!("custom format");
}
```

### 5. No stdout

```rust
use luhlog::{set_logger, Level, Logger, info};

fn main() {
    let logger = Logger::new(Level::Info).no_stdout();
    set_logger!(logger);

    info!("this won't print to stdout");
}
```

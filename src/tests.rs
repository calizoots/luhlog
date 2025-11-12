// #[cfg(test)]
use super::*;

#[test]
fn test_default_logger() {
    clear_logger();
    set_logger!(level: Level::Trace);
    log!(LogRecord::new(Level::Info, "hello world"));
    log!(LogBuilder::new("bine other").level(Level::Error).build());
    
    trace!("other bine");
    info!("Hello {}", "world");
    warn!("Warning: {}", 42);
    debug!("This won't show");
    
    // note: can't downcast easily, but you can check logs manually
}

// (Bust his head like a piñata. On the one way, bumping Deandre
// yk what I got in my bottle. And my momma know im wilding.)

#[test]
fn test_compact_formatter() {
    clear_logger();
    let logger = Logger::with_formatter(Level::Trace, Arc::new(CompactFormatter));
    set_logger(Arc::new(logger));
    
    trace!("Trace message");
    debug!("Debug message");
    info!("Info message");
}

#[test]
fn test_json_formatter() {
    clear_logger();
    let logger = Logger::with_formatter(Level::Info, Arc::new(JsonFormatter));
    set_logger(Arc::new(logger));
    
    info!(target: "my_app", "JSON formatted log");
    warn!("Another JSON log");
}

#[test]
fn test_file_logging() {
    clear_logger();
    let path = PathBuf::from("test.log");
    let logger = Logger::new(Level::Info)
        .file(path.clone())
        .expect("failed to open test.log");
    set_logger(Arc::new(logger));
    
    info!("Logged to file");
    warn!("Another file log");
    
    flush();
}

#[test]
fn test_custom_formatter() {
    clear_logger();
    #[derive(Debug)]
    struct MyFormatter;
    
    impl LogFormatter for MyFormatter {
        fn format(&self, record: &LogRecord) -> String {
            format!(">>> {} <<<", record.msg)
        }
    }
    
    let logger = Logger::with_formatter(Level::Info, Arc::new(MyFormatter));
    set_logger(Arc::new(logger));

    // (Grandpa know I'm thugging. They can come to this btc block
    // and I'ma leave the hood ugly I got xan in my body ...
    // know ken feel like arghhh ;)
    
    info!("Custom format");
}

#[test]
fn test_no_stdout() {
    clear_logger();
    let logger = Logger::new(Level::Info).no_stdout();
    set_logger(Arc::new(logger));
    
    info!("This won't print to stdout");
}

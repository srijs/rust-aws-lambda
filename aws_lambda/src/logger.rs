use log::{self, Level, LevelFilter, Log, Metadata, Record};

/// Initialize the logging system.
pub fn init() {
    log::set_boxed_logger(Box::new(Logger)).unwrap();
    log::set_max_level(LevelFilter::Info);
}

struct Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("[{}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

#![allow(clippy::uninlined_format_args, clippy::missing_const_for_fn)]

mod timestamp;

use log::{Level, LevelFilter, Log, Metadata, Record};

pub struct PicoLogger {
    level: LevelFilter,
    colors: bool,
}

impl PicoLogger {
    /// Create (but not initialize!) a new logger instance. Must be initialized for the logger to
    /// take effect.
    ///
    /// # Example
    ///
    /// ```rust
    /// use log::{debug, LevelFilter};
    /// use picolog::PicoLogger;
    ///
    /// PicoLogger::new(LevelFilter::Trace).init();
    /// debug!("hi!");
    /// ```
    #[must_use]
    pub fn new(level: LevelFilter) -> Self {
        Self {
            level,
            colors: true,
        }
    }

    /// Enable or disable colors.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use log::LevelFilter;
    /// # use picolog::PicoLogger;
    /// PicoLogger::new(LevelFilter::Trace)
    ///     .with_colors(false)
    ///     .init();
    /// ```
    #[must_use]
    pub fn with_colors(mut self, colors: bool) -> Self {
        self.colors = colors;
        self
    }

    /// Initialize the logger.
    ///
    /// # Panics
    ///
    /// This function panics if called more than once.
    pub fn init(self) {
        log::set_max_level(self.level);
        log::set_boxed_logger(Box::new(self)).expect("logger already set");
    }
}

impl Log for PicoLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let level = {
            let level = record.level();

            let red = "\x1b[31m";
            let yellow = "\x1b[33m";
            let cyan = "\x1b[36m";
            let purple = "\x1b[35m";
            let normal = "\x1b[m";

            if self.colors {
                let color = match level {
                    Level::Error => red,
                    Level::Warn => yellow,
                    Level::Info => cyan,
                    Level::Debug => purple,
                    Level::Trace => normal,
                };

                format!("{}{:<5}{}", color, level, normal)
            } else {
                format!("{:<5}", level)
            }
        };

        let location = {
            let module = if !record.target().is_empty() {
                record.target()
            } else if let Some(path) = record.module_path() {
                path
            } else {
                "?"
            };

            module.split("::").last().unwrap_or("?")
        };

        let thread = match std::thread::current().name() {
            Some("main") | None => String::new(),
            Some(name) => format!("/{}", name),
        };

        let timestamp = timestamp::Timestamp::new();

        println!(
            "{} {} [{}{}] {}",
            timestamp,
            level,
            location,
            thread,
            record.args()
        );
    }

    fn flush(&self) {}
}

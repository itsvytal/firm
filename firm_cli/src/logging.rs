use std::sync::OnceLock;

use super::ui;
use indicatif::MultiProgress;
use indicatif_log_bridge::LogWrapper;
use log::{Level, Log, Metadata, Record};

/// Tracks indicatif progress bars so they can be stalled when outputting logs
static MULTI_PROGRESS: OnceLock<MultiProgress> = OnceLock::new();

pub fn get_multi_progress() -> &'static MultiProgress {
    MULTI_PROGRESS.get_or_init(|| MultiProgress::new())
}

/// A logger implementation that outputs crate logs to console UI messages
struct UiLogger {
    verbose: bool,
}

impl Log for UiLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        // Determine if the log originates from a firm crate
        let target = metadata.target();
        let is_firm_crate = target.contains("firm");

        // In verbose mode, set debug level for firm crates and warn level for others
        if self.verbose && is_firm_crate {
            metadata.level() <= Level::Debug
        } else {
            metadata.level() <= Level::Warn
        }
    }

    fn log(&self, record: &Record) {
        // Pipe library logs to the appropriate UI message
        if self.enabled(record.metadata()) {
            match record.level() {
                Level::Error => ui::error(&record.args().to_string()),
                Level::Warn => ui::warning(&record.args().to_string()),
                Level::Info => ui::info(&record.args().to_string()),
                Level::Debug => ui::debug(&record.args().to_string()),
                Level::Trace => ui::debug(&record.args().to_string()),
            }
        }
    }

    fn flush(&self) {}
}

/// Initialize logging for the CLI
pub fn initialize(verbose: bool) -> Result<(), log::SetLoggerError> {
    let ui_logger = UiLogger { verbose };

    // Wrap it with LogWrapper for indicatif handling and set that as the global logger
    let wrapped_logger = LogWrapper::new(get_multi_progress().clone(), Box::new(ui_logger));
    log::set_boxed_logger(Box::new(wrapped_logger))?;

    if verbose {
        log::set_max_level(log::LevelFilter::Debug);
    } else {
        log::set_max_level(log::LevelFilter::Warn);
    }

    Ok(())
}

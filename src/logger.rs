#![deny(missing_docs)]

use log::{Level, SetLoggerError};

/// The logger that will latter be broken out into a crate to hook into the anti-spam.internal.ablecorp.us regex based filtering.
/// This library will also handle getting parsing and dealing with
pub struct Logger;
impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        let _ = metadata;
        true
    }

    fn log(&self, record: &log::Record) {
        let lvl = record.level();
        let _lvl_color = match lvl {
            Level::Error => "160",
            Level::Warn => "172",
            Level::Info => "47",
            Level::Debug => "25",
            Level::Trace => "103",
        };
        let contents = record.args();
        match record.module_path() {
         Some(module) => {
            if module.contains("warp"){

            } else {
                let msg = format!("[{}] - {}", module, contents);
                println!("{}", msg);
                // format!("{}", abc)
            }
         }
         None => {
        //   format!("");
         }
        };
    
    }

    fn flush(&self) {
        todo!()
    }
}






/// Initializes the logger with appropriate log levels based on debug assertions.
/// Returns `Ok(())` if logger was set successfully, or a `SetLoggerError` if it fails.
pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&crate::logger::Logger)?;
    if cfg!(debug_assertions) {
        log::set_max_level(log::LevelFilter::Debug);
    } else {
        log::set_max_level(log::LevelFilter::Info);
    }

    Ok(())
}
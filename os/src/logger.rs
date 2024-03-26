use log::{Level, Log};
pub struct Logger;
const TRACE: u8 = 90;
const DEBUG: u8 = 32;
const INFO: u8 = 34;
const WARN: u8 = 93;
const ERROR: u8 = 31;

impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            match record.level() {
                Level::Error => {
                    println!(
                        "\x1b[{}m{}\x1b[0m",
                        ERROR,
                        format_args!("[{:>5}] {}", record.level(), record.args())
                    );
                }
                Level::Warn => {
                    println!(
                        "\x1b[{}m{}\x1b[0m",
                        WARN,
                        format_args!("[{:>5}] {}", record.level(), record.args())
                    );
                }
                Level::Info => {
                    println!(
                        "\x1b[{}m{}\x1b[0m",
                        INFO,
                        format_args!("[{:>5}] {}", record.level(), record.args())
                    );
                }
                Level::Debug => {
                    println!(
                        "\x1b[{}m{}\x1b[0m",
                        DEBUG,
                        format_args!("[{:>5}] {}", record.level(), record.args())
                    );
                }
                Level::Trace => {
                    println!(
                        "\x1b[{}m{}\x1b[0m",
                        TRACE,
                        format_args!("[{:>5}] {}", record.level(), record.args())
                    );
                }
            }
        }
    }

    fn flush(&self) {}
}

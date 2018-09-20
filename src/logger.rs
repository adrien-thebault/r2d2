//! This mod provides a common logger to keep logs consistent

use chrono;
use fern;
use fern::colors::{Color, ColoredLevelConfig};
use log;
use std::{io, process};

/// The actual logger
pub struct Logger;

impl Logger {
    /// Inits the logger using `log_level' as the current log level, effectively
    /// filtering anything below this level
    pub fn init(log_level: log::LevelFilter) {
        let colors = ColoredLevelConfig::new()
            .error(Color::BrightRed)
            .warn(Color::BrightYellow)
            .info(Color::BrightWhite)
            .debug(Color::BrightCyan)
            .trace(Color::BrightBlack);

        fern::Dispatch::new()
            .format(move |out, message, record| {
                let mut target = String::new();
                let mut splitted: Vec<&str> = record.target().split("::").collect();
                let last = splitted.pop().unwrap();
                let mut iter = splitted.into_iter();

                if let Some(first) = iter.next() {
                    target.push_str(first);
                    target.push_str("::");
                }

                for module in iter {
                    target.push(module.chars().next().unwrap());
                    target.push_str("::");
                }

                target.push_str(last);

                out.finish(format_args!(
                    "\x1B[{light}m{date}\x1B[0m \x1B[{color}m{level:5}\x1B[0m {pid:5} \x1B[{light}m--- {target:30.30}\x1B[0m : \x1B[{color}m{message}\x1B[0m",
                    light = Color::BrightBlack.to_fg_str(),
                    color = colors.get_color(&record.level()).to_fg_str(),
                    date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                    level = record.level(),
                    pid = process::id(),
                    target = target,
                    message = message
                ));
            })
            .level(log_level)
            .chain(io::stdout())
            .apply()
            .unwrap();
    }
}

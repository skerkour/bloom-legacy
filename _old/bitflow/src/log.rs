use log; // see https://github.com/rust-lang/rust/issues/56398
use slog;
use slog_json;
use slog_scope;
// use lazy_static;

use std::sync::Mutex;

pub mod macros {
    // Since slog also defines log's macros, we can't blindly import "slog::*" but always repeating
    // these imports is a pain. So just `use logs::macros::*` and you're all set.
    pub use log::{debug, error, info, warn};
    pub use slog::{
        slog_crit, slog_debug, slog_error, slog_info, slog_kv, slog_log, slog_o, slog_trace,
        slog_warn,
    };
}

// lazy_static::lazy_static! {
//   pub static ref LOGGER: Mutex<slog::Logger> = Mutex::new(SlogStdLogger::new());
// }

/// Setup the slog logging framework and the log->slog bridge for crates that use log.
///
/// The result contains the root logger and a slog-scope global logger guard for this root logger.
/// The global logger is unset once the guard is dropped.
///
pub fn setup_slog() -> (slog_scope::GlobalLoggerGuard, slog::Logger) {
    use slog::*;

    // let decorator = slog_term::TermDecorator::new().force_color().build();
    // let _term_drain = slog_term::FullFormat::new(decorator).build();

    let json_drain = slog_json::Json::default(std::io::stdout());

    // // Pick your format
    // //let drain = term_drain;
    let drain = json_drain;

    // // Display only info+
    let drain = drain.filter_level(Level::Info);

    // let drain = slog_async::Async::new(drain.fuse()).build().fuse();

    // let logger = slog::Logger::root(
    //     // slog_json::Json::default(std::io::stdout()),
    //     o!("version" => env!("CARGO_PKG_VERSION"))
    // );
    let logger = Logger::root(
        Mutex::new(drain.fuse()).map(Fuse),
        slog_o!(
            // "location" => FnValue(|info : &Record| {
            //     format!("{}:{}", info.module(), info.line())
            // })
        ),
    );

    // Bridge std log
    log::set_boxed_logger(Box::new(SlogStdLogger(logger.clone()))).unwrap();
    log::set_max_level(log::LevelFilter::max());

    // Set slog default logger
    let guard = slog_scope::set_global_logger(logger.clone());

    (guard, logger)
}

/// Bridge from log to slog. The slog-stdlog crate has not yet been updated to log 0.4
/// (see <https://github.com/slog-rs/stdlog/pull/5>)
///
struct SlogStdLogger(slog::Logger);

impl SlogStdLogger {
    // pub fn new() -> slog::Logger {
    //         use slog::*;

    // // let decorator = slog_term::TermDecorator::new().force_color().build();
    // // let _term_drain = slog_term::FullFormat::new(decorator).build();

    // let json_drain = slog_json::Json::default(std::io::stdout());

    // // // Pick your format
    // // //let drain = term_drain;
    // let drain = json_drain;

    // // // Display only info+
    // let drain = drain.filter_level(Level::Info);

    // // let drain = slog_async::Async::new(drain.fuse()).build().fuse();

    // // let logger = slog::Logger::root(
    // //     // slog_json::Json::default(std::io::stdout()),
    // //     o!("version" => env!("CARGO_PKG_VERSION"))
    // // );
    // let logger = Logger::root(
    //     Mutex::new(drain.fuse()).map(Fuse),
    //     slog_o!(
    //         // "location" => FnValue(|info : &Record| {
    //         //     format!("{}:{}", info.module(), info.line())
    //         // })
    //     ),
    // );

    // // Bridge std log
    // log::set_boxed_logger(Box::new(SlogStdLogger(logger.clone()))).unwrap();
    // log::set_max_level(log::LevelFilter::max());

    // // Set slog default logger
    // let _guard = slog_scope::set_global_logger(logger.clone());

    //     logger
    // }

    #[inline]
    pub fn log_to_slog_level(level: log::Level) -> slog::Level {
        match level {
            log::Level::Trace => slog::Level::Trace,
            log::Level::Debug => slog::Level::Debug,
            log::Level::Info => slog::Level::Info,
            log::Level::Warn => slog::Level::Warning,
            log::Level::Error => slog::Level::Error,
        }
    }
}

impl log::Log for SlogStdLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        use slog::Drain;
        self.0.is_enabled(Self::log_to_slog_level(metadata.level()))
    }

    fn log(&self, r: &log::Record) {
        // log provides Option<&'a str> while slog expects &'static str
        // We can expect log's strings to be static, but we can't safely decide to coerce them
        // into static strings, so we use an interning pool.
        // let as_static = |opt: Option<&str>| -> &'static str {
        //     use intern::Intern;
        //     match opt {
        //         None => "<unknown>",
        //         Some(s) => s.intern(),
        //     }
        // };

        let s = slog::RecordStatic {
            location: &slog::RecordLocation {
                file: "<unknown>", // Using 'module' is nicer, so save the interning time.
                line: r.line().unwrap_or(0),
                column: 0,
                function: "<unknown>",
                module: "<unknown>", //as_static(r.module_path()),
            },
            level: Self::log_to_slog_level(r.metadata().level()),
            tag: r.target(),
        };

        self.0
            .log(&slog::Record::new(&s, r.args(), slog::BorrowedKV(&())));
    }

    fn flush(&self) {}
}

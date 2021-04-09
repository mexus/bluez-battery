use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;

/// Sets up logging.
pub fn setup_logs(debug: usize) -> Result<(), log::SetLoggerError> {
    let log_level = match debug {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .debug(Color::Cyan)
        .trace(Color::BrightCyan)
        .warn(Color::Magenta)
        .error(Color::Red);
    // Warning and errors go to "stderr".
    let errors_dispatch = fern::Dispatch::new()
        .level(LevelFilter::Warn)
        .chain(fern::Output::stderr("\n"));
    // Info, debug and trace messages go to "stdout".
    let info_dispatch = fern::Dispatch::new()
        .level(log_level)
        .filter(|meta| {
            matches!(
                meta.level(),
                log::Level::Info | log::Level::Debug | log::Level::Trace
            )
        })
        .chain(fern::Output::stdout("\n"));
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}] {}",
                colors.color(record.level()),
                message
            ))
        })
        .level(log_level)
        .chain(info_dispatch)
        .chain(errors_dispatch)
        .apply()
}

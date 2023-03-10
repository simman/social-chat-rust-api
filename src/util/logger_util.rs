use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use std::path::PathBuf;

// {f} 文件
static CONSOLE_PATTERN: &str = "{d(%Y-%m-%d %H:%M:%S:%6f)} [{L}] [{P}] [{T}] {h({l})} - {m}{n}";
static ROLLING_FILE_PATTERN: &str = "{d(%Y-%m-%d %H:%M:%S:%6f)} [{f}:{L}] [{P}] [{T}] {l} - {m}{n}";

pub const HTTP_LOG: &str = "http";
pub const SOCKET_LOG: &str = "socket";
pub const COMMON_LOG: &str = "common";

fn get_rolling_file_appender(base_path: &str, name: &str) -> RollingFileAppender {
    let trigger_size = 1024 * 1024 * 1; // 1m
    let trigger = Box::new(SizeTrigger::new(trigger_size));
    let roller_pattern = PathBuf::new()
        .join(base_path)
        .join("logs")
        .join(name.to_owned() + "_{}.gz");
    let roller_pattern = roller_pattern.to_str().unwrap();
    let roller_count = 10;
    let roller_base = 1;
    let roller = Box::new(
        FixedWindowRoller::builder()
            .base(roller_base)
            .build(roller_pattern, roller_count)
            .unwrap(),
    );

    let compound_policy = Box::new(CompoundPolicy::new(trigger, roller));

    let roller_pattern = PathBuf::new()
        .join(base_path)
        .join("logs")
        .join(name.to_owned() + ".log");
    let roller_pattern = roller_pattern.to_str().unwrap();

    RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(ROLLING_FILE_PATTERN)))
        .build(roller_pattern, compound_policy)
        .unwrap()
}

pub(crate) fn init_log(base_path: &str) {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(CONSOLE_PATTERN)))
        .build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(
            Appender::builder().build("log", Box::new(get_rolling_file_appender(base_path, "log"))),
        )
        .appender(Appender::builder().build(
            "common",
            Box::new(get_rolling_file_appender(base_path, COMMON_LOG)),
        ))
        .appender(Appender::builder().build(
            "http",
            Box::new(get_rolling_file_appender(base_path, HTTP_LOG)),
        ))
        .appender(Appender::builder().build(
            "socket",
            Box::new(get_rolling_file_appender(base_path, SOCKET_LOG)),
        ))
        .loggers(vec![
            Logger::builder()
                .appender(COMMON_LOG)
                .build(COMMON_LOG, LevelFilter::Debug),
            Logger::builder()
                .appender(HTTP_LOG)
                .build(HTTP_LOG, LevelFilter::Debug),
            Logger::builder()
                .appender(SOCKET_LOG)
                .build(SOCKET_LOG, LevelFilter::Debug),
        ])
        .build(
            Root::builder()
                .appender("stdout")
                .appender("log")
                .build(LevelFilter::Debug),
        )
        .unwrap();

    // You can use handle to change logger config at runtime
    log4rs::init_config(config).unwrap();
}

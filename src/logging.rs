use log::LevelFilter;

use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;
use log4rs::encode::pattern::PatternEncoder;

/// log4rs config generator
pub fn get_log_config() -> log4rs::Config {
    // Returns log4rs configuration
    
    let log_line_pattern = "{h({d(%Y-%m-%d %H:%M:%S)(utc)} - {l}: {m}{n})}";
    
    let stdout = ConsoleAppender::builder().encoder(Box::new(PatternEncoder::new(log_line_pattern))).build();
    
    let config = Config::builder()
    .appender(Appender::builder().build("stdout", Box::new(stdout)))
    .build(Root::builder().appender("stdout").build(LevelFilter::Info))
    .unwrap();
    return config;
}
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};

pub fn run() {
    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .expect("Failed to initialize logger.");
}
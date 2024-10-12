use flexi_logger::{FileSpec, Logger, LoggerHandle};

pub fn init() -> Option<LoggerHandle> {
    println!("creating logger");
    Logger::try_with_env()
        .ok()?
        .log_to_file(FileSpec::default().directory("logs"))
        .start()
        .ok()
}

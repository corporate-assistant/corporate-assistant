use std::fmt;

// Let's extend Result with logging
pub trait ResultExt<T, E> {
    fn expect_and_log(self, msg: &str) -> T;
}

impl<T, E: fmt::Debug> ResultExt<T, E> for Result<T, E> {
    fn expect_and_log(self, err_msg: &str) -> T {
        self.map_err(|e| {
            log::error!("{}", err_msg);
            e
        })
        .expect(err_msg)
    }
}

pub fn init_logging_infrastructure() {
    // TODO(jczaja): test on windows/macos
    syslog::init(
        syslog::Facility::LOG_USER,
        log::LevelFilter::Debug,
        Some("corporate-assistant"),
    )
    .expect("Error initializing syslog");
}

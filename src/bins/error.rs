#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Could not load config file.")]
    ConfigLoad {
        #[from]
        source: std::io::Error,
    },

    #[error("Could not read json in file.")]
    InvalidJson,

    #[error("SystemTime before UNIX EPOCH!")]
    InvalidSystemTime,

    #[error("Could not create a duration from given time.")]
    InvalidDuration,

    #[error("Could not extract a time from date string.")]
    InvalidDateString,
}

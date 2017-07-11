#[derive(Debug)]
pub enum LogLevel {
    Quiet,
    Error,
    Info,
    Debug,
}

impl LogLevel {
    pub fn to_int(&self) -> i32 {
        match *self {
            LogLevel::Quiet => -1,
            LogLevel::Error => 0,
            LogLevel::Info => 1,
            LogLevel::Debug => 2,
        }
    }

    pub fn from_int(int: i32) -> Self {
        match int {
            -1 => LogLevel::Quiet,
            0 => LogLevel::Error,
            1 => LogLevel::Info,
            2 => LogLevel::Debug,
            _ => LogLevel::Debug,
        }
    }
}

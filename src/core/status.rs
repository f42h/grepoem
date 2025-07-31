use std::fmt;

use colored::Colorize;

pub enum StatusMark {
    Info,
    Error,
    Success
}

impl fmt::Display for StatusMark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StatusMark::Info => write!(f, "[{}]", "*".blue()),
            StatusMark::Error => write!(f, "[{}]", "-".blue()),
            StatusMark::Success => write!(f, "[{}]", "+".green())
        }
    }
}
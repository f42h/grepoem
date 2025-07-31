use std::{fmt, process};

#[derive(Debug)]
pub enum GrepOemError {
    OemKeyNotFound,
    AcpiTableInaccessible(String),
    TableParsingError(String),
    SaveOemKeyError(String)

}

impl fmt::Display for GrepOemError {
    fn fmt(&self, form: &mut fmt::Formatter) -> fmt::Result {
       match self {
           GrepOemError::OemKeyNotFound => write!(form, "OEM key not found in MSDM"),
           GrepOemError::AcpiTableInaccessible(path) => write!(form, "Unable to access ACPI table`{}`", path),
           GrepOemError::TableParsingError(reason) => write!(form, "Failed to parse MSDM content: {}", reason),
           GrepOemError::SaveOemKeyError(reason) => write!(form, "Failed to save OEM key to file: {}", reason)
       }
    }
}

pub struct ExitStderr;
impl ExitStderr {
    pub fn quit(msg: String, code: i32) {
        eprintln!("{}", msg);
        process::exit(code)
    }
}
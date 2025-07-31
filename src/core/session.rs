use std::{
    fs::{File, OpenOptions}, 
    io::{BufRead, BufReader, Write}
};

use regex::Regex;

use crate::core::{err::GrepOemError, key::WinOemKey, status::StatusMark};

pub struct GrepOem {
    acpi_table: String
}

impl GrepOem {
    pub fn new() -> Self {
        Self { acpi_table: String::from("/sys/firmware/acpi/tables/MSDM") }
    }

    fn extract_oem(&self, hay: &str) -> Option<WinOemKey> {
        println!("{} Extracting key..", StatusMark::Info);

        let re = Regex::new(r"([A-Z0-9]{5}-[A-Z0-9]{5}-[A-Z0-9]{5}-[A-Z0-9]{5}-[A-Z0-9]{5})").unwrap();
        
        if let Some(key) = re.captures(hay) {
            return Some(WinOemKey::from(&key[0]));
        }
        
        None
    }

    pub fn open_msdm(&self) -> Result<WinOemKey, GrepOemError> {
        let stream = File::open(&self.acpi_table)
            .map_err(|_| GrepOemError::AcpiTableInaccessible(self.acpi_table.clone()))?;
        
        let reader = BufReader::new(stream);
    
        for line in reader.lines() {
            let line = line.map_err(|err| GrepOemError::TableParsingError(err.to_string()))?;
            
            if let Some(key) = self.extract_oem(&line) {
                if WinOemKey::validate(key.clone()) {
                    return Ok(key);
                }
            }
        }
    
        Err(GrepOemError::OemKeyNotFound)
    }
    

    pub fn save_key(&self, key: WinOemKey, output_file: &str) -> Result<(), GrepOemError> {
        let mut stream = OpenOptions::new()
            .append(true)
            .create(true)
            .open(output_file)
            .map_err(|err| GrepOemError::SaveOemKeyError(err.to_string()))?;
    
        writeln!(stream, "{}", key.value).map_err(|err| {
            eprintln!("{} Error: {}", StatusMark::Error, err);
            GrepOemError::SaveOemKeyError(err.to_string())
        })?;
    
        println!("{} Output saved to `{}`", StatusMark::Info, output_file);

        Ok(())
    }    
}
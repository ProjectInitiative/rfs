use std::fmt;
use std::error::Error;




#[derive(Debug)]
pub struct FilerParseError {
    pub details: String
}

impl FilerParseError {
    pub fn new(msg: &str) -> FilerParseError {
        FilerParseError{details: msg.to_string()}
    }
}

impl fmt::Display for FilerParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for FilerParseError {
    fn description(&self) -> &str {
        &self.details
    }
}


#[derive(Debug)]
pub struct FilerRequestError {
    pub details: String
}

impl FilerRequestError {
    pub fn new(msg: &str) -> FilerRequestError {
        FilerRequestError{details: msg.to_string()}
    }
}

impl fmt::Display for FilerRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for FilerRequestError {
    fn description(&self) -> &str {
        &self.details
    }
}
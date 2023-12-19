use std::collections::HashMap;
use logger::Logger;
use result::Result;
pub struct Process {
    log: Logger,
    result: Result,
}


impl Process {
    pub fn detect(&self, input_text: &str) -> Result<Vec<DetectResult>, &'static str> {
    }

    pub fn mask(&self, input_text: &str, method_name: &str) -> Result<String, &'static str> {
    }

    pub fn verify(&self, input_text: &str) -> Result<bool, &'static str> {
    }

    pub fn close(&self) {
    } 


}




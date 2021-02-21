use std::fmt;
use serde::{Serialize,Deserialize};

#[derive(Debug, PartialEq, Clone)]
#[derive(Serialize, Deserialize)]
pub struct ValidationError {
    #[serde(default)]
    field_name: String,
    #[serde(default)]
    inputted_value: String,
}

impl ValidationError {
    pub(crate) fn new(field_name :String, inputted_value:String) -> ValidationError {
        ValidationError {
            field_name,
            inputted_value
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("Value \"{}\" is not valid for field \"{}\"", self.inputted_value, self.field_name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_and_format() {
        let exp = "Value \"invalid_value\" is not valid for field \"invalid_field\"";

        let act = format!("{}", ValidationError::new(String::from("invalid_field"), String::from("invalid_value")));

        assert_eq!(act, exp);
    }
}


use failure::format_err;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::time::{SystemTime, UNIX_EPOCH};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Environment {
    pub id: String,
    pub name: String,
    pub values: Vec<Value>,
    pub timestamp: u128,
}

impl Environment {
    pub fn empty() -> Self{
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        Self{
            id: "".to_owned(), //some uuid?
            name: "".to_owned(),
            values: vec![],
            timestamp: since_the_epoch.as_millis()
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Value {
    pub enabled: bool,
    pub key: String,
    pub value: String,
    pub r#type: Type,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Type {
    Text,
}

pub fn read_from_file(file_name: &str) -> Result<Environment, Box<dyn Error>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let environment = serde_json::de::from_reader(reader)?;
    Ok(environment)
}

impl Environment {
    fn get(&self, key: &str) -> Result<String, Box<dyn Error>> {
        for v in self.values.iter() {
            if v.key == key {
                return Ok(v.value.clone());
            }
        }
        Err(format_err!("not found {}", key).into())
    }

    pub fn resolve(&self, text: &str) -> Result<String, Box<dyn Error>> {
        let mut cb1 = false;
        let mut cb2 = false;
        let mut key = "".to_owned();
        let mut resolved = "".to_owned();
        for g in text.graphemes(true) {
            if g == "{" {
                if !cb1 {
                    cb1 = true;
                } else {
                    if !cb2 {
                        cb2 = true;
                    } else {
                        return Err(format_err!("triple {{ not allowed").into());
                    }
                }
            } else if g == "}" {
                if cb2 {
                    cb2 = false;
                } else if cb1 {
                    cb1 = false;
                    let value = self.get(&key)?;
                    resolved.push_str(&value);
                } else {
                    return Err(format_err!("triple }} not allowed").into());
                }
            } else {
                if cb2 {
                    key.push_str(g);
                } else if !cb1 {
                    resolved.push_str(g);
                }
            }
        }
        Ok(resolved.to_owned())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn read_env() {
        let env = read_from_file("tests/json/environment.json").expect("");
        assert_eq!(12, env.values.len());
    }
}

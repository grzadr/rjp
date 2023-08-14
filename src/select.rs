// src/select.rs

pub mod token;

use core::fmt;
use itertools::join;
use serde_json::Value;
use token::{parse_tokens, Token};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Select {
    fields: Vec<Token>,
}

impl Select {
    pub fn new(s: &str) -> Self {
        Select {
            fields: parse_tokens(s)
                .map_err(|e| format!("Error creating Select - {e}"))
                .unwrap(),
        }
    }

    pub fn collect(&self) -> Vec<String> {
        self.fields
            .iter()
            .map(|t| t.to_string())
            .collect()
    }

    pub fn name(&self) -> Option<String> {
        match self.fields.last() {
            Some(t) => Some(t.to_string()),
            _ => None,
        }
    }
}

impl From<&str> for Select {
    fn from(s: &str) -> Self {
        Select::new(s)
    }
}

impl std::str::FromStr for Select {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Select::new(s))
    }
}

impl fmt::Display for Select {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            join(self.fields.iter().map(|t| t.to_string()), ".")
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selects(pub Vec<Select>);

impl Selects {
    pub fn new(v: Vec<Select>) -> Self {
        Selects(v)
    }
}

impl fmt::Display for Selects {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for select in &self.0 {
            s.push_str(&format!("{} ", select));
        }
        write!(f, "{}", s.trim_end())
    }
}

#[derive(Debug)]
pub struct SelectedValue {
    pub value: Value,
    pub path: Select,
}

impl SelectedValue {
    pub fn new(value: Value, path: Select) -> Self {
        let mut value = value;

        for field in path.collect() {
            if field == "." {
                continue;
            }
            value = match value {
                Value::Object(obj) => {
                    if let Some(v) = obj.get(&field) {
                        v.clone()
                    } else {
                        Value::Null
                    }
                }
                _ => break,
            }
        }
        Self { value, path }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_new() {
        assert_eq!(
            Select::new("{}"),
            Select {
                fields: vec![Token::Object]
            }
        );
        assert_eq!(
            Select::new("{}.foo"),
            Select {
                fields: vec![Token::Object, "foo".try_into().unwrap()]
            }
        );
        assert_eq!(
            Select::new("foo.bar"),
            Select {
                fields: vec!["foo".try_into().unwrap(), "bar".try_into().unwrap()]
            }
        );
        assert_eq!(
            Select::new("foo.\"bar.baz\""),
            Select {
                fields: vec!["foo".try_into().unwrap(), "bar.baz".try_into().unwrap()]
            }
        );
        assert_eq!(
            Select::new("foo.\"bar.baz\".qux"),
            Select {
                fields: vec![
                    "foo".try_into().unwrap(),
                    "bar.baz".try_into().unwrap(),
                    "qux".try_into().unwrap()
                ]
            }
        );
    }

    #[test]
    fn test_select_collect() {
        assert_eq!(Select::new("{}").collect(), vec!["{}"]);
        assert_eq!(Select::new("{}.foo").collect(), vec!["{}", "foo"]);
        assert_eq!(
            Select::new("{}.foo.bar").collect(),
            vec!["{}", "foo", "bar"]
        );
        assert_eq!(
            Select::new("{}.foo.\"bar.baz\"").collect(),
            vec!["{}", "foo", "bar.baz"]
        );
        assert_eq!(
            Select::new("{}.foo.\"bar.baz\".qux").collect(),
            vec!["{}", "foo", "bar.baz", "qux"]
        );
    }
}

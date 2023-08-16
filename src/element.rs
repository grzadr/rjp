// src/element.rs

pub mod token;

use core::fmt;
use itertools::join;
use serde_json::Value;
use token::{parse_tokens, Token};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    tokens: Vec<Token>,
}

impl Element {
    pub fn new(s: &str) -> Self {
        Element {
            tokens: parse_tokens(s)
                .map_err(|e| format!("Error creating Select - {e}"))
                .unwrap(),
        }
    }

    pub fn collect(&self) -> Vec<String> {
        self.tokens
            .iter()
            .map(|t| t.to_string())
            .collect()
    }

    pub fn name(&self) -> Option<String> {
        match self.tokens.last() {
            Some(t) => Some(t.to_string()),
            _ => None,
        }
    }
}

impl From<&str> for Element {
    fn from(s: &str) -> Self {
        Element::new(s)
    }
}

impl std::str::FromStr for Element {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Element::new(s))
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            join(self.tokens.iter().map(|t| t.to_string()), ".")
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selects(pub Vec<Element>);

impl Selects {
    pub fn new(v: Vec<Element>) -> Self {
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
    pub filename: String,
    pub value: Value,
    pub path: Element,
}

impl SelectedValue {
    pub fn new(filename: String, value: Value, path: Element) -> Self {
        let mut value = value;

        for token in path.tokens {
            match token {
                Token::Any => continue,
                Token::Field { name, with_name }
            if token == "." {
                continue;
            }
            value = match value {
                Value::Object(obj) => {
                    if let Some(v) = obj.get(&field) {
                        v
                    } else {
                        break
                    }
                },
                _ => break,
            }
        }
        Self { filename, value, path }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_new() {
        assert_eq!(
            Element::new("{}"),
            Element {
                tokens: vec![Token::Object]
            }
        );
        assert_eq!(
            Element::new("{}.foo"),
            Element {
                tokens: vec![Token::Object, "foo".try_into().unwrap()]
            }
        );
        assert_eq!(
            Element::new("foo.bar"),
            Element {
                tokens: vec!["foo".try_into().unwrap(), "bar".try_into().unwrap()]
            }
        );
        assert_eq!(
            Element::new("foo.\"bar.baz\""),
            Element {
                tokens: vec!["foo".try_into().unwrap(), "bar.baz".try_into().unwrap()]
            }
        );
        assert_eq!(
            Element::new("foo.\"bar.baz\".qux"),
            Element {
                tokens: vec![
                    "foo".try_into().unwrap(),
                    "bar.baz".try_into().unwrap(),
                    "qux".try_into().unwrap()
                ]
            }
        );
    }

    #[test]
    fn test_select_collect() {
        assert_eq!(Element::new("{}").collect(), vec!["{}"]);
        assert_eq!(Element::new("{}.foo").collect(), vec!["{}", "foo"]);
        assert_eq!(
            Element::new("{}.foo.bar").collect(),
            vec!["{}", "foo", "bar"]
        );
        assert_eq!(
            Element::new("{}.foo.\"bar.baz\"").collect(),
            vec!["{}", "foo", "bar.baz"]
        );
        assert_eq!(
            Element::new("{}.foo.\"bar.baz\".qux").collect(),
            vec!["{}", "foo", "bar.baz", "qux"]
        );
    }
}

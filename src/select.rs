// src/select.rs

pub mod token;

use core::fmt;
use regex::Regex;
use serde_json::Value;

fn parse_path(s: &str) -> Vec<String> {
    let mut result = vec![".".to_string()];
    if s == "." {
        return result;
    }

    let re = Regex::new(r#"\.(?:(?:\"([^\"]+)\")|([^.]+))"#).unwrap();
    result.append(&mut re.captures_iter(s).map(|cap| {
        if let Some(s) = cap.get(1) {
            s.as_str().to_string()
        } else if let Some(s) = cap.get(2) {
            s.as_str().to_string()
        } else {
            panic!("Couldn't parse '{}'", s);
        }
    }).collect());

    result
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Select {
    path: String,
    fields: Vec<String>,
}

impl Select {
    pub fn new(s: &str) -> Self {
        Select {
            path: s.to_string(),
            fields: parse_path(s),
        }
    }

    pub fn collect(&self) -> Vec<&str> {
        self.fields.iter().map(|s| s.as_str()).collect()
    }

    pub fn name(&self) -> &str {
        self.fields.last().unwrap()
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
        write!(f, "{}", self.path)
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
                Value::Object(obj) => if let Some(v) = obj.get(field) {v.clone()} else {Value::Null},
                _ => break
            }
        }
        Self { value, path }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::token::*;

    #[test]
    fn test_parse_tokens() {
        assert_eq!(parse_tokens("").unwrap(), Vec::new());
        assert_eq!(parse_tokens("*").unwrap(), vec![Token::Any]);
        assert_eq!(parse_tokens("*.*").unwrap(), vec![Token::Any, Token::Any]);
        assert_eq!(parse_tokens("{}").unwrap(), vec![Token::Object]);
        assert_eq!(parse_tokens("[]").unwrap(), vec![Token::Array]);
        assert_eq!(parse_tokens("name").unwrap(), vec![Token::Field{name: "name".to_string(), with_name: false}]);
        assert_eq!(parse_tokens("=name").unwrap(), vec![Token::Field{name: "name".to_string(), with_name: true}]);
        assert_eq!(parse_tokens("{}.=name").unwrap(), vec![Token::Object, Token::Field{name: "name".to_string(), with_name: true}]);
        assert_eq!(parse_tokens("{}.name").unwrap(), vec![Token::Object, Token::Field{name: "name".to_string(), with_name: false}]);
        assert_eq!(parse_tokens("{}.=\"name\"").unwrap(), vec![Token::Object, Token::Field{name: "name".to_string(), with_name: true}]);
        assert_eq!(parse_tokens("{}.\"na.me\"").unwrap(), vec![Token::Object, Token::Field{name: "na.me".to_string(), with_name: false}]);
        assert_eq!(parse_tokens("{}.=\"na.me.me\"").unwrap(), vec![Token::Object, Token::Field{name: "na.me.me".to_string(), with_name: true}]);
        assert_eq!(parse_tokens("{}.\"na.me\".[]").unwrap(), vec![Token::Object, Token::Field{name: "na.me".to_string(), with_name: false}, Token::Array]);
    }

    #[test]
    fn test_parse_path() {
        assert_eq!(parse_path("."), vec![".".to_string()]);
        assert_eq!(parse_path(".foo"), vec![".".to_string(), "foo".to_string()]);
        assert_eq!(parse_path(".foo.bar"), vec![".".to_string(), "foo".to_string(), "bar".to_string()]);
        assert_eq!(parse_path(".foo.\"bar.baz\""), vec![".".to_string(), "foo".to_string(), "bar.baz".to_string()]);
        assert_eq!(parse_path(".foo.\"bar.baz\".qux"), vec![".".to_string(), "foo".to_string(), "bar.baz".to_string(), "qux".to_string()]);
    }

    #[test]
    fn test_parse_tokens_err() {
        assert!(matches!(parse_tokens(".."), Err(_)));
    }

    #[test]
    fn test_select_new() {
        assert_eq!(Select::new("."), Select { path: ".".to_string(), fields: vec![".".to_string()] });
        assert_eq!(Select::new(".foo"), Select { path: ".foo".to_string(), fields: vec![".".to_string(), "foo".to_string()] });
        assert_eq!(Select::new(".foo.bar"), Select { path: ".foo.bar".to_string(), fields: vec![".".to_string(), "foo".to_string(), "bar".to_string()] });
        assert_eq!(Select::new(".foo.\"bar.baz\""), Select { path: ".foo.\"bar.baz\"".to_string(), fields: vec![".".to_string(), "foo".to_string(), "bar.baz".to_string()] });
        assert_eq!(Select::new(".foo.\"bar.baz\".qux"), Select { path: ".foo.\"bar.baz\".qux".to_string(), fields: vec![".".to_string(), "foo".to_string(), "bar.baz".to_string(), "qux".to_string()] });
    }

    #[test]
    fn test_select_collect() {
        assert_eq!(Select::new(".").collect(), vec!["."]);
        assert_eq!(Select::new(".foo").collect(), vec![".", "foo"]);
        assert_eq!(Select::new(".foo.bar").collect(), vec![".", "foo", "bar"]);
        assert_eq!(Select::new(".foo.\"bar.baz\"").collect(), vec![".", "foo", "bar.baz"]);
        assert_eq!(Select::new(".foo.\"bar.baz\".qux").collect(), vec![".", "foo", "bar.baz", "qux"]);
    }
}
// src/select/token.rs

use core::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenParsingError {
    msg: String,
}

impl fmt::Display for TokenParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Field { name: String, with_name: bool },
    Object,
    Array,
    Any,
}

impl std::str::FromStr for Token {
    type Err = TokenParsingError;

    fn from_str(s: &str) -> Result<Self, TokenParsingError> {
        match s {
            "{}" => Ok(Self::Object),
            "[]" => Ok(Self::Array),
            "*" => Ok(Self::Any),
            "" => Err(TokenParsingError {
                msg: "Cannot convert empty string".to_string(),
            }),
            s => {
                let with_name = s.starts_with('=');
                let name = if with_name { &s[1..] } else { s };

                let correct = if name.starts_with('"') || name.ends_with('"') {
                    name.starts_with('"') && name.ends_with('"')
                } else {
                    true
                };

                if !correct {
                    return Err(TokenParsingError {
                        msg: format!("Token {s} contains unbalanced '\"' )"),
                    });
                }

                Ok(Self::Field {
                    name: name.trim_matches('"').to_string(),
                    with_name,
                })
            }
        }
    }
}

impl From<&str> for Token {
    fn from(s: &str) -> Self {
        Token::from_str(s).unwrap()
    }
}

pub fn parse_tokens(s: &str) -> Result<Vec<Token>, TokenParsingError> {
    dbg!("Parsing", s);
    let mut result = Vec::new();

    if s.is_empty() {
        return Ok(result);
    }

    let mut previous = String::new();
    for ele in s.split('.') {
        dbg!(&ele);
        previous += ele;
        dbg!(&previous);

        if (previous.starts_with("=\"") || previous.starts_with("\"")) && !previous.ends_with("\"") {
            dbg!("Add .");
            previous += ".";
            continue;
        }

        dbg!("Push");

        result.push(Token::from(&previous[..]));
        previous.clear();
    }

    dbg!("Parsed");
    dbg!();
    Ok(result)
}

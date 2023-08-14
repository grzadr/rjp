// src/select/token.rs

use core::fmt;
use log::debug;
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

impl std::str::FromStr for TokenParsingError {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        Ok(Self { msg: s.to_string() })
    }
}

impl From<&str> for TokenParsingError {
    fn from(s: &str) -> Self {
        TokenParsingError::from_str(s).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Any,
    Array,
    Object,
    Field { name: String, with_name: bool },
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Any => write!(f, "*"),
            Self::Array => write!(f, "[]"),
            Self::Object => write!(f, "{{}}"),
            Self::Field { name, with_name } => {
                write!(f, "{}{name}", if *with_name { "=" } else { "" })
            }
        }
    }
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

impl TryFrom<&str> for Token {
    type Error = TokenParsingError;

    fn try_from(s: &str) -> Result<Self, TokenParsingError> {
        Token::from_str(s)
    }
}

pub fn parse_tokens(s: &str) -> Result<Vec<Token>, TokenParsingError> {
    debug!("Parsing '{}'", s);
    let mut result = Vec::new();

    if s.is_empty() {
        return Ok(result);
    }

    let mut previous = String::new();
    for ele in s.split('.') {
        dbg!(&ele);
        previous += ele;
        dbg!(&previous);

        if (previous.starts_with("=\"") || previous.starts_with("\"")) && !previous.ends_with("\"")
        {
            dbg!("Add .");
            previous += ".";
            continue;
        }

        dbg!("Push");

        result.push(match Token::try_from(&previous[..]) {
            Ok(k) => k,
            Err(e) => {
                return Err(TokenParsingError {
                    msg: format!("Error parsing '{s}' - {e}"),
                })
            }
        });
        previous.clear();
    }

    dbg!("Parsed");
    dbg!();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokens_repr() {
        assert_eq!(Token::from_str("{}").unwrap(), Token::Object);
        assert_eq!(Token::from_str("[]").unwrap(), Token::Array);
        assert_eq!(Token::from_str("*").unwrap(), Token::Any);
        assert_eq!(
            Token::from_str("name").unwrap(),
            Token::Field {
                name: String::from("name"),
                with_name: false
            }
        );
        assert_eq!(
            Token::from_str("=name").unwrap(),
            Token::Field {
                name: String::from("name"),
                with_name: true
            }
        );
    }

    #[test]
    fn test_tokens_parse() {
        assert_eq!(parse_tokens("").unwrap(), Vec::new());
        assert_eq!(parse_tokens("*").unwrap(), vec![Token::Any]);
        assert_eq!(parse_tokens("*.*").unwrap(), vec![Token::Any, Token::Any]);
        assert_eq!(parse_tokens("{}").unwrap(), vec![Token::Object]);
        assert_eq!(parse_tokens("[]").unwrap(), vec![Token::Array]);
        assert_eq!(
            parse_tokens("name").unwrap(),
            vec![Token::Field {
                name: "name".to_string(),
                with_name: false
            }]
        );
        assert_eq!(
            parse_tokens("=name").unwrap(),
            vec![Token::Field {
                name: "name".to_string(),
                with_name: true
            }]
        );
        assert_eq!(
            parse_tokens("{}.=name").unwrap(),
            vec![
                Token::Object,
                Token::Field {
                    name: "name".to_string(),
                    with_name: true
                }
            ]
        );
        assert_eq!(
            parse_tokens("{}.name").unwrap(),
            vec![
                Token::Object,
                Token::Field {
                    name: "name".to_string(),
                    with_name: false
                }
            ]
        );
        assert_eq!(
            parse_tokens("{}.=\"name\"").unwrap(),
            vec![
                Token::Object,
                Token::Field {
                    name: "name".to_string(),
                    with_name: true
                }
            ]
        );
        assert_eq!(
            parse_tokens("{}.\"na.me\"").unwrap(),
            vec![
                Token::Object,
                Token::Field {
                    name: "na.me".to_string(),
                    with_name: false
                }
            ]
        );
        assert_eq!(
            parse_tokens("{}.=\"na.me.me\"").unwrap(),
            vec![
                Token::Object,
                Token::Field {
                    name: "na.me.me".to_string(),
                    with_name: true
                }
            ]
        );
        assert_eq!(
            parse_tokens("{}.\"na.me\".[]").unwrap(),
            vec![
                Token::Object,
                Token::Field {
                    name: "na.me".to_string(),
                    with_name: false
                },
                Token::Array
            ]
        );
    }

    #[test]
    fn test_tokens_parse_err() {
        assert_eq!(
            parse_tokens("..").expect_err(""),
            TokenParsingError::from("Error parsing '..' - Cannot convert empty string")
        );
    }
}

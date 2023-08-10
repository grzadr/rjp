// src/select.rs

use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Select {
    path: String
}

impl Select {
    pub fn new(s: &str) -> Self {
        Select {
            path: s.to_string()
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

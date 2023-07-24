use std::{
    error::Error,
    fmt::Display,
    num::ParseIntError,
    str::{FromStr, ParseBoolError},
};

use chrono::*;

const ITEM_COUNT: usize = 7;

#[derive(Debug)]
pub struct Item {
    id: u32,
    name: String,
    completed: bool,
    deleted: bool,
    created_at: Option<i64>,
    completed_at: Option<i64>,
    deleted_at: Option<i64>,
}

impl Item {
    pub fn new(
        id: u32,
        name: &str,
        completed: bool,
        deleted: bool,
        created_at: Option<i64>,
        completed_at: Option<i64>,
        deleted_at: Option<i64>,
    ) -> Self {
        Item {
            id,
            name: name.to_string(),
            completed,
            deleted,
            created_at,
            completed_at,
            deleted_at,
        }
    }

    pub fn show(&self) -> String {
        format!(
            "{:3} {} {} {:?}",
            self.id,
            if self.completed { "ok" } else { "todo" },
            self.name,
            NaiveDateTime::from_timestamp_opt(self.created_at.unwrap(), 0).unwrap(),
        )
    }
}

impl ToString for Item {
    fn to_string(&self) -> String {
        let mut created_at = String::new();
        if let Some(x) = self.created_at {
            created_at = x.to_string();
        }

        let mut completed_at = String::new();
        if let Some(x) = self.completed_at {
            completed_at = x.to_string();
        }

        let mut deleted_at = String::new();
        if let Some(x) = self.deleted_at {
            deleted_at = x.to_string();
        }

        format!(
            "{},{},{},{},{},{},{}",
            self.id, self.name, self.completed, self.deleted, created_at, completed_at, deleted_at,
        )
    }
}

impl FromStr for Item {
    type Err = ItemError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splited = s.split(',').collect::<Vec<_>>();
        if splited.len() < ITEM_COUNT {
            return Err(ItemError::ParseErr("item lack".to_string()));
        }
        let id = splited[0].parse::<u32>()?;
        let name = splited[1];
        let completed = splited[2].parse::<bool>()?;
        let deleted = splited[3].parse::<bool>()?;

        let created_at_str = splited[4];
        let created_at = if created_at_str.is_empty() {
            None
        } else {
            Some(created_at_str.parse::<i64>()?)
        };

        let completed_at_str = splited[5];
        let completed_at = if completed_at_str.is_empty() {
            None
        } else {
            Some(completed_at_str.parse::<i64>()?)
        };

        let deleted_at_str = splited[6];
        let deleted_at = if deleted_at_str.is_empty() {
            None
        } else {
            Some(deleted_at_str.parse::<i64>()?)
        };

        Ok(Item::new(
            id,
            name,
            completed,
            deleted,
            created_at,
            completed_at,
            deleted_at,
        ))
    }
}

#[derive(Debug)]
pub enum ItemError {
    ParseErr(String),
}

impl Error for ItemError {}

impl Display for ItemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ItemError::*;
        match self {
            ParseErr(s) => write!(f, "{}", s),
        }
    }
}

impl From<ParseIntError> for ItemError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseErr(value.to_string())
    }
}

impl From<ParseBoolError> for ItemError {
    fn from(value: ParseBoolError) -> Self {
        Self::ParseErr(value.to_string())
    }
}

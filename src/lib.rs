use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Flairs represents flairs your users can utilize
#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct Flair {
    pub id: usize,
    pub name: String,
    pub assigned_on: DateTime<Utc>,
    pub flair: usize,
}

impl Flair {
    pub fn new(id: usize, name: String, assigned_on: DateTime<Utc>, flair: usize) -> Self {
        Self {
            id,
            name,
            assigned_on,
            flair,
        }
    }
}

/// FlairDirectory represents Lemmy instance users. It's designed to be flexable, and exist whether you use
/// Flairs with your Lemmy's Postgres database or Sqlite, which is why the name's so ambiguous.
#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct FlairDirectory {
    pub id: Option<usize>,
    pub special: bool,
    pub ref_id: String,
    pub pos: usize,
    pub flair: usize,
    pub path: Option<String>,
}

impl FlairDirectory {
    pub fn new(
        id: Option<usize>,
        special: bool,
        ref_id: String,
        pos: usize,
        flair: usize,
        path: Option<String>,
    ) -> Self {
        Self {
            id,
            special,
            ref_id,
            pos,
            flair,
            path,
        }
    }
}

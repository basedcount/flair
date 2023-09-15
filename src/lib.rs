use serde::{Deserialize, Serialize};
use ts_rs::TS;
use chrono::{DateTime, Utc}; 

/// Flairs represents flairs your users can utilize
#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct Flair {
    pub id: Option<usize>,
    /// Flair internal name (used for config purposes, eg: mod view)
    pub name: String,
    /// Flair displayed name (visible on the website)
    pub display_name: String,
    /// Flair image path/url if present
    pub path: Option<String>,
    pub assigned_on: DateTime<Utc>,
    /// Community where the flair exists
    pub community_actor_id: String,
    pub mod_only: bool,
}

impl Flair {
    pub fn new(
        id: Option<usize>,
        name: String,
        display_name: String,
        path: Option<String>,
        assigned_on: DateTime<Utc>,
        community_actor_id: String,
        mod_only: bool,
    ) -> Self {
        Self {
            id,
            name,
            display_name,
            path,
            assigned_on,
            community_actor_id,
            mod_only,
        }
    }
}

/// FlairDirectory represents Lemmy instance users. It's designed to be flexable, and exist whether you use
/// Flairs with your Lemmy's Postgres database or Sqlite, which is why the name's so ambiguous.
#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct UserFlair {
    pub id: Option<usize>,
    pub user_actor_id: usize,
    pub flair_id: usize,
    pub assigned_on: DateTime<Utc>,  // This represents a non-timezone-aware datetime
}

impl UserFlair {
    pub fn new(id: Option<usize>, user_actor_id: usize, flair_id: usize, assigned_on: DateTime<Utc>) -> Self { Self { id, user_actor_id, flair_id, assigned_on } }
}
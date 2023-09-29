use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Flairs represents flairs your users can utilize
#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct Flair {
    /// Flair internal name (used for config purposes, eg: mod view)
    pub name: String,
    /// Flair displayed name (visible on the website)
    pub display_name: String,
    /// Flair image path/url if present
    pub path: Option<String>,
    /// Community where the flair exists
    pub community_actor_id: String,
    pub mod_only: bool,
}

impl Flair {
    pub fn new(
        name: String,
        display_name: String,
        path: Option<String>,
        community_actor_id: String,
        mod_only: bool,
    ) -> Self {
        Self {
            name,
            display_name,
            path,
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
    pub user_actor_id: String,
    pub flair_name: String,
    pub flair_community_actor_id: String,
    pub assigned_on: DateTime<Utc>, // This represents a non-timezone-aware datetime
}

impl UserFlair {
    pub fn new(
        user_actor_id: String,
        flair_name: String,
        flair_community_actor_id: String,
        assigned_on: DateTime<Utc>,
    ) -> Self {
        Self {
            user_actor_id,
            flair_name,
            flair_community_actor_id,
            assigned_on,
        }
    }
}

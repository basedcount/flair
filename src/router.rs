use axum::{
    extract::{Json, State},
    http::StatusCode, debug_handler,
};
use chrono::Utc;
use deadpool_sqlite::{rusqlite::params, Pool};
use serde::{Deserialize, Serialize};

use crate::internal_error;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct AddUserRequest {
    pub user_actor_id: String,
    pub community_actor_id: String,
    pub flair: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct DeleteUserRequest {
    user_actor_id: String,
    community_actor_id: String,
}

impl DeleteUserRequest {
    fn new(user_actor_id: String, community_actor_id: String) -> Self {
        Self {
            user_actor_id,
            community_actor_id,
        }
    }
}

#[debug_handler]
pub(crate) async fn put_user_flair(
    State(pool): State<Pool>,
    Json(payload): Json<AddUserRequest>,
) -> (StatusCode, String) {
    let conn = match pool.get().await {
        Ok(a) => a,
        Err(e) => return internal_error(e),
    };

    let flair_name = payload.flair.clone();

    if let Err(e) = conn
        .interact(move |conn| {
            return conn.execute(
                r"INSERT INTO user_flairs (user_actor_id, flair_id, assigned_on)
        SELECT ?, f.id, ?
        FROM flairs f
        WHERE f.name = ?
        LIMIT 1;",
                params![
                    payload.user_actor_id,
                    Utc::now().to_rfc3339(),
                    payload.flair
                ],
            );
        })
        .await
    {
        return crate::internal_error(e);
    }

    (
        StatusCode::CREATED,
        format!("Added flair {} to database!", flair_name),
    )
}



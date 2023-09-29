use axum::{
    debug_handler,
    extract::{Json, State},
    http::StatusCode,
    response::Html,
};
use chrono::Utc;
use deadpool_sqlite::{rusqlite::params, Pool};
use flair::Flair;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    db::{add_flair, get_community_flairs, get_user_community_flairs},
    internal_error,
};

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub(crate) struct AddUserRequest {
    pub user_actor_id: String,
    pub community_actor_id: String,
    pub flair_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub(crate) struct DeleteUserRequest {
    pub user_actor_id: String,
    pub community_actor_id: String,
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

    let flair_name = payload.flair_name.clone();
    let user_name = payload.user_actor_id.clone();

    if let Err(e) = conn
        .interact(move |conn| {
            return conn.execute(
                r"INSERT OR REPLACE INTO user_flairs (user_actor_id, flair_name, flair_community_actor_id, assigned_on)
                SELECT ?, ?, ?, ?
                ",
                params![
                    payload.user_actor_id,
                    payload.flair_name,
                    payload.community_actor_id,
                    Utc::now().to_rfc3339(),
                ],
            );
        })
        .await
    {
        return crate::internal_error(e);
    }

    (
        StatusCode::CREATED,
        format!("Assigned flair '{flair_name}' to user '{user_name}'"),
    )
}

#[debug_handler]
pub(crate) async fn delete_user(
    State(pool): State<Pool>,
    Json(payload): Json<DeleteUserRequest>,
) -> (StatusCode, String) {
    let conn = match pool.get().await {
        Ok(a) => a,
        Err(e) => return internal_error(e),
    };

    let actor_id = payload.user_actor_id.clone();

    if let Err(e) = conn
        .interact(move |conn| {
            return conn.execute(
                r"DELETE FROM user_flairs WHERE user_actor_id = ? AND flair_community_actor_id = ?",
                params![payload.user_actor_id, payload.community_actor_id],
            );
        })
        .await
    {
        return crate::internal_error(e);
    }

    (StatusCode::OK, format!("User {actor_id} deleted"))
}

#[debug_handler]
pub(crate) async fn render_index() -> Html<&'static str> {
    let template = include_str!("../views/index.html");

    Html(template)
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub(crate) struct AddFlairForm {
    pub name: String,
    pub display_name: String,
    pub path: Option<String>,
    pub community_actor_id: String,
    pub mod_only: bool,
}

#[debug_handler]
pub(crate) async fn post_community_flairs(
    State(pool): State<Pool>,
    Json(payload): Json<AddFlairForm>,
) -> (StatusCode, String) {
    let conn = match pool.get().await {
        Ok(a) => a,
        Err(e) => return internal_error(e),
    };

    let name = payload.name.clone();

    if let Err(e) = conn
        .interact(move |conn| return add_flair(&conn, &payload))
        .await
    {
        return crate::internal_error(e);
    }

    (StatusCode::CREATED, format!("Flair '{name}' created"))
}

#[derive(Debug, Deserialize, Serialize, Default, TS)]
#[ts(export)]
pub(crate) struct GetUserFlairRequest {
    pub community_actor_id: String,
    pub user_actor_id: String,
}

#[debug_handler]
pub(crate) async fn get_user_flair(
    State(pool): State<Pool>,
    Json(payload): Json<GetUserFlairRequest>,
) -> Result<Json<Option<Flair>>, StatusCode> {
    let conn = match pool.get().await {
        Ok(a) => a,
        Err(e) => return Err(internal_error(e).0),
    };

    let result = conn
        .interact(move |conn| {
            get_user_community_flairs(conn, &payload).expect("Issue fetching user flairs")
        })
        .await;

    match result {
        Ok(flair) => Ok(Json(flair)),
        Err(e) => {
            eprintln!("{}", e); // Fixed the logging interpolation here as well
            Err(crate::internal_error(e).0)
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default, TS)]
#[ts(export)]
pub(crate) struct GetCommunityFlairsRequest {
    pub community_actor_id: String,
    pub mod_only: Option<bool>,
}

#[debug_handler]
pub(crate) async fn get_community_flairs_api(
    State(pool): State<Pool>,
    Json(payload): Json<GetCommunityFlairsRequest>,
) -> Result<Json<Vec<Flair>>, StatusCode> {
    let conn = match pool.get().await {
        Ok(a) => a,
        Err(e) => return Err(internal_error(e).0),
    };

    let result = conn
        .interact(move |conn| {
            get_community_flairs(conn, &payload).expect("Issue getting community flairs")
        })
        .await;

    match result {
        Ok(flairs) => Ok(Json(flairs)),
        Err(e) => {
            eprintln!("{}", e); // Fixed the logging interpolation here as well
            Err(crate::internal_error(e).0)
        }
    }
}

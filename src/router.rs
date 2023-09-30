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
    db::{add_flair, get_community_flairs, get_user_flair},
    internal_error,
};

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub(crate) struct AddUserFlairJson {
    pub user_actor_id: String,
    pub community_actor_id: String,
    pub flair_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub(crate) struct DeleteUserFlairJson {
    pub user_actor_id: String,
    pub community_actor_id: String,
}

#[debug_handler]
pub(crate) async fn put_user_flair_api(
    State(pool): State<Pool>,
    Json(payload): Json<AddUserFlairJson>,
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
                VALUES (?, ?, ?, ?)
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
pub(crate) async fn delete_user_api(
    State(pool): State<Pool>,
    Json(payload): Json<DeleteUserFlairJson>,
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

    (
        StatusCode::OK,
        format!("Removed flair from user '{actor_id}'"),
    )
}

#[debug_handler]
pub(crate) async fn render_index() -> Html<&'static str> {
    let template = include_str!("../views/index.html");

    Html(template)
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub(crate) struct AddFlairJson {
    pub name: String,
    pub display_name: String,
    pub path: Option<String>,
    pub community_actor_id: String,
    pub mod_only: bool,
}

#[debug_handler]
pub(crate) async fn put_community_flairs_api(
    State(pool): State<Pool>,
    Json(payload): Json<AddFlairJson>,
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

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub(crate) struct DeleteFlairJson {
    pub name: String,
    pub community_actor_id: String,
}

#[debug_handler]
pub(crate) async fn delete_community_flairs_api(
    State(pool): State<Pool>,
    Json(payload): Json<DeleteFlairJson>,
) -> (StatusCode, String) {
    let conn = match pool.get().await {
        Ok(a) => a,
        Err(e) => return internal_error(e),
    };

    let flair_name = payload.name.clone();
    let community_actor_id = payload.community_actor_id.clone();

    if let Err(e) = conn
        .interact(move |conn| {
            return conn.execute(
                r"DELETE FROM flairs WHERE name = ? AND community_actor_id = ?",
                params![payload.name, payload.community_actor_id],
            );
        })
        .await
    {
        return crate::internal_error(e);
    }

    (
        StatusCode::OK,
        format!("Removed flair '{flair_name}' from community'{community_actor_id}'"),
    )
}

#[derive(Debug, Deserialize, Serialize, Default, TS)]
#[ts(export)]
pub(crate) struct GerUserFlairJson {
    pub community_actor_id: String,
    pub user_actor_id: String,
}

#[debug_handler]
pub(crate) async fn get_user_flair_api(
    State(pool): State<Pool>,
    Json(payload): Json<GerUserFlairJson>,
) -> Result<Json<Option<Flair>>, StatusCode> {
    let conn = match pool.get().await {
        Ok(a) => a,
        Err(e) => return Err(internal_error(e).0),
    };

    let result = conn
        .interact(move |conn| get_user_flair(conn, &payload).expect("Issue fetching user flairs"))
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
pub(crate) struct GetFlairsJson {
    pub community_actor_id: String,
    pub mod_only: Option<bool>,
}

#[debug_handler]
pub(crate) async fn get_community_flairs_api(
    State(pool): State<Pool>,
    Json(payload): Json<GetFlairsJson>,
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

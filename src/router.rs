use axum::{
    debug_handler,
    extract::{Json, State},
    http::StatusCode,
    response::Html,
    Form,
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
    pub flair: String,
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
                r"delete from user_flairs where user_actor_id = ?",
                params![payload.user_actor_id,],
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
pub(crate) async fn post_index(
    State(pool): State<Pool>,
    Form(payload): Form<AddFlairForm>,
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

    (StatusCode::CREATED, format!("Flair {name} created"))
}

#[debug_handler]
pub(crate) async fn post_index_json(
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
pub(crate) struct CommunityActorQuery {
    pub actor_id: String,
    pub user_id: Option<String>,
    pub mod_only: Option<bool>,
}

#[debug_handler]
pub(crate) async fn get_community_info(
    State(pool): State<Pool>,
    Json(payload): Json<CommunityActorQuery>,
) -> Result<Json<Vec<Flair>>, StatusCode> {
    let conn = match pool.get().await {
        Ok(a) => a,
        Err(e) => return Err(internal_error(e).0),
    };

    let result = conn
        .interact(move |conn| {
            if payload.user_id.is_some() {
                get_user_community_flairs(conn, &payload).expect("Issue fetching user flairs")
            } else {
                get_community_flairs(conn, &payload).expect("Issue getting flairs")
            }
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

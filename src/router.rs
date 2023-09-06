use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use deadpool_sqlite::{rusqlite::params, Pool};
use flairs::FlairDirectory;

use crate::internal_error;

pub(crate) async fn get_user_flair(
    Query(params): Query<HashMap<String, String>>,
    State(pool): State<Pool>,
) -> (StatusCode, String) {
    let id = match params.get("id") {
        Some(i) => i,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                String::from("You need to set the 'id' url query"),
            )
        }
    };

    let conn = match pool.get().await.map_err(crate::internal_error) {
        Ok(a) => a,
        Err(e) => return e,
    };

    let submit_params = id.clone();
    let _result = conn
        .interact(move |conn| {
            let mut stmt = conn
                .prepare("SELECT * FROM flairs where user = ?1")
                .unwrap();
            let mut rows = stmt.query([submit_params]).unwrap();
            while let Some(_row) = rows.next().unwrap() {}
        })
        .await
        .unwrap();

    (StatusCode::OK, format!("You asked for me? {id}"))
}

pub(crate) async fn add_user(
    State(pool): State<Pool>,
    Json(payload): Json<FlairDirectory>,
) -> (StatusCode, String) {
    let conn = match pool.get().await {
        Ok(a) => a,
        Err(e) => return internal_error(e),
    };

    if let Err(e) = conn
        .interact(move |conn| {
            return conn
                .execute(
                    r"INSERT INTO flair_directory
        (special, ref_id, pos, flair, path)
        Values (?, ?, ?, ?, ?)
        ",
                    params![
                        payload.special,
                        payload.ref_id,
                        payload.pos,
                        payload.flair,
                        payload.path
                    ],
                )
                .map_err(internal_error);
        })
        .await
    {
        return crate::internal_error(e);
    }

    (StatusCode::CREATED, format!("Welcome to the party! 🎉"))
}

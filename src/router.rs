use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use deadpool_sqlite::{rusqlite::params, Pool};
use flair::FlairDirectory;

use crate::internal_error;

pub(crate) async fn get_user_flair(
    Query(params): Query<HashMap<String, String>>,
    State(pool): State<Pool>,
) -> Result<Json<Vec<FlairDirectory>>, (StatusCode, String)> {
    let id = match params.get("id") {
        Some(i) => i,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                String::from("You need to set the 'id' url query"),
            ))
        }
    };

    let conn = match pool.get().await.map_err(crate::internal_error) {
        Ok(a) => a,
        Err(e) => return Err(e),
    };

    let submit_params = id.clone();
    match conn
        .interact(move |conn| {
            let mut stmt = conn
                .prepare("SELECT ID, special, ref_id, pos, flair, path FROM flairs where ID = ?")
                .unwrap();
            let mut rows = stmt.query([submit_params]).unwrap();
            let mut users: Vec<FlairDirectory> = vec![];
            while let Some(row) = rows.next().unwrap() {
                users.push(FlairDirectory::new(
                    row.get(0).unwrap_or(None),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                    row.get(3).unwrap(),
                    row.get(4).unwrap(),
                    row.get(5).unwrap_or(None),
                ))
            }

            return users;
        })
        .await.map_err(internal_error)
    {
        Err(e) => return Err(e),
        Ok(o) => return Ok(Json(o)),
    }

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

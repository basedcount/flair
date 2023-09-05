use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    http::StatusCode,
};
use deadpool_sqlite::Pool;

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
    let result = conn
        .interact(move |conn| {
            let mut stmt = conn
                .prepare("SELECT * FROM flairs where user = ?1")
                .unwrap();
            let mut rows = stmt.query([submit_params]).unwrap();
            while let Some(row) = rows.next().unwrap() {
                
            }
        })
        .await
        .unwrap();

    (StatusCode::OK, format!("You asked for me? {id}"))
}

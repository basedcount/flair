use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    http::StatusCode,
};

use crate::ConnectionPool;

pub(crate) async fn get_user_flair(
    Query(params): Query<HashMap<String, String>>,
    State(pool): State<ConnectionPool>
) -> (StatusCode, String) {
    let id = match params.get("id") {
        Some(i) => i,
        None => return (StatusCode::BAD_REQUEST, String::from("You need to set the 'id' url query")),
    };

    (StatusCode::OK, format!("You asked for me? {id}"))
}

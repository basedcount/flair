use reqwest::header::COOKIE;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize)]
struct Site {
    my_user: MyUser,
}

#[derive(Serialize, Deserialize)]
struct MyUser {
    moderates: Vec<Moderate>,
    local_user_view: LocalUser,
}

#[derive(Serialize, Deserialize)]
struct LocalUser {
    person: Person,
}

#[derive(Serialize, Deserialize)]
struct Person {
    actor_id: String
}

#[derive(Serialize, Deserialize)]
struct Moderate {
    community: Community,
}

#[derive(Serialize, Deserialize)]
struct Community {
    actor_id: String,
}

///Polls the Lemmy API, verifies if a user is allowed to edit a flair
/// - **Mods** are allowed to change everyone's flair within the community they moderate
/// - **Users** can only change their own flair
pub async fn verify_user(
    lemmy_port: &u16,
    jwt: &str,
    user_actor_id: &str,
    community_actor_id: &str,
) ->Result<bool, Box<dyn Error>>{
    let url = format!("http://127.0.0.1:{}/api/v3/site?auth={}", lemmy_port, jwt);
    let cookie = format!("jwt={}", jwt);

    let client = reqwest::Client::new();
    let res = client.get(&url).header(COOKIE, cookie).send().await?;

    let json: Site = res.json().await?;
    let moderated = &json
        .my_user
        .moderates
        .into_iter()
        .map(|el| el.community.actor_id)
        .collect::<Vec<String>>();
    let person_actor_id = json.
        my_user
        .local_user_view
        .person
        .actor_id;

    Ok(person_actor_id == user_actor_id || moderated.contains(&community_actor_id.to_string()))
}

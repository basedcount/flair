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
    actor_id: String,
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
    docker: &bool,
    jwt: &str,
    user_actor_id: &str,
    community_actor_id: &str,
    local_domain: &str,
    user_domain: &str,
) -> Result<bool, Box<dyn Error>> {
    let url = get_url(lemmy_port, jwt, local_domain, user_domain, docker);

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
    let person_actor_id = json.my_user.local_user_view.person.actor_id;

    Ok(person_actor_id == user_actor_id || moderated.contains(&community_actor_id.to_string()))
}

///Polls the Lemmy API, verifies if a user is a community moderator
/// - **Mods** are allowed to add and delete community flairs
/// - **Users** aren't allowed to do anything
pub async fn verify_mod(
    lemmy_port: &u16,
    docker: &bool,
    jwt: &str,
    community_actor_id: &str,
    local_domain: &str,
    user_domain: &str,
) -> Result<bool, Box<dyn Error>> {
    let url = get_url(lemmy_port, jwt, local_domain, user_domain, docker);

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

    Ok(moderated.contains(&community_actor_id.to_string()))
}

fn get_url(port: &u16, jwt: &str, local_domain: &str, user_domain: &str, docker: &bool) -> String {
    let url = if !local_domain.eq(user_domain) {
        format!("https://{}/api/v3/site?auth={}", user_domain, jwt)
    } else if *docker {
        format!("http://lemmy:{}/api/v3/site?auth={}", port, jwt)
    } else {
        format!("http://127.0.0.1:{}/api/v3/site?auth={}", port, jwt)
    };

    return url;
}

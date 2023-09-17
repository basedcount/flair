use chrono::{DateTime, Utc};
use deadpool_sqlite::rusqlite::params;
use deadpool_sqlite::rusqlite::Connection;

use flair::Flair;

use crate::router::AddFlairForm;
use crate::router::CommunityActorQuery;

pub(crate) fn get_user_community_flairs(
    client: &mut Connection,
    pl: &CommunityActorQuery,
) -> anyhow::Result<Vec<Flair>> {
    let mut stmt = client.prepare_cached(
        "SELECT f.name, f.display_name, f.path, f.assigned_on, f.community_actor_id, f.mod_only
    FROM flairs f
    JOIN user_flairs uf ON f.id = uf.flair_id
    WHERE uf.user_actor_id = ? AND f.community_actor_id = ? AND f.mod_only = ?;
    ",
    )?;

    let mut rows = stmt
        .query(params![
            urlencoding::encode(&pl.actor_id),
            urlencoding::encode(&pl.id.as_ref().unwrap()),
            pl.mod_only
        ])
        .unwrap();

    let mut val: Vec<Flair> = vec![];
    while let Ok(s) = rows.next() {
        if let Some(r) = s {
            let time: String = r.get(4).unwrap();
            let time_p = parse_date_time(&time).unwrap();
            val.push(Flair::new(
                r.get(0).unwrap(),
                r.get(1).unwrap(),
                r.get(2).unwrap(),
                r.get(3).unwrap(),
                time_p,
                r.get(5).unwrap(),
                r.get(6).unwrap(),
            ))
        }
    }

    Ok(val)
}

pub(crate) fn get_community_flairs(
    client: &mut Connection,
    pl: &CommunityActorQuery,
) -> anyhow::Result<Vec<Flair>> {
    let mut stmt =
        client.prepare_cached("select * flairs where community_actor_id = ? and mod_only = ?")?;

    let mut rows = stmt
        .query(params![urlencoding::encode(&pl.actor_id), &pl.mod_only])
        .unwrap();

    let mut val: Vec<Flair> = vec![];
    while let Ok(s) = rows.next() {
        if let Some(r) = s {
            let time: String = r.get(4).unwrap();
            let time_p = parse_date_time(&time).unwrap();
            val.push(Flair::new(
                r.get(0).unwrap(),
                r.get(1).unwrap(),
                r.get(2).unwrap(),
                r.get(3).unwrap(),
                time_p,
                r.get(5).unwrap(),
                r.get(6).unwrap(),
            ))
        }
    }

    Ok(val)
}

pub(crate) fn parse_date_time(info: &str) -> anyhow::Result<DateTime<Utc>> {
    let offset = DateTime::parse_from_rfc3339(&info)?;
    Ok(DateTime::from(offset))
}

pub(crate) fn add_flair(client: &Connection, pl: &AddFlairForm) -> anyhow::Result<usize> {
    let pl_path: Option<String> = match &pl.path {
        Some(p) => {
            let val = urlencoding::encode(&p).to_string();
            Some(val)
        }
        None => None,
    };

    let result = client.execute(
        r"insert into flairs
        (name, display_name, path, assigned_on, community_actor_id, mod_only)
        values (?,?,?,?,?,?)",
        params![
            urlencoding::encode(&pl.name),
            urlencoding::encode(&pl.display_name),
            pl_path,
            urlencoding::encode(&Utc::now().to_rfc3339()),
            urlencoding::encode(&pl.community_actor_id),
            &pl.mod_only,
        ],
    )?;

    Ok(result)
}

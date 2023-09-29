use chrono::{DateTime, Utc};
use deadpool_sqlite::rusqlite::params;
use deadpool_sqlite::rusqlite::Connection;

use flair::Flair;

use crate::router::AddFlairForm;
use crate::router::GetCommunityFlairsRequest;
use crate::router::GetUserFlairRequest;

pub(crate) fn get_user_community_flairs(
    client: &mut Connection,
    pl: &GetUserFlairRequest,
) -> anyhow::Result<Vec<Flair>> {
    let mut stmt = client.prepare_cached(
        "SELECT f.name, f.display_name, f.path, f.assigned_on, f.community_actor_id, f.mod_only
    FROM flairs f
    JOIN user_flairs uf ON f.id = uf.flair_id
    WHERE uf.user_actor_id = ? AND f.community_actor_id = ?;
    ",
    )?;

    let mut rows = stmt
        .query(params![
            urlencoding::encode(&pl.community_actor_id),
            urlencoding::encode(&pl.user_actor_id),
        ])
        .unwrap();

    let mut val: Vec<Flair> = vec![];
    while let Ok(s) = rows.next() {
        if let Some(r) = s {
            let time: String = r.get(4).unwrap();
            let time_p = parse_date_time(&time).unwrap();
            let path_r: String = r.get(3).unwrap_or_default();
            let path_x = urlencoding::decode(&path_r).unwrap().to_string();
            let mut flair = Flair::new(
                r.get(0).unwrap(),
                r.get(1).unwrap(),
                r.get(2).unwrap(),
                Some(path_x),
                time_p,
                r.get(5).unwrap(),
                r.get(6).unwrap(),
            );

            flair.community_actor_id = urlencoding::decode(&flair.community_actor_id)
                .unwrap()
                .to_string();

            val.push(flair)
        } else {
            break;
        }
    }

    Ok(val)
}

pub(crate) fn get_community_flairs(
    client: &mut Connection,
    pl: &GetCommunityFlairsRequest,
) -> anyhow::Result<Vec<Flair>> {
    let mut stmt = client
        .prepare_cached("select * from flairs where community_actor_id = ? and mod_only = ?")?;

    let mut rows = stmt
        .query(params![
            urlencoding::encode(&pl.community_actor_id),
            &pl.mod_only.unwrap_or(false)
        ])
        .unwrap();

    let mut val: Vec<Flair> = vec![];
    while let Ok(s) = rows.next() {
        if let Some(r) = s {
            let time: String = r.get(4).unwrap();
            let time_p = parse_date_time(&time).unwrap();
            let path_r: String = r.get(3).unwrap_or_default();
            let path_x = urlencoding::decode(&path_r).unwrap().to_string();
            let dsply_r: String = r.get(2).unwrap();
            let dslpy_x = urlencoding::decode(&dsply_r).unwrap().to_string();

            let mut flair = Flair::new(
                r.get(0).unwrap(),
                r.get(1).unwrap(),
                dslpy_x,
                Some(path_x),
                time_p,
                r.get(5).unwrap(),
                r.get(6).unwrap(),
            );

            flair.community_actor_id = urlencoding::decode(&flair.community_actor_id)
                .unwrap()
                .to_string();

            val.push(flair);
        } else {
            break;
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
            &Utc::now().to_rfc3339(),
            urlencoding::encode(&pl.community_actor_id),
            &pl.mod_only,
        ],
    )?;

    Ok(result)
}

use deadpool_sqlite::rusqlite::params;
use deadpool_sqlite::rusqlite::Connection;

use flair::Flair;

use crate::router::AddFlairJson;
use crate::router::GetFlairsJson;
use crate::router::GerUserFlairJson;

pub(crate) fn get_user_flair(
    client: &mut Connection,
    pl: &GerUserFlairJson,
) -> anyhow::Result<Option<Flair>> {
    let mut stmt = client.prepare_cached("
            SELECT f.name, f.display_name, f.path, f.community_actor_id, f.mod_only
            FROM flairs f
            JOIN user_flairs uf ON f.name = uf.flair_name AND f.community_actor_id = uf.flair_community_actor_id
            WHERE f.community_actor_id = ? AND uf.user_actor_id = ?;
        ",
    )?;

    let mut rows = stmt
        .query(params![pl.community_actor_id, pl.user_actor_id,])
        .unwrap();

    if let Ok(Some(r)) = rows.next() {
        let flair = Flair::new(
            r.get(0).unwrap(),
            r.get(1).unwrap(),
            r.get(2).unwrap_or_default(),
            r.get(3).unwrap(),
            r.get(4).unwrap(),
        );

        Ok(Some(flair))
    } else {
        Ok(None)
    }
}

pub(crate) fn get_community_flairs(
    client: &mut Connection,
    pl: &GetFlairsJson,
) -> anyhow::Result<Vec<Flair>> {
    //If mod only == true display both non mod and mod flairs
    //If mod only == false display only non mod flairs
    let mut stmt = client.prepare_cached(
        "SELECT name, display_name, path, community_actor_id, mod_only
            FROM flairs
            WHERE community_actor_id = ? and mod_only <= ?
        ",
    )?;

    let mut rows = stmt
        .query(params![
            pl.community_actor_id,
            &pl.mod_only.unwrap_or(false)
        ])
        .unwrap();

    let mut val: Vec<Flair> = vec![];
    while let Ok(s) = rows.next() {
        if let Some(r) = s {
            let flair = Flair::new(
                r.get(0).unwrap(),
                r.get(1).unwrap(),
                r.get(2).unwrap_or_default(),
                r.get(3).unwrap(),
                r.get(4).unwrap(),
            );

            val.push(flair);
        } else {
            break;
        }
    }

    Ok(val)
}

pub(crate) fn add_flair(client: &Connection, pl: &AddFlairJson) -> anyhow::Result<usize> {
    let result = client.execute(
        r"INSERT INTO flairs (name, display_name, path, community_actor_id, mod_only)
            VALUES (?,?,?,?,?)
            ",
        params![
            pl.name,
            pl.display_name,
            pl.path,
            pl.community_actor_id,
            &pl.mod_only,
        ],
    )?;

    Ok(result)
}


use server::db_conn::DbConn;
use diesel::QueryResult;
use common::race_run::NewRaceRun;
use models::*;
use diesel::*;

pub fn create_racerun(conn: &PgConnection, new_run: &NewRaceRun) -> QueryResult<i32> {
    let run = create_run(conn, new_run.duration_in_seconds as i32)?;
    let zones = new_run
        .zones
        .iter()
        .map(|zone| {
            create_zoneentry(conn, zone.seconds_after_start as i32, run.id, &zone.name)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let levels = new_run
        .levels
        .iter()
        .map(|level| {
            create_levelup(
                conn,
                level.seconds_after_start as i32,
                run.id,
                level.level as i16,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(run.id)
}

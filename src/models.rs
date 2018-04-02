
use diesel::insert_into;
use super::schema::runs;
use super::schema::zones;
use super::schema::levels;
use diesel::prelude::*;


#[derive(Insertable)]
#[table_name = "runs"]
struct NewRun {
    pub duration_in_seconds: i32,
}

pub fn create_run(conn: &PgConnection, duration_in_seconds: i32) -> QueryResult<Run> {
    let new_run = NewRun { duration_in_seconds };

    insert_into(runs::table).values(&new_run).get_result(conn)
}

#[derive(Insertable)]
#[table_name = "levels"]
struct NewLevelUp {
    run_id: i32,
    duration_in_seconds: i32,
    level: i16,
}

pub fn create_levelup(
    conn: &PgConnection,
    duration_in_seconds: i32,
    run_id: i32,
    level: i16,
) -> QueryResult<LevelUp> {
    let lvlup = NewLevelUp {
        duration_in_seconds,
        run_id,
        level,
    };
    insert_into(levels::table).values(&lvlup).get_result(conn)
}

#[derive(Insertable)]
#[table_name = "zones"]
pub struct NewZoneEntry<'a> {
    run_id: i32,
    duration_in_seconds: i32,
    name: &'a str,
}

pub fn create_zoneentry(
    conn: &PgConnection,
    duration_in_seconds: i32,
    run_id: i32,
    name: &str,
) -> QueryResult<ZoneEntry> {
    let lvlup = NewZoneEntry {
        duration_in_seconds,
        run_id,
        name,
    };
    insert_into(zones::table).values(&lvlup).get_result(conn)
}


#[derive(Identifiable, Queryable)]
pub struct Run {
    pub id: i32,
    duration_in_seconds: i32,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Run, foreign_key = "run_id")]
#[table_name = "levels"]
pub struct LevelUp {
    id: i32,
    run_id: i32,
    level: i16,
    duration_in_seconds: i32,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Run, foreign_key = "run_id")]
#[table_name = "zones"]
pub struct ZoneEntry {
    id: i32,
    run_id: i32,
    name: String,
    duration_in_seconds: i32,
}

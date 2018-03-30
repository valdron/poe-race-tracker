use super::schema::runs;
use super::schema::zones;
use super::schema::levels;


#[derive(Insertable)]
#[table_name="runs"]
struct NewRun {
    duration_in_seconds: i32
}

#[derive(Insertable)]
#[table_name="levels"]
struct NewLevelUp {
    run_id: i32,
    duration_in_seconds: i32,
    level: i16,
}

#[derive(Insertable)]
#[table_name="zones"]
struct NewZoneEntry<'a> {
    run_id: i32,
    duration_in_seconds: i32,
    name: &'a str,
}


#[derive(Identifiable, Queryable)]
struct Run {
    id: i32,
    duration_in_seconds: i32
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Run, foreign_key = "run_id")]
#[table_name="levels"]
struct LevelUp {
    id: i32,
    run_id: i32,
    duration_in_seconds: i32,
    level: i16,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Run, foreign_key = "run_id")]
#[table_name="zones"]
struct ZoneEntry {
    id: i32,
    run_id: i32,
    duration_in_seconds: i32,
    name: String,
}



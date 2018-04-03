use common::race_run::{self, NewRaceRun};
use db::models::*;
use db::schema::*;
use diesel::QueryResult;
use diesel::prelude::*;
use diesel::result;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use server::db_conn::DbConn;
use server::db_conn::Pool;
use server::runstore::RunStore;

pub struct DbExecuter {
    conn: DbConn,
}

impl<'a, 'r> FromRequest<'a, 'r> for DbExecuter {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbExecuter, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Self { conn: DbConn(conn) }),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl RunStore for DbExecuter {
    type Error = result::Error;
    type Key = i32;
    fn get_racerun(&self, id: &Self::Key) -> QueryResult<Option<NewRaceRun>> {
        self.get_run_by_id(&id).optional()
    }
    fn create_racerun(&self, new_run: &NewRaceRun) -> QueryResult<Self::Key> {
        let run = create_run(&self.conn, new_run.duration_in_seconds as i32)?;
        let _ = new_run
            .zones
            .iter()
            .map(|zone| {
                create_zoneentry(
                    &self.conn,
                    zone.seconds_after_start as i32,
                    run.id,
                    &zone.name,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;

        let _ = new_run
            .levels
            .iter()
            .map(|level| {
                create_levelup(
                    &self.conn,
                    level.seconds_after_start as i32,
                    run.id,
                    level.level as i16,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(run.id)
    }
}

impl DbExecuter {
    fn get_run_by_id(&self, id: &i32) -> QueryResult<NewRaceRun> {
        let run: Run = runs::table.find(id).get_result(&*self.conn)?;
        let db_zones: Vec<ZoneEntry> = ZoneEntry::belonging_to(&run).load(&*self.conn)?;
        let db_levels: Vec<LevelUp> = LevelUp::belonging_to(&run).load(&*self.conn)?;
        let zones = db_zones
            .into_iter()
            .map(|zone| race_run::ZoneEntry::new(zone.name, zone.duration_in_seconds as u64))
            .collect();

        let levels = db_levels
            .into_iter()
            .map(|levelup| {
                race_run::LevelUp::new(levelup.level as u8, levelup.duration_in_seconds as u64)
            })
            .collect();

        let new_race_run = NewRaceRun::new(run.duration_in_seconds as u64, levels, zones);
        Ok(new_race_run)
    }
}

#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_attribute)]

extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod common;
mod db;
mod server;

use common::race_run::NewRaceRun;
use diesel::QueryResult;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use rocket_contrib::Json;
use server::db_conn::Pool;
use server::db_executer::DbExecuter;
use server::runstore::RunStore;

static DATABASE_URL: &'static str = dotenv!("DATABASE_URL");

fn init_pool() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL);
    r2d2::Pool::new(manager).expect("db pool")
}

#[post("/runs", format = "application/json", data = "<run>")]
fn create_run(store: DbExecuter, run: Json<NewRaceRun>) -> QueryResult<Json<i32>> {
    store.create_racerun(&run).map(|id| Json(id))
}

#[get("/run/<run_id>")]
fn get_run(store: DbExecuter, run_id: i32) -> QueryResult<Option<Json<NewRaceRun>>> {
    store
        .get_racerun(&run_id)
        .map(|run_opt| run_opt.map(|run| Json(run)))
}

fn main() {
    rocket::ignite()
        .manage(init_pool())
        .mount("/", routes![create_run, get_run])
        .launch();
}

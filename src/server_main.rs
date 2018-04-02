#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_attribute)]

extern crate rocket;
extern crate rocket_contrib;
extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate diesel;
extern crate r2d2_diesel;
extern crate r2d2;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod common;
mod server;
pub mod schema;
pub mod models;

use diesel::OptionalExtension;
use diesel::QueryResult;
use server::db_executer::DbExecuter;
use rocket_contrib::Json;
use common::race_run::NewRaceRun;
use server::db_conn::Pool;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

static DATABASE_URL: &'static str = dotenv!("DATABASE_URL");

fn init_pool() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL);
    r2d2::Pool::new(manager).expect("db pool")
}

#[post("/runs", format = "application/json", data = "<run>")]
fn create_run(exec: DbExecuter, run: Json<NewRaceRun>) -> QueryResult<Json<i32>> {

    exec.create_racerun(&run).map(|id| Json(id))
}

#[get("/run/<run_id>")]
fn get_run(exec: DbExecuter, run_id: i32) -> QueryResult<Option<Json<NewRaceRun>>> {
    exec.get_racerun(run_id).optional().map(|run_opt| run_opt.map(|run|Json(run)))
}

fn main() {
    rocket::ignite()
        .manage(init_pool())
        .mount("/", routes![create_run, get_run])
        .launch();
}

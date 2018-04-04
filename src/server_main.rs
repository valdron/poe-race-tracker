#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_attribute)]

extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;
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
#[cfg(test)]
mod server_tests;

use common::race_run::NewRaceRun;
use diesel::QueryResult;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use rocket_contrib::Json;
use server::db_conn::Pool;
use server::db_executer::DbExecuter;
use server::runstore::RunStore;

pub fn init_pool(db_url: &str) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::new(manager).expect("db pool")
}

#[post("/run", format = "application/json", data = "<run>")]
pub fn create_run(store: DbExecuter, run: Json<NewRaceRun>) -> QueryResult<Json<i32>> {
    store.create_racerun(&run).map(|id| Json(id))
}

#[get("/run/<run_id>")]
fn get_run(store: DbExecuter, run_id: i32) -> QueryResult<Option<Json<NewRaceRun>>> {
    store.get_racerun(&run_id).map(|run_opt| {
        run_opt.map(|run| Json(run))
    })
}

fn main() {
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL was expected to be set");

    rocket::ignite()
        .manage(init_pool(&db_url))
        .mount("/", routes![create_run, get_run])
        .launch();
}

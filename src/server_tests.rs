use super::*;
use rocket::http::*;
use rocket::local::*;

const BRUTUS_RUN_JSON: &str = include_str!("../test_runs/brutusrun.json");

fn get_rocket_instance() -> rocket::Rocket {
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL was expected to be set");

    let rocket = rocket::ignite()
        .manage(init_pool(&db_url))
        .mount("/", routes![create_run, get_run]);

    rocket
}

#[test]
fn upload_and_then_download() {
    let rocket = get_rocket_instance();
    let client = Client::new(rocket).expect("Expected rocket instance");

    let mut res_create = client
        .post("/run")
        .body(BRUTUS_RUN_JSON)
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(res_create.status(), Status::Ok);

    let id: i32 = res_create
        .body_string()
        .expect("Expected response body")
        .parse()
        .expect("expected numerical id");

    let mut res = client.get(format!("/run/{}", id)).dispatch();

    assert_eq!(res.status(), Status::Ok);

    let run: NewRaceRun = serde_json::from_reader(
        res.body().expect("expected response body").into_inner(),
    ).expect("expected parsable response body");

    let expected_run = serde_json::from_str(BRUTUS_RUN_JSON).unwrap();

    assert_eq!(run, expected_run);
}

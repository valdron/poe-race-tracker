#![feature(conservative_impl_trait)]

extern crate chrono;
extern crate env_logger;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod client;
mod common;


use client::get_zone_entries;
use client::fill_vec_and_return_end_time;
use client::{wait_for_start_of_run, get_race_iter, get_level_ups};
use common::race_run::ZoneEntry;
use common::race_run::LevelUp;
use client::client_error::ClientResult;
use std::path::Path;
use log::LevelFilter;
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom};

use client::logline_generator::{DefaultLogLineGenerator, LogLineGenerator};
use common::race_run::NewRaceRun;

const CLIENT_TXT: &str =
    "C:\\Program Files (x86)\\Grinding Gear Games\\Path of Exile\\logs\\Client.txt";

fn main() {
    let mut builder = env_logger::Builder::from_default_env();
    builder.filter(None, LevelFilter::Debug).init();
    info!("Starting!");
    run().unwrap();
}

fn run() -> ClientResult<()> {
    let file = get_file_seeked_to_end(CLIENT_TXT)?;
    let log_line_generator = DefaultLogLineGenerator::from_reader(file);

    let mut event_iter = get_race_iter(log_line_generator).inspect(|e| debug!("{:?}", e));

    let start = wait_for_start_of_run(&mut event_iter)?;

    let mut v = Vec::new();

    info!("starting to process events!");
    let end = fill_vec_and_return_end_time(&mut event_iter, &mut v)?;
    info!("run finished compiling info and sending");

    let duration = end.signed_duration_since(start).num_seconds() as u64;

    let levels: Vec<LevelUp> = get_level_ups(start, &v);
    let zones: Vec<ZoneEntry> = get_zone_entries(start, &v);

    let run = NewRaceRun::new(duration, levels, zones);

    println!("{:#?}", run);
    let mut save_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("./save")?;
    serde_json::to_writer_pretty(&mut save_file, &run)?;
    Ok(())
}

fn get_file_seeked_to_end<T: AsRef<Path>>(s: T) -> ClientResult<File> {
    debug!("Opening File: {:?}", s.as_ref());
    let mut f = File::open(s)?;
    f.seek(SeekFrom::End(0))?;
    Ok(f)
}

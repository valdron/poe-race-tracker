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

use common::race_run::ZoneEntry;
use common::race_run::LevelUp;
use chrono::Local;
use chrono::DateTime;
use client::race_event::SimpleEvent;
use client::client_error::ClientResult;
use std::path::Path;
use log::LevelFilter;
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom};

use client::clientlog::ClientLogLine;
use client::client_error::ClientError;
use client::logline_generator::{DefaultLogLineGenerator, LogLineGenerator};
use common::race_run::RaceRun;

const CLIENT_TXT: &str =
    "C:\\Program Files (x86)\\Grinding Gear Games\\Path of Exile\\logs\\Client.txt";

type EventTime = (DateTime<Local>, SimpleEvent);

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

    let run = RaceRun::new(duration, levels, zones);

    println!("{:#?}", run);
    let mut save_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("./save")?;
    serde_json::to_writer_pretty(&mut save_file, &run)?;
    Ok(())
}

fn get_race_iter<I: Iterator<Item = std::io::Result<String>>>(
    i: I,
) -> impl Iterator<Item = ClientResult<EventTime>> {
    i.map(|line_result| -> ClientResult<EventTime> {
        let line = line_result?;
        let cll: ClientLogLine = line.parse()?;
        let event: SimpleEvent = cll.message.parse()?;
        Ok((cll.date, event))
    }).filter_map(|event_result| match event_result {
        Err(ClientError::EventParseError) => None,
        item @ _ => Some(item),
    })
}

fn wait_for_start_of_run<T: Iterator<Item = ClientResult<EventTime>>>(
    i: &mut T,
) -> ClientResult<DateTime<Local>> {
    info!("waiting for start command!");
    while let Some(item) = i.next() {
        match item? {
            (start, SimpleEvent::StartRun) => return Ok(start),
            _ => continue,
        }
    }
    Err("Ended before start!".to_owned().into())
}

fn fill_vec_and_return_end_time<T: Iterator<Item = ClientResult<EventTime>>>(
    i: &mut T,
    v: &mut Vec<EventTime>,
) -> ClientResult<DateTime<Local>> {
    while let Some(item) = i.next() {
        let item = item?;
        if let (time, SimpleEvent::EndRun) = item {
            return Ok(time);
        } else {
            v.push(item);
        }
    }
    Err("Ended before race end!".to_owned().into())
}

fn get_level_ups(start: DateTime<Local>, v: &[EventTime]) -> Vec<LevelUp> {
    v.iter().filter_map(|event| {
        match event { 
            &(time, SimpleEvent::LevelUp(level)) => {
                Some(LevelUp::new(level, time.signed_duration_since(start).num_seconds() as u64))
            }
            _ => None
            }
    }).collect()
}

fn get_zone_entries(start: DateTime<Local>, v: &[EventTime]) -> Vec<ZoneEntry> {
    v.iter().filter_map(|event| {
        match event { 
            &(time, SimpleEvent::EnterZone(ref name)) => {
                Some(ZoneEntry::new(name.clone(), time.signed_duration_since(start).num_seconds() as u64))
            }
            _ => None
            }
    }).collect()
}

fn get_file_seeked_to_end<T: AsRef<Path>>(s: T) -> ClientResult<File> {
    debug!("Opening File: {:?}", s.as_ref());
    let mut f = File::open(s)?;
    f.seek(SeekFrom::End(0))?;
    Ok(f)
}

#[cfg(test)]
mod tests {
    const RUN_LINES: &[u8] = include_bytes!("../test_runs/ok_run.txt");
    use super::*;
    #[test]
    fn test_wait_for_start_run() {
        let log_line_generator = DefaultLogLineGenerator::from_reader(RUN_LINES);
        let mut event_iter = get_race_iter(log_line_generator);

        if let Ok(_) = wait_for_start_of_run(&mut event_iter) {

        } else {
            assert!(false)
        }
    }
}

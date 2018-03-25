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

mod clientlog;
mod logline_generator;
mod race_event;
mod client_error;
mod race_run;

use chrono::Local;
use chrono::DateTime;
use race_event::SimpleEvent;
use client_error::ClientResult;
use std::path::Path;
use log::LevelFilter;
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom};

use clientlog::ClientLogLine;
use client_error::ClientError;
use logline_generator::{DefaultLogLineGenerator, LogLineGenerator};
use race_run::RaceRun;

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


    let mut v = Vec::new();

    while let Some(item) = event_iter.next() {
        let item = item?;
        if let (_, SimpleEvent::EndRun) = item {
            v.push(item);
            break;
        } else {
            v.push(item);
        }
    }

    let run = RaceRun::from_iter(v.into_iter())?;

    println!("{:#?}", run);
    let mut save_file = OpenOptions::new().write(true).create(true).append(true).open("./save")?;
    serde_json::to_writer_pretty(&mut save_file, &run)?;
    Ok(())
}

fn get_race_iter<I: Iterator<Item=std::io::Result<String>>>(i: I) -> impl Iterator<Item=ClientResult<(DateTime<Local>, SimpleEvent)>> {
    i.map(|line_result| -> ClientResult<(DateTime<Local>,SimpleEvent)> {
            let line = line_result?;
            let cll: ClientLogLine = line.parse()?;
            let event: SimpleEvent = cll.message.parse()?;
            Ok((cll.date, event))
        }).filter_map(|event_result| {
            match event_result {
                Err(ClientError::EventParseError) => None,
                item @ _ => Some(item)
            }
        }).skip_while(|event_result| {
            match event_result {
                &Ok((_, SimpleEvent::StartRun)) => false,
                _ => true
            }
        })
}

fn get_file_seeked_to_end<T: AsRef<Path>>(s: T) -> ClientResult<File> {
    debug!("Opening File: {:?}", s.as_ref());
    let mut f = File::open(s)?;
    f.seek(SeekFrom::End(0))?;
    Ok(f)
}

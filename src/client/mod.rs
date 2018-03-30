pub mod client_error;
pub mod clientlog;
pub mod race_event;
pub mod logline_generator;

use common::race_run::ZoneEntry;
use common::race_run::LevelUp;
use client::clientlog::ClientLogLine;
use self::race_event::SimpleEvent;
use chrono::Local;
use chrono::DateTime;
use self::client_error::{ClientResult, ClientError};
use std::io;


pub type EventTime = (DateTime<Local>, SimpleEvent);

pub fn get_race_iter<I: Iterator<Item = io::Result<String>>>(
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

pub fn wait_for_start_of_run<T: Iterator<Item = ClientResult<EventTime>>>(
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

pub fn fill_vec_and_return_end_time<T: Iterator<Item = ClientResult<EventTime>>>(
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

pub fn get_level_ups(start: DateTime<Local>, v: &[EventTime]) -> Vec<LevelUp> {
    v.iter().filter_map(|event| {
        match event { 
            &(time, SimpleEvent::LevelUp(level)) => {
                Some(LevelUp::new(level, time.signed_duration_since(start).num_seconds() as u64))
            }
            _ => None
            }
    }).collect()
}

pub fn get_zone_entries(start: DateTime<Local>, v: &[EventTime]) -> Vec<ZoneEntry> {
    v.iter().filter_map(|event| {
        match event { 
            &(time, SimpleEvent::EnterZone(ref name)) => {
                Some(ZoneEntry::new(name.clone(), time.signed_duration_since(start).num_seconds() as u64))
            }
            _ => None
            }
    }).collect()
}

#[cfg(test)]
mod tests {
    const RUN_LINES: &[u8] = include_bytes!("../../test_runs/ok_run.txt");
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
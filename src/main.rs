extern crate chrono;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;

mod clientlog;
mod logline_generator;
mod race_event;

use std::path::PathBuf;
use log::LevelFilter;
use race_event::Event;
use race_event::EventType;
use std::fs::File;
use std::io::{Seek, SeekFrom};

use clientlog::ClientLogLine;
use logline_generator::{DefaultLogLineGenerator, LogLineGenerator};

const CLIENT_TXT: &str =
    "C:\\Program Files (x86)\\Grinding Gear Games\\Path of Exile\\logs\\Client.txt";

type ClientResult<T> = Result<T, ClientError>;

#[derive(Debug, Fail)]
enum ClientError {
    #[fail(display = "IoError: {}", err)]
    IoError{err: std::io::Error},
    #[fail(display = "StringError: {}", message)]
    StringError{message: String}
}

impl From<String> for ClientError{
    fn from(message: String) -> Self {
        ClientError::StringError{message}
    }
}

impl From<std::io::Error> for ClientError{
    fn from(err: std::io::Error) -> Self {
        ClientError::IoError{err}
    }
}

fn main() {
    let mut builder = env_logger::Builder::from_default_env();
    builder.filter(None, LevelFilter::Debug).init();
    info!("Starting!");
    run().unwrap();
}

fn run() -> ClientResult<()> {
    let file = get_file_seeked_to_end(CLIENT_TXT.into()).map_err(|e| format!("{:?}", e))?;
    let log_line_generator = DefaultLogLineGenerator::from_reader(file);

    let mut events = log_line_generator.filter_map(|line_result| {
        if let Ok(line) = line_result {
            if let Ok(cll) = line.parse::<ClientLogLine>() {
                Some(Event::from(cll))
            } else { None }
        } else { None }
    }).filter(|event| match event {
            &Event{event_type: EventType::Other(_), ..} => false,
            _ => true
    });



    while let Some(cll) = events.next() {
        let race_event: Event = cll.into();
        println!("{:#?}", race_event);
    }

    Ok(())
}

fn get_file_seeked_to_end(s: PathBuf) -> ClientResult<File> {
    debug!("Opening File: {:?}", s);
    let mut f = File::open(s)?;
    f.seek(SeekFrom::End(0))?;
    Ok(f)
}


// #[test]
// fn test_work_file() {
//     const LINES: &str = r#"2018/03/22 22:44:20 536347953 b7d [DEBUG Client 8672] Got Instance Details from login server
//                            2018/03/22 22:44:20 536347968 b9a [INFO Client 8672] Just before calling client instance session
//                            2018/03/22 22:44:20 536347968 d9 [INFO Client 8672] Connecting to instance server at 159.122.142.230:6112
//                            2018/03/22 22:44:20 536348031 161 [DEBUG Client 8672] Connect time to instance server was 47ms
//                            2018/03/22 22:44:21 536348765 80f [DEBUG Client 8672] Joined guild named Kiwi! with 41 members
//                            2018/03/22 22:44:21 536348921 9b2 [INFO Client 8672] : You have entered Lioneye's Watch.
//                            2018/03/22 22:44:21 536348937 d8b [DEBUG Client 8672] Entering area 1_1_town"#;
//     let res = work_file(LINES.as_bytes(), Local.datetime_from_str("2018/03/22 22:44:19", "%Y/%m/%d %H:%M:%S").unwrap());
//     assert!(res.is_ok());
//     assert_eq!(res.unwrap().len(), 7);
// }

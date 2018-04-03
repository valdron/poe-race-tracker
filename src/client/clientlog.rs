use std::str::FromStr;

use chrono::prelude::*;
use regex::Regex;

const REGEX_STRING: &str = r"(?P<date>[\d/]{10} [\d:]{8}) \d+ (?P<id>\w+) \[(?P<log_string>[^\[]+)\] :? ?(?P<message>.*)";
const TIME_PARSE_STRING: &str = "%Y/%m/%d %H:%M:%S";

#[derive(Debug, PartialEq)]
pub struct ClientLogLine {
    pub date: DateTime<Local>,
    pub message: String,
}

impl FromStr for ClientLogLine {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static!{
            static ref RE: Regex = Regex::new(REGEX_STRING).unwrap();
        }
        if let Some(caps) = RE.captures(s.trim()) {
            let date = Local
                .datetime_from_str(caps.name("date").unwrap().as_str(), TIME_PARSE_STRING)
                .map_err(|e| format!("{:?}", e))?;
            Ok(Self {
                date: date,
                message: caps.name("message").unwrap().as_str().into(),
            })
        } else {
            Err(format!("Regex didn't match for line. {}", s))
        }
    }
}

#[test]
fn test_regex() {
    const LINE: &str = "2018/03/22 22:44:20 536347968 d9 [INFO Client 8672] Connecting to instance server at 159.122.142.230:6112";
    let re = Regex::new(REGEX_STRING);
    assert!(re.is_ok());
    let re = re.unwrap();
    let caps = re.captures(LINE);
    assert!(caps.is_some());
    let caps = caps.unwrap();
    let date_string = caps.name("date").unwrap().as_str();
    let message = caps.name("message").unwrap().as_str();

    assert_eq!("2018/03/22 22:44:20", date_string);
    assert_eq!(
        "Connecting to instance server at 159.122.142.230:6112",
        message
    );
}

#[test]
fn test_date_parse() {
    const DATE_STRING: &str = "2018-03-22T22:44:20Z";
    let _: DateTime<Local> = DATE_STRING.parse().unwrap();
}

#[test]
fn test_client_log_line_from_str() {
    const LINE: &str = "2018/03/22 22:44:20 536347968 d9 [INFO Client 8672] Connecting to instance server at 159.122.142.230:6112";

    let cll: Result<ClientLogLine, _> = LINE.parse();
    let cll = cll.unwrap();

    let expected_cll = ClientLogLine {
        date: Local
            .datetime_from_str("2018/03/22 22:44:20", TIME_PARSE_STRING)
            .unwrap(),
        message: "Connecting to instance server at 159.122.142.230:6112".into(),
    };
    assert_eq!(cll, expected_cll);
}

use super::client_error::ClientError;
use std::str::FromStr;
use regex::Regex;

const AREA_ENTERED_REGEX_STRING: &str = r"You have entered (?P<zone>.+)\.";
const LEVELED_UP_REGEX_STRING: &str =
    r"(?P<name>[^(]+) \((?P<class>\w+)\) is now level (?P<level>\d{1,2})";

const START_RUN_COMMAND: &str = "RT__start__";
const END_RUN_COMMAND: &str = "RT__end__";

lazy_static!{
static ref AREA_ENTERED_REGEX: Regex = Regex::new(AREA_ENTERED_REGEX_STRING).unwrap();
static ref LEVELED_UP_REGEX: Regex = Regex::new(LEVELED_UP_REGEX_STRING).unwrap();
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SimpleEvent {
    StartRun,
    EndRun,
    LevelUp(u8),
    EnterZone(String),
    LogIn,
}

pub struct EventParseError;

impl FromStr for SimpleEvent {
    type Err = ClientError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::SimpleEvent::*;
        match s {
            ref s if AREA_ENTERED_REGEX.is_match(s) => {
                let captures = AREA_ENTERED_REGEX.captures(s).unwrap();
                Ok(EnterZone(captures.name("zone").unwrap().as_str().into()))
            }
            ref s if LEVELED_UP_REGEX.is_match(s) => {
                let captures = LEVELED_UP_REGEX.captures(s).unwrap();
                let level = captures.name("level").unwrap().as_str().parse()?;
                Ok(LevelUp(level))
            }
            ref s if s.starts_with("Connected to ") => Ok(LogIn),
            ref s if s.contains(START_RUN_COMMAND) => Ok(StartRun),
            ref s if s.contains(END_RUN_COMMAND) => Ok(EndRun),
            _ => Err(EventParseError.into()),
        }
    }
}

#[test]
fn test_parse() {
    const START: &str = "<KIWIZ> askdjgfhaksjndf: RT__start__";
    const END: &str = "<KIWIZ> askdjgfhaksjndf: RT__end__";
    const ZONE: &str = "You have entered The Coast.";
    const LOGIN: &str = "Connected to mil2.login.pathofexile.com in 47ms.";
    const LEVELUP: &str = "yolsduhfasljdfasdf (Shadow) is now level 2";
    const ERROR: &str = "Connecting to instance server at 159.122.142.233:6112";

    let mut event: SimpleEvent;

    event = START.parse().unwrap();
    assert_eq!(event, SimpleEvent::StartRun);

    event = END.parse().unwrap();
    assert_eq!(event, SimpleEvent::EndRun);

    event = ZONE.parse().unwrap();
    assert_eq!(event, SimpleEvent::EnterZone("The Coast".into()));

    event = LOGIN.parse().unwrap();
    assert_eq!(event, SimpleEvent::LogIn);

    event = LEVELUP.parse().unwrap();
    assert_eq!(event, SimpleEvent::LevelUp(2));

    assert!(ERROR.parse::<SimpleEvent>().is_err());
}

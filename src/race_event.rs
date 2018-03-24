use clientlog::ClientLogLine;
use regex::Regex;
use chrono::Local;
use chrono::DateTime;

const AREA_ENTERED_REGEX_STRING: &str = r"You have entered (?P<zone>.+).";
const LEVELED_UP_REGEX_STRING: &str =
    r"(?P<name>[^(]+) \((?P<class>\w+)\) is now level (?P<level>\d{1,2})";

lazy_static!{
static ref AREA_ENTERED_REGEX: Regex = Regex::new(AREA_ENTERED_REGEX_STRING).unwrap();
static ref LEVELED_UP_REGEX: Regex = Regex::new(LEVELED_UP_REGEX_STRING).unwrap();
}

#[derive(Debug)]
pub struct Event {
    pub timestamp: DateTime<Local>,
    pub event_type: EventType,
}

#[derive(Debug, PartialEq)]
pub enum EventType {
    EnteredArea(String),
    LoggedIn,
    LeveledUp {
        level: u8,
        name: String,
        class: String,
    },
    Other(String),
}

impl From<ClientLogLine> for Event {
 fn from(other: ClientLogLine) -> Event {
     Self {
         timestamp: other.date,
         event_type: other.message.into()
     }
 }
} 
impl From<String> for EventType {
    fn from(s: String) -> Self {
        use self::EventType::*;
        match s {
            ref s if AREA_ENTERED_REGEX.is_match(s) => {
                let captures = AREA_ENTERED_REGEX.captures(s).unwrap();
                EnteredArea(captures.name("zone").unwrap().as_str().into())
            }
            ref s if LEVELED_UP_REGEX.is_match(s) => {
                let captures = LEVELED_UP_REGEX.captures(s).unwrap();
                LeveledUp {
                    level: captures.name("level").unwrap().as_str().parse().unwrap(),
                    class: captures.name("class").unwrap().as_str().into(),
                    name: captures.name("name").unwrap().as_str().into(),
                }
            }
            ref s if s.starts_with("Connected to ") => LoggedIn,
            _ => Other(s),
        }
    }
}

#[test]
fn test_from_string_event_type() {
    const OTHER1: &str = "Building Uncached Shader 8afbe50306fc06f32c35d5f58917e566bd85466d048efdc037ea55e6b8f94c67 Resampler_OutColor_Copy_Nearest_Ms_16x. Profile ps_4_0 and entrypoint Resample with macros:  OUT_COLOR=1 COPY=1 NEAREST_FILTER=1 MULTISAMPLING=1 SAMPLE_COUNT=16";
    const LOGIN: &str = "Connected to mil.login.pathofexile.com in 47ms.";
    const OTHER2: &str = "Got Instance Details from login server";
    const OTHER3: &str = "Just before calling client instance session";
    const AREA: &str = "You have entered The Twilight Strand.";
    const LEVELUP: &str = "yolsduhfasljdfasdf (Shadow) is now level 2";

    let other: EventType = OTHER1.to_owned().into();
    assert_eq!(other, EventType::Other(OTHER1.to_owned()));
    let other: EventType = OTHER2.to_owned().into();
    assert_eq!(other, EventType::Other(OTHER2.to_owned()));
    let other: EventType = OTHER3.to_owned().into();
    assert_eq!(other, EventType::Other(OTHER3.to_owned()));

    let area: EventType = AREA.to_owned().into();
    assert_eq!(area, EventType::EnteredArea("The Twilight Strand".into()));

    let login: EventType = LOGIN.to_owned().into();
    assert_eq!(login, EventType::LoggedIn);

    let level: EventType = LEVELUP.to_owned().into();
    assert_eq!(
        level,
        EventType::LeveledUp {
            name: "yolsduhfasljdfasdf".into(),
            class: "Shadow".into(),
            level: 2,
        }
    );
}

use client_error::ClientResult;
use chrono::Local;
use chrono::DateTime;
use race_event::SimpleEvent;
use std::iter::DoubleEndedIterator;

#[derive(Serialize, Deserialize, Debug)]
pub struct RaceRun {
    start: DateTime<Local>,
    end: DateTime<Local>,
    events: Vec<(DateTime<Local>, SimpleEvent)>,
}

impl RaceRun {
    pub fn from_iter<I: DoubleEndedIterator<Item=(DateTime<Local>, SimpleEvent)>>(mut iter: I) -> ClientResult<Self> {
        let (start, _) = iter.next().ok_or("empty iterator".to_owned())?;
        let (end, _ ) = iter.next_back().ok_or("empty iterator".to_owned())?;
        let events = iter.filter(|&(_, ref event)| {
            match event {
                &SimpleEvent::EnterZone(_) => true,
                &SimpleEvent::LevelUp(_) => true,
                &SimpleEvent::LogIn => true,
                _ => false
            }
        }).collect();

        Ok(Self {
            start,
            end,
            events
        })
    }
}

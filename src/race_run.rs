use chrono::Local;
use chrono::DateTime;
use race_event::SimpleEvent;

#[derive(Serialize, Deserialize, Debug)]
pub struct RaceRun {
    start: DateTime<Local>,
    end: DateTime<Local>,
    events: Vec<(DateTime<Local>, SimpleEvent)>,
}

impl RaceRun {
    pub fn new(
        start: DateTime<Local>,
        end: DateTime<Local>,
        events: Vec<(DateTime<Local>, SimpleEvent)>,
    ) -> Self {
        Self { start, end, events }
    }
}

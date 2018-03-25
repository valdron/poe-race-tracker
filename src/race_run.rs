use race_event::EventType;
use ClientResult;
use chrono::Local;
use chrono::DateTime;

#[derive(Clone)]
pub enum SimpleEvent {
    StartRun,
    EndRun,
    LevelUp(u8),
    EnterZone(String),
    LogIn,
}

impl SimpleEvent {
    pub fn from_event_type(e_type: EventType) -> Option<Self> {
        match e_type {
            EventType::EnteredArea(s) => Some(SimpleEvent::EnterZone(s)),
            EventType::LeveledUp { level, .. } => Some(SimpleEvent::LevelUp(level)),
            EventType::LoggedIn => Some(SimpleEvent::LogIn),
            EventType::Other(ref s) if s.contains("_start_") => Some(SimpleEvent::StartRun),
            EventType::Other(ref s) if s.contains("_end_") => Some(SimpleEvent::EndRun),
            _ => None,
        }
    }
}

pub struct RaceRun {
    inner: InnerRaceRun,
}

impl RaceRun {
    pub fn new() -> Self {
        Self {
            inner: InnerRaceRun::NotStarted(NotStarted),
        }
    }

    pub fn process_event(
        &mut self,
        date_time: DateTime<Local>,
        event: SimpleEvent,
    ) -> ClientResult<()> {
        match event {
            SimpleEvent::StartRun => {
                if let InnerRaceRun::NotStarted(state) = self.inner.clone() {
                    self.inner = InnerRaceRun::Started(state.start(date_time));
                    Ok(())
                } else {
                    Err("Tried to start run but was already started"
                        .to_owned()
                        .into())
                }
            }
            SimpleEvent::EndRun => {
                if let InnerRaceRun::Started(state) = self.inner.clone() {
                    self.inner = InnerRaceRun::Finished(state.finish(date_time));
                    Ok(())
                } else {
                    Err("Tried to end run but run wasn't running".to_owned().into())
                }
            }
            e @ SimpleEvent::LevelUp(_)
            | e @ SimpleEvent::EnterZone(_)
            | e @ SimpleEvent::LogIn => {
                if let InnerRaceRun::Started(ref mut state) = self.inner {
                    state.add_event(date_time, e);
                    Ok(())
                } else {
                    Err("Tried to add event to run but run wasn't running"
                        .to_owned()
                        .into())
                }
            }
        }
    }

    fn get_run(&mut self) -> ClientResult<Finished> {
        match self.inner.clone() {
            InnerRaceRun::Finished(f) => Ok(f),
            _ => Err("Run is not finished yet".to_owned().into()),
        }
    }
}

#[derive(Clone)]
enum InnerRaceRun {
    NotStarted(NotStarted),
    Started(Started),
    Finished(Finished),
}

#[derive(Clone)]
struct NotStarted;

impl NotStarted {
    fn start(self, start: DateTime<Local>) -> Started {
        Started {
            start,
            events: vec![],
        }
    }
}

#[derive(Clone)]
struct Started {
    start: DateTime<Local>,
    events: Vec<(DateTime<Local>, SimpleEvent)>,
}

impl Started {
    fn finish(self, end: DateTime<Local>) -> Finished {
        Finished {
            start: self.start,
            end,
            events: self.events,
        }
    }

    fn add_event(&mut self, date_time: DateTime<Local>, event: SimpleEvent) {
        self.events.push((date_time, event));
    }
}

#[derive(Clone)]
struct Finished {
    start: DateTime<Local>,
    end: DateTime<Local>,
    events: Vec<(DateTime<Local>, SimpleEvent)>,
}

struct RaceRunState<T> {
    state: T,
}

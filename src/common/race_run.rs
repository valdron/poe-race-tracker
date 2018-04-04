#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ZoneEntry {
    pub name: String,
    pub seconds_after_start: u64,
}

impl ZoneEntry {
    pub fn new(name: String, seconds_after_start: u64) -> Self {
        Self {
            name,
            seconds_after_start,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LevelUp {
    pub level: u8,
    pub seconds_after_start: u64,
}
impl LevelUp {
    pub fn new(level: u8, seconds_after_start: u64) -> Self {
        Self {
            level,
            seconds_after_start,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NewRaceRun {
    pub duration_in_seconds: u64,
    pub zones: Vec<ZoneEntry>,
    pub levels: Vec<LevelUp>,
}

impl NewRaceRun {
    pub fn new(duration_in_seconds: u64, levels: Vec<LevelUp>, zones: Vec<ZoneEntry>) -> Self {
        Self {
            duration_in_seconds,
            levels,
            zones,
        }
    }
}

#[test]
fn test_deserialize() {
    use serde_json::from_reader;
    const BRUTUS_RUN_JSON: &[u8] = include_bytes!("../../test_runs/brutusrun.json");

    let res: Result<NewRaceRun, _> = from_reader(BRUTUS_RUN_JSON);
    assert!(res.is_ok());
}

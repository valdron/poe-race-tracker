use common::race_run::NewRaceRun;

pub trait RunStore {
    type Error;
    type Key;
    fn create_racerun(&self, new_run: &NewRaceRun) -> Result<Self::Key, Self::Error>;
    fn get_racerun(&self, id: &Self::Key) -> Result<Option<NewRaceRun>, Self::Error>;
}

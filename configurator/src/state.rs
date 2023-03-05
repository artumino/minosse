use common::ProcessRuleSet;
use std::path::{PathBuf, Path};

#[derive(Debug, Default)]
pub(crate) struct State {
    pub rule_set: ProcessRuleSet,
    pub dirty: bool,
    pub saving: bool
}

#[derive(Debug, Default, Clone)]
pub(crate) struct SavedState {
    rule_set: ProcessRuleSet
}


#[derive(Debug, Clone)]
pub(crate) struct SaveError;

#[derive(Debug, Clone)]
pub(crate) enum LoadError {
    File,
    Format
}

impl SavedState {
    fn get_path() -> anyhow::Result<std::path::PathBuf> {
        let mut path = std::env::current_dir()?;
        path.push("rules.json");

        if path.exists() {
            return Ok(path);
        }

        path.pop();
        path.pop();
        path.push("rules.json");

        if path.exists() {
            return Ok(path);
        }

        anyhow::bail!("Could not find rules.json")
    }

    pub async fn load() -> Result<Self, LoadError> {
        let save_file = Self::get_path().ok()
            .and_then(|path| std::fs::File::open(path).ok());

        let save_file = match save_file {
            Some(file) => file,
            None => return Err(LoadError::File)
        };

        let reader = std::io::BufReader::new(save_file);
        match serde_json::from_reader(reader) {
            Ok(rule_set) => Ok(Self {
                rule_set
            }),
            Err(_) => Err(LoadError::Format)
        }
    }

    pub async fn save(self) -> Result<(), SaveError> {
        let save_file = Self::get_path().ok()
                                        .unwrap_or(Path::new("rules.json").to_path_buf());

        let writer = match std::fs::File::create(save_file).ok()
                                                .map(std::io::BufWriter::new) {
            Some(writer) => writer,
            None => return Err(SaveError)
        };

        if serde_json::to_writer_pretty(writer, &self.rule_set).is_err() {
            return Err(SaveError);
        }
        
        Ok(())
    }
}

impl <'a> From<&'a mut State> for SavedState {
    fn from(state: &'a mut State) -> Self {
        Self {
            rule_set: state.rule_set.clone()
        }
    }
}

impl From<SavedState> for State {
    fn from(saved_state: SavedState) -> Self {
        State {
            rule_set: saved_state.rule_set,
            ..Default::default()
        }
    }
}
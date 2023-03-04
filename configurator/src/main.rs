use std::path::{PathBuf, Path};

use common::ProcessRuleSet;
use iced::{window, Settings, Application, Theme, Command};

enum MinosseConfigurator {
    Loading,
    Loaded(State)
}

#[derive(Debug, Default)]
struct State {
    rule_set: ProcessRuleSet,
    dirty: bool,
    saving: bool
}

#[derive(Debug, Default, Clone)]
struct SavedState {
    rule_set: ProcessRuleSet
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<SavedState, LoadError>),
    Saved(Result<(), SaveError>),
    ToggleFullscreen(window::Mode),
}


#[derive(Debug, Clone)]
struct SaveError;

#[derive(Debug, Clone)]
enum LoadError {
    File,
    Format
}

pub fn main() -> iced::Result {
    MinosseConfigurator::run(Settings {
        window: window::Settings {
            size: (500, 800),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

impl Application for MinosseConfigurator {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, iced::Command<Message>) {
        (
            MinosseConfigurator::Loading,
            Command::perform(SavedState::load(), Message::Loaded)
        )
    }

    fn title(&self) -> String {
        "Test Title".into()
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match self {
            MinosseConfigurator::Loading => {
                match message {
                    Message::Loaded(Ok(state)) => {
                        *self = MinosseConfigurator::Loaded(State {
                            rule_set: state.rule_set,
                            ..Default::default()
                        });
                    },
                    Message::Loaded(Err(_)) => {
                        *self = MinosseConfigurator::Loaded(Default::default());
                    },
                    _ => {}
                }

                Command::none()
            },
            MinosseConfigurator::Loaded(state) => {
                let mut saved = false;
                let command = match message {
                    Message::Saved(_) => {
                        state.saving = false;
                        saved = true;

                        Command::none()
                    },
                    Message::ToggleFullscreen(mode) => {
                        window::change_mode(mode)
                    },
                    _ => Command::none()
                };

                if !saved {
                    state.dirty = true;
                }

                let save = if state.dirty && !state.saving {
                    state.dirty = false;
                    state.saving = true;

                    let save_state: SavedState = state.into();
                    Command::perform(
                        save_state.save(),
                        Message::Saved,
                    )
                } else {
                    Command::none()
                };

                Command::batch(vec![command, save])
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        todo!()
    }
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

    async fn load() -> Result<Self, LoadError> {
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

    async fn save(self) -> Result<(), SaveError> {
        let save_file = Self::get_path().ok()
                                        .unwrap_or(Path::new("rules.json").to_path_buf());

        let file = std::fs::File::create(save_file).unwrap();
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self.rule_set).unwrap();
        
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
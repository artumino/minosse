use std::path::{PathBuf, Path};

use common::ProcessRuleSet;
use iced::{window, Settings, Application, Theme, Command, Length};
use iced::widget::{
    self, button, checkbox, column, container, row, scrollable, text,
    text_input, Text,
};
use iced::alignment::{self, Alignment};

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
    Saved(Result<(), SaveError>)
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
        "Minosse Configurator".into()
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

    fn view(&self) -> iced::Element<Message> {
        match self {
            Self::Loading => self.view_loading(),
            Self::Loaded(state) => self.view_loaded(state)
        }
    }
}

impl MinosseConfigurator {
    fn view_loading(&self) -> iced::Element<Message> {
        container(
            text("Loading saved rules...")
                        .horizontal_alignment(alignment::Horizontal::Center)
                        .size(50),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y()
        .center_x()
        .into()
    }
    
    fn view_loaded(&self, state: &State) -> iced::Element<Message> {
        container(
            text(format!("Loaded {} rules", state.rule_set.rules.len()))
                        .horizontal_alignment(alignment::Horizontal::Center)
                        .size(50),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y()
        .center_x()
        .into()
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
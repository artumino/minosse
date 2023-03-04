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
enum LoadError {}

#[derive(Debug, Clone)]
enum SaveError {}

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

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            MinosseConfigurator::Loading,
            Command::perform(SavedState::load(), Message::Loaded)
        )
    }

    fn title(&self) -> String {
        todo!()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        todo!()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        todo!()
    }
}

impl SavedState {
    async fn load() -> Result<Self, LoadError> {
        todo!()
    }

    async fn save(&self) -> Result<(), SaveError> {
        todo!()
    }
}
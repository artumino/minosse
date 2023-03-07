use iced::{window, Settings, Application, Theme, Command, Length};
use iced::widget::{
    self, button, checkbox, column, container, row, scrollable, text,
    text_input, Text,
};
use iced::alignment::{self, Alignment};
use crate::state::*;
use crate::rule::*;

mod state;
mod rule;

enum MinosseConfigurator {
    Loading,
    Loaded(State)
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<SavedState, LoadError>),
    RuleWidgetMessage(usize, RuleWidgetMessage),
    Saved(Result<(), SaveError>)
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
                        *self = MinosseConfigurator::Loaded(state.into());
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
                    Message::RuleWidgetMessage(i, rule_widget_message) => {
                        if let Some(widget) = state.rule_set.get_mut(i) {
                            widget.update(rule_widget_message);   
                        }
                        
                        //TODO: Check Outer Messages
                        
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
            Self::Loaded(State {
                rule_set,
                ..
            }) => self.view_loaded(rule_set)
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
    
    fn view_loaded<'a>(&'a self, rule_set: &'a Vec<RuleWidget>) -> iced::Element<Message> {
        let rule_widgets: iced::Element<_> = column(
            rule_set.iter()
            .enumerate()
            .map(|(idx, rule_widget)| 
                rule_widget.view(idx).map(move |message|
                    Message::RuleWidgetMessage(idx, message)
                )
            )
            .collect()
        )
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .spacing(10)
        .padding(10)
        .into();
          
        let header = text(format!("Loaded {} rules", rule_set.len()))
                        .horizontal_alignment(alignment::Horizontal::Center)
                        .width(Length::Fill)
                        .size(50);
        let content = column![header, rule_widgets]
                .spacing(20);
                
        scrollable(
            container(content)
            .width(Length::Fill)
            .center_x()
        )
        .into()
    }
}
use iced::{window, Settings, Application, Theme, Command, Length, Element, theme};
use iced::widget::{
    self, button, checkbox, column, container, row, scrollable, text,
    text_input, Text,
};
use iced::alignment::{self, Alignment};
use common::{ProcessRule, ProcessPriority};


#[derive(Debug, Clone, Default)]
pub(crate) struct RuleWidget {
    pub process_rule: ProcessRule,
    pending_rule: Option<ProcessRule>
}

#[derive(Debug, Clone)]
pub(crate) enum RuleWidgetMessage {
    Edit,
    AcceptEdit,
    RegexChanged(String),
    PriorityChanged(Option<ProcessPriority>),
    AffinityChanged(Option<Vec<usize>>),
    Undo,
    Duplicate,
    Delete
}

impl RuleWidget {
    fn new(rule: ProcessRule) -> Self {
        Self {
            process_rule: rule,
            pending_rule: None
        }
    }
    
    fn update(&mut self, message: RuleWidgetMessage) {
        match message {
            RuleWidgetMessage::Edit => self.pending_rule = Some(self.process_rule.clone()),
            RuleWidgetMessage::Undo => self.pending_rule = None,
            RuleWidgetMessage::AcceptEdit => {
                if self.pending_rule.is_some() {
                    self.process_rule = self.pending_rule.take().unwrap();
                }
                self.pending_rule = None;
            },
            RuleWidgetMessage::RegexChanged(regex) => {
                if let Some(pending) = self.pending_rule.as_mut() {
                    pending.pattern = regex;
                }
            },
            RuleWidgetMessage::PriorityChanged(priority) => {
                if let Some(pending) = self.pending_rule.as_mut() {
                    pending.priority = priority;
                }
            },
            RuleWidgetMessage::AffinityChanged(affinity) => {
                if let Some(pending) = self.pending_rule.as_mut() {
                    pending.core_affinity = affinity;
                }
            },
            RuleWidgetMessage::Duplicate => {},
            RuleWidgetMessage::Delete => {}
        }
    }
    
    fn view(&self, i: usize) -> Element<RuleWidgetMessage> {
        let is_editing = self.pending_rule.is_some();
        let model = self.pending_rule.as_ref().unwrap_or(&self.process_rule);
        let header_text = text(format!("Rule #{}", i));
        
        if is_editing {
            row![
                header_text,
                text_input("Process Rule (e.g. '[Pp]rocess\\.exe')",
                            model.pattern.as_str(),
                            RuleWidgetMessage::RegexChanged)
                .on_submit(RuleWidgetMessage::AcceptEdit)
                .padding(10)
            ].spacing(20)
             .align_items(Alignment::Center)
             .into()
        } else {
            row![
                header_text,
                button(text("Edit"))
                    .on_press(RuleWidgetMessage::Edit)
                    .style(theme::Button::Text)
            ].spacing(20)
             .align_items(Alignment::Center)
             .into()
        }
    }
}
use iced::{window, Settings, Application, Theme, Command, Length, Element, theme};
use iced::widget::{
    self, button, checkbox, column, container, row, scrollable, text,
    text_input, Text, Checkbox,
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
    pub fn new(rule: ProcessRule) -> Self {
        Self {
            process_rule: rule,
            pending_rule: None
        }
    }
    
    pub fn update(&mut self, message: RuleWidgetMessage) {
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
    
    fn compute_new_affinity_vector(&self, idx: usize, checked: bool) -> Option<Vec<usize>> {
        let model = self.pending_rule.as_ref().unwrap_or(&self.process_rule);
        let max_processors = sys_info::cpu_num().unwrap() as usize;
        let mut affinity = vec![false; max_processors];
        
        for affinity_core in model.core_affinity.as_deref().unwrap_or(&[]) {
            affinity[*affinity_core] = true;
        }
        
        affinity[idx] = checked;
        
        let affinity_vector: Vec<_> = affinity.into_iter()
            .enumerate()
            .filter(|(_,value)| *value)
            .map(|(idx,_)| idx)
            .collect();
        if affinity_vector.is_empty() {
            None
        } else {
            Some(affinity_vector)
        }
    }
    
    pub fn view(&self, i: usize) -> Element<RuleWidgetMessage> {
        let is_editing = self.pending_rule.is_some();
        let model = self.pending_rule.as_ref().unwrap_or(&self.process_rule);
        let header_text = text(format!("Rule #{}", i))
            .width(Length::Fill)
            .size(24);
            
        let max_processors = sys_info::cpu_num().unwrap() as usize;
        let mut affinity = vec![false; max_processors];
        
        for affinity_core in model.core_affinity.as_deref().unwrap_or(&[]) {
            affinity[*affinity_core] = true;
        }
        
        let affinity = affinity.as_slice();
        let processor_affinity_boxes: Vec<Element<_>> = (0..max_processors).into_iter()
            .map(|idx| checkbox(format!("{}", idx), 
                                       affinity[idx], 
                                       move |checked| RuleWidgetMessage::AffinityChanged(self.compute_new_affinity_vector(idx, checked)))
                                    .spacing(2)
                                    .width(Length::Fill)
                                    .into()
            )
            .collect();
        
        let affinity_row = row(processor_affinity_boxes)
            .width(Length::Fill)
            .align_items(alignment::Alignment::Center);
            
        if is_editing {
            let header_row = row![
                header_text,
                button(text("Undo"))
                    .on_press(RuleWidgetMessage::Undo)
                    .style(theme::Button::Text)
                    .width(Length::Shrink),
                button(text("Save"))
                    .on_press(RuleWidgetMessage::AcceptEdit)
                    .style(theme::Button::Text)
                    .width(Length::Shrink)
            ];
            
            column![
                header_row,
                text_input("Process Rule (e.g. '[Pp]rocess\\.exe')",
                            model.pattern.as_str(),
                            RuleWidgetMessage::RegexChanged)
                .on_submit(RuleWidgetMessage::AcceptEdit),
                affinity_row
            ].spacing(10)
             .align_items(Alignment::Center)
             .into()
        } else {
            column![
                row![
                    header_text,
                    button(text("Edit"))
                        .on_press(RuleWidgetMessage::Edit)
                        .style(theme::Button::Text)
                        .width(Length::Shrink)
                ],
                text(model.pattern.as_str())
                    .width(Length::Fill),
                affinity_row
            ].spacing(10)
             .align_items(Alignment::Center)
             .into()
        }
    }
}
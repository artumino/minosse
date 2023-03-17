use common::{ProcessRule, ProcessPriority};
use dioxus::prelude::*;

use strum::IntoEnumIterator;
use crate::process_affinity::process_affinity;

#[derive(Props)]
pub struct SavedRule<'a> {
    pub index: usize,
    pub rule: &'a ProcessRule,
    pub on_change: EventHandler<'a, (usize, ProcessRule)>,
    pub on_delete: EventHandler<'a, usize>
}

pub fn rule_instance<'a>(cx: Scope<'a, SavedRule>) -> Element<'a> {
    let is_editing = use_state(cx, || false);
    let rule = cx.props.rule;
    let pending_rule = use_state(cx, || rule.clone());
    cx.render(
        if *is_editing.get() {
            rsx! {
                div {
                    input {
                        value: "{pending_rule.pattern}",
                        oninput: move |e| {
                            pending_rule.make_mut().pattern = e.value.clone();
                        }
                    },
                    button {
                        onclick: move |_| {
                            is_editing.set(false);
                            cx.props.on_change.call((cx.props.index, pending_rule.get().to_owned()));
                        },
                        "Save"
                    },
                    button {
                        onclick: move |_| {
                            is_editing.set(false);
                        },
                        "Cancel"
                    }
                },
                select {
                    onchange: move |e| {
                        pending_rule.make_mut().priority = match e.value.as_str() {
                            "None" => None,
                            "Low" => Some(ProcessPriority::Low),
                            "BelowNormal" => Some(ProcessPriority::BelowNormal),
                            "Normal" => Some(ProcessPriority::Normal),
                            "AboveNormal" => Some(ProcessPriority::AboveNormal),
                            "High" => Some(ProcessPriority::High),
                            "Realtime" => Some(ProcessPriority::Realtime),
                            _ => None
                        };
                    },
                    option {
                        value: "None",
                        "None"
                    },
                    for priority in ProcessPriority::iter() {
                        option {
                            value: "{priority}",
                            "{priority}"
                        }
                    }
                },
                process_affinity {
                    affinity: &pending_rule.core_affinity,
                    allow_change: true,
                    on_change: move |change: Option<Vec<usize>>| {
                        pending_rule.make_mut().core_affinity = change;
                    }
                }
            }
        } else {
            rsx! {
                div {
                    span { "{rule.pattern}" },
                    button {
                        onclick: move |_| {
                            pending_rule.set(rule.clone());
                            is_editing.set(true);
                        },
                        "Edit"
                    },
                    button {
                        onclick: move |_| {
                            cx.props.on_delete.call(cx.props.index);
                        },
                        "Delete"
                    }
                },
                select {
                    disabled: true,
                    option {
                        selected: true,
                        rule.priority.as_ref().map(|p| p.to_string()).unwrap_or("None".to_string())
                    }
                },
                process_affinity {
                    affinity: &rule.core_affinity,
                    allow_change: false,
                    on_change: move |_| {}
                }
            }
        }
    )
}
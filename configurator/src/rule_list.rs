use common::ProcessRuleSet;
use dioxus::prelude::*;

use crate::rule::rule_instance;

#[derive(Props)]
pub struct RuleList<'a> {
    pub rule_set: &'a ProcessRuleSet,
    pub on_change: EventHandler<'a, ProcessRuleSet>
}

pub fn rule_list<'a>(cx: Scope<'a, RuleList>) -> Element<'a> {
    cx.render(rsx! {
        ul {
            cx.props.rule_set.rules.iter().enumerate().map(|(idx, process_rule)| {
                rsx! {
                    rule_instance { 
                        rule: process_rule,
                        index: idx,
                        on_change: move |(idx, rule)| {
                            let mut rule_set = cx.props.rule_set.to_owned();
                            rule_set.rules[idx] = rule;
                            cx.props.on_change.call(rule_set);
                        },
                        on_delete: move |idx| {
                            let mut rule_set = cx.props.rule_set.to_owned();
                            rule_set.rules.remove(idx);
                            cx.props.on_change.call(rule_set);
                        }
                    }
                }
            })
        }
    })
}
use crate::state::SavedState;
use crate::rule_list::rule_list;
use dioxus::prelude::*;
mod state;
mod rule;
mod rule_list;
mod process_affinity;

fn app(cx: Scope) -> Element {
    let saved_state_loading = use_future(cx, (), |_| async move { SavedState::load().await });

    let loading_finished = saved_state_loading.value().is_some();
    let rule_set = use_state(cx, || None);
    if let (None, Some(Ok(saved_state))) = (rule_set.get(), saved_state_loading.value()) {
        rule_set.set(Some(saved_state.rule_set.clone()));
    }

    let is_dirty = use_state(cx, || false);

    let creating_file = use_state(cx, || None);
    let create_rule_file = move |_| {
        creating_file.set(Some(false));
        cx.spawn({
            let creating_file = creating_file.to_owned();
            async move {
                let save_operation = SavedState::default().save().await;
                creating_file.set(match save_operation.is_err() {
                    true => None,
                    false => Some(true),
                });
            }
        });
    };

    let save_changes = move |_| {
        let rule_set = rule_set.get().to_owned();
        let is_dirty = is_dirty.to_owned();
        cx.spawn(async move {
            if let Some(rule_set) = rule_set {
                    let save_operation = SavedState { rule_set }.save().await;
                    if save_operation.is_ok() {
                        is_dirty.set(false);
                    }
            }
        });
    };

    let is_creating_file = creating_file.get();
    if let Some(true) = is_creating_file {
        println!("Restarting");
        saved_state_loading.restart();
        creating_file.set(None);
    }

    let add_rule = move |_| {
        let mut rule_set = rule_set.make_mut();
        if let Some(rule_set) = rule_set.as_mut() {
            rule_set.rules.push(Default::default());
            is_dirty.set(true);
        }
    };

    cx.render(match (loading_finished, rule_set.get()) {
        (false, None) => {
            rsx!(h1 {"Loading..."})
        }
        (true, Some(rules)) => {
            rsx! {
                h1 {
                    "align": "center",
                    "Loaded"
                },
                rule_list { 
                    rule_set: rules,
                    on_change: move |changed_rule_set| {
                        rule_set.set(Some(changed_rule_set));
                        is_dirty.set(true);
                    } 
                },
                button { 
                    onclick: add_rule,
                    "Add" 
                },
                if *is_dirty.get() {
                    rsx! {
                        button { onclick: save_changes, "Save" }
                    }
                }
            }
        }
        _ => {
            rsx! { p {
                "align": "center",
                 h1 { "color": "grey",
                     "No rules file available" },
                 if is_creating_file.is_none() {
                     rsx!{
                         button { onclick: create_rule_file,
                                  "Create" }
                     }
                 }
            }}
        }
    })
}

fn main() {
    #[cfg(debug_assertions)]
    hot_reload_init!();

    dioxus_desktop::launch(app);
}

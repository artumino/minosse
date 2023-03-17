use dioxus::prelude::*;

#[derive(Props)]
pub struct ProcessAffinity<'a> {
    pub affinity: &'a Option<Vec<usize>>,
    pub allow_change: bool,
    pub on_change: EventHandler<'a, Option<Vec<usize>>>
}

impl <'a> ProcessAffinity<'a> {
    fn compute_new_affinity_vector(&self, idx: usize, checked: bool) -> Option<Vec<usize>> {
        let max_processors = sys_info::cpu_num().unwrap() as usize;
        let mut affinity = vec![self.affinity.is_none(); max_processors];
    
        if let Some(affinity_vec) = self.affinity {
            for affinity_core in affinity_vec {
                affinity[*affinity_core] = true;
            }
        }
    
        affinity[idx] = checked;
    
        let affinity_vector: Vec<_> = affinity.into_iter()
            .enumerate()
            .filter(|(_,value)| *value)
            .map(|(idx,_)| idx)
            .collect();

        match affinity_vector.len() {
            0 => Some(vec![0]),
            n if n == max_processors => None,
            _ => Some(affinity_vector)
        }
    }
}

pub fn process_affinity<'a>(cx: Scope<'a, ProcessAffinity>) -> Element<'a> {
    let affinity_vec = cx.props.affinity;
    let max_processors = sys_info::cpu_num().unwrap() as usize;

    let affinity = use_state(cx, move || {
        let mut affinity = vec![affinity_vec.is_none(); max_processors];
        
        if let Some(affinity_vec) = affinity_vec {
            for affinity_core in affinity_vec {
                affinity[*affinity_core] = true;
            }


            if affinity_vec.is_empty() {
                affinity[0] = true;
            }
        }

        affinity
    });

    
    cx.render(rsx! {
        div {
            for idx in (0..max_processors) {
                rsx! {
                    span {
                        if cx.props.allow_change {
                            rsx!{
                                input {
                                    //class: "toggle",
                                    r#type: "checkbox",
                                    checked: "{affinity[idx]}",
                                    oninput: move |_| {
                                        let affinity_vec = cx.props.compute_new_affinity_vector(idx, !affinity[idx]);
                                        cx.props.on_change.call(affinity_vec);
                                    }
                                }
                            }
                        } else {
                            rsx! {
                                input {
                                    //class: "toggle",
                                    r#type: "checkbox",
                                    checked: "{affinity[idx]}",
                                    disabled: true
                                }
                            }
                        },
                        "{idx}"
                    }
                }
            }
        }
    })
}

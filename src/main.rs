use dioxus::{events::KeyCode, prelude::*};

mod_use::mod_use![api, provider];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Panel {
    Side,
    Chat,
}

impl Panel {
    pub fn flip(self) -> Self {
        match self {
            Panel::Side => Panel::Chat,
            Panel::Chat => Panel::Side,
        }
    }
}

#[inline_props]
fn Title<'a>(cx: Scope, title: &'a str) -> Element {
    cx.render(rsx! {
        h1 { color: "black", "{title}" }
    })
}

fn main() {
    dioxus_tui::launch(|cx| -> Option<VNode> {
        let mut count = use_state(&cx, || 0);
        let focusing = use_state(&cx, || Panel::Side);

        cx.render(rsx! {
            div {
                display: "flex",
                flex_direction: "column",
                width: "100%",
                height: "100%",
                color: "black",
                onkeydown: move |e| {
                    match e.data.key_code {
                        KeyCode::LeftArrow => {
                            if focusing.get() == &Panel::Chat {
                                focusing.set(Panel::Side);
                            }
                        }
                        KeyCode::RightArrow => {
                            if focusing.get() == &Panel::Side {
                                focusing.set(Panel::Chat);
                            }
                        }
                        _ => {
                            count += 1
                        }
                    }
                },
                Title {
                    title: "Title!",
                },
                div {
                    flex_grow: "1",
                    div {
                        width: "30%",
                        height: "100%",
                        color: "black",
                        justify_content: "center",
                        border_style: "solid",
                        border_color: format_args!("{}", if focusing.get() == &Panel::Side {"cyan"} else {"black"}),
                        align_items: "center",
                        onclick: move |_| {
                            focusing.set(Panel::Side);
                        },
                        "Hello world: {count}!"
                    },
                    div {
                        width: "70%",
                        height: "100%",
                        color: "black",
                        justify_content: "center",
                        border_style: "solid",
                        border_color: format_args!("{}", if focusing.get() == &Panel::Chat {"cyan"} else {"black"}),
                        align_items: "center",
                        onclick: move |_| focusing.set(Panel::Chat),
                        "Hello world!"
                    }
                }
            }
        })
    });
}

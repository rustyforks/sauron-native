use log::*;
use sauron_native::{
    event::{on, InputEvent},
    widget::{attribute::*, *},
    Attribute, Callback, Component, Event, Node, Program, Value,
};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

pub struct App {
    click_count: u32,
    text: String,
    events: Vec<String>,
    debug: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    Click,
    ChangeText(String),
    Decrement,
}

impl App {
    pub fn new(count: u32) -> App {
        App {
            click_count: count,
            #[cfg(feature = "with-tui")]
            text: String::from("CTRL-C to exit"),
            #[cfg(not(feature = "with-tui"))]
            text: String::from("Some text"),
            events: vec![],
            debug: vec![],
        }
    }
}

impl Component<Msg> for App {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Click => self.click_count += 1,
            Msg::Decrement => self.click_count -= 1,
            Msg::ChangeText(txt) => {
                self.text = txt;
            }
        }
    }

    fn on_event(&mut self, event: Event) {
        //self.events.push(format!("{:?}", event));
    }

    fn debug(&mut self, s: String) {
        //self.debug.push(s);
    }

    fn view(&self) -> Node<Msg> {
        column(
            vec![],
            vec![
                column(
                    vec![],
                    vec![
                        text(&self.debug.join("\n")),
                        button(vec![on_click(|_| Msg::Decrement), label(&self.text)]),
                    ],
                ),
                button(vec![
                    on_click(|_| Msg::Click),
                    label(format!("Hello: {}", self.click_count)),
                ]),
                checkbox(vec![label("Checkbox1"), value(true)]),
                checkbox(vec![label("Checkbox2"), value(true)]),
                checkbox(vec![label("Checkbox3"), value(true)]),
                radio(vec![label("Radio1"), value(true)]),
                radio(vec![label("Radio2"), value(false)]),
                column(vec![], {
                    (0..self.click_count)
                        .map(|x| button(vec![label("Hello".to_string())]))
                        .collect()
                }),
                text_input(vec![
                    value(self.events.join("\n")),
                    on_input(|event: Event| match event {
                        Event::InputEvent(input) => Msg::ChangeText(input.value),
                        _ => {
                            trace!("This is unexpected: {:#?}", event);
                            panic!();
                        }
                    }),
                ]),
                image(include_bytes!("../horse.jpg").to_vec()),
                text(
                    "Hello, will this be a paragraph
The quick brown fox jumps over the lazy
dog. Lorem ipsum",
                ),
            ],
        )
    }
}

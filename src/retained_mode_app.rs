// #[cfg(feature = "retained-mode")] // This file is compiled only for `retained-mode`

use crate::common::{Tab, UserInfo};

use iced::{
    widget::{button, Button, Column, Container, Row, Rule, Text, TextInput},
    Element, Length, Sandbox, Settings,
};

use rand::{thread_rng, Rng};

pub fn run() -> iced::Result {
    TableApp::run(Settings::default())
}

#[derive(Default)]
pub struct RetainedModeApp {
    #[allow(unused)]
    active_tab: Tab,

    #[allow(unused)]
    saved_user_info: Option<UserInfo>,
}

// TODO move to common
pub const CHANNELS_COUNT: usize = 9;

// TODO rename into smth logical after understand what's example for
#[derive(Default)]
struct TableApp {
    prev_value: String,
    prev_suspicious: String,
    prev_channel: String,
    current_value: String,
    current_suspicious: String,
    current_channel: String,
    current_channel_number: usize,
    channel_data: [(String, String, String); 9],
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(usize, String),
    ButtonPressed(usize),
    ChangeChannel(i32),
}

impl Sandbox for TableApp {
    type Message = Message;

    fn new() -> Self {
        TableApp {
            channel_data: Default::default(),
            ..Self::default()
        }
    }

    fn title(&self) -> String {
        String::from("Table App with Channels") // TODO rename based on struct renaming
    }

    fn update(&mut self, message: Message) {
        match message {
            // TODO absolutely requires refactoring, generated with chatGPT
            Message::InputChanged(index, value) => match index {
                0 => self.prev_value = value,
                1 => self.prev_suspicious = value,
                2 => self.prev_channel = value,
                3 => self.current_value = value,
                4 => self.current_suspicious = value,
                5 => self.current_channel = value,
                _ => {}
            },
            Message::ButtonPressed(index) => {
                // TODO carry out on initialization, the same AI misunderstanding
                let mut rng = thread_rng();
                self.current_value = rng.gen_range(1..100).to_string();
                self.current_suspicious = "false".to_string();
                self.current_channel = index.to_string();
                self.current_channel_number = index - 1;
                self.channel_data[self.current_channel_number] = (
                    self.current_value.clone(),
                    self.current_suspicious.clone(),
                    self.current_channel.clone(),
                );
            }
            Message::ChangeChannel(change) => {
                // Update previous values with current values
                let (prev_value, prev_suspicious, prev_channel) =
                    &self.channel_data[self.current_channel_number];
                self.prev_value = prev_value.clone();
                self.prev_suspicious = prev_suspicious.clone();
                self.prev_channel = prev_channel.clone();

                let new_channel_index =
                    ((self.current_channel_number as i32 + change).rem_euclid(9)) as usize;
                self.current_channel_number = new_channel_index;

                let (value, suspicious, channel) = &self.channel_data[self.current_channel_number];
                self.current_value = value.clone();
                self.current_suspicious = suspicious.clone();
                self.current_channel = channel.clone();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let table = Row::new()
            .spacing(20)
            .push(
                Column::new()
                    .width(Length::FillPortion(2)) // Ensure equal width
                    .spacing(10)
                    .push(Text::new("Value"))
                    .push(
                        TextInput::new("Previous Value", &self.prev_value)
                            .on_input(move |v| Message::InputChanged(0, v))
                            .padding(10),
                    )
                    .push(
                        TextInput::new("Current Value", &self.current_value)
                            .on_input(move |v| Message::InputChanged(3, v))
                            .padding(10),
                    ),
            )
            .push(
                Column::new()
                    .width(Length::FillPortion(2)) // Ensure equal width
                    .spacing(10)
                    .push(Text::new("Suspicious"))
                    .push(
                        TextInput::new("Suspicious?", &self.prev_suspicious)
                            .on_input(move |v| Message::InputChanged(1, v))
                            .padding(10),
                    )
                    .push(
                        TextInput::new("Suspicious?", &self.current_suspicious)
                            .on_input(move |v| Message::InputChanged(4, v))
                            .padding(10),
                    ),
            )
            .push(
                Column::new()
                    .width(Length::FillPortion(2)) // Ensure equal width
                    .spacing(10)
                    .push(Text::new("Channel"))
                    .push(
                        TextInput::new("Channel", &self.prev_channel)
                            .on_input(move |v| Message::InputChanged(2, v))
                            .padding(10),
                    )
                    .push(
                        TextInput::new("Channel", &self.current_channel)
                            .on_input(move |v| Message::InputChanged(5, v))
                            .padding(10),
                    ),
            )
            .push(
                Column::new()
                    .width(Length::FillPortion(1)) // Ensure equal width
                    .spacing(10)
                    .push(Text::new("Action"))
                    .push(Button::new(Text::new("Action")))
                    .push(Button::new(Text::new("Action"))),
            );

        let separator = Rule::horizontal(20);

        let mut buttons_row = Row::new().spacing(10);
        for i in 0..CHANNELS_COUNT {
            let label = (i + 1).to_string();
            let button = Button::new(Text::new(label))
                .on_press(Message::ButtonPressed(i + 1))
                .padding(if self.current_channel_number == i {
                    20
                } else {
                    10
                });
            buttons_row = buttons_row.push(button);
        }

        // dummies for now
        let wider_buttons = Row::new()
            .spacing(20)
            .push(Button::new(Text::new("Wide Button 1")))
            .push(Button::new(Text::new("Wide Button 2")))
            .push(Button::new(Text::new("Wide Button 3")));

        let arrows = Row::new()
            .spacing(10)
            .push(Button::new(Text::new("<")).on_press(Message::ChangeChannel(-1)))
            .push(Button::new(Text::new(">")).on_press(Message::ChangeChannel(1)));

        let content = Column::new()
            .align_items(iced::Alignment::Center)
            .spacing(20)
            .padding(100)
            .push(table)
            .push(separator)
            .push(buttons_row)
            .push(wider_buttons)
            .push(arrows);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

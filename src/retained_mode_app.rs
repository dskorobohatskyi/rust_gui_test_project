// #[cfg(feature = "retained-mode")] // This file is compiled only for `retained-mode`

// Useful links:
// https://iced.rs/

use crate::common::{Tab, UserInfo};

use iced::{
    widget::{button, Button, Column, Container, Row, Rule, Space, Text, TextInput},
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
pub const INVALID_CHANNEL_INDEX: usize = usize::MAX;

// TODO move to common?
#[derive(Default)]
pub struct ChannelInfo {
    integer_value: u32,
    is_suspicious: bool,
}

struct ChannelInfoUIAdapter {}

impl ChannelInfoUIAdapter {
    fn get_value_as_text(info: &ChannelInfo) -> String {
        info.integer_value.to_string()
    }

    fn get_suspicious_as_text(info: &ChannelInfo) -> String {
        format!("{}", if info.is_suspicious { "Yes" } else { "No" })
    }
}

// TODOs:
// Add some tabs
// Play with stretching the window

// TODO rename into smth logical after understand what's example for
#[derive(Default)]
struct TableApp {
    prev_value: String, // TODO need to set as text? then add _text suffix
    prev_suspicious: String,
    prev_channel: String,
    current_value: String,
    current_suspicious: String,
    current_channel: String,

    // previous_channel_number: usize, // Looks unuseful, but let's see during implementation
    current_channel_number: usize,
    channel_data: [ChannelInfo; CHANNELS_COUNT],
}

#[derive(Debug, Clone, PartialEq)]
enum ChannelDataRow {
    Previous,
    Current,
}

#[derive(Debug, Clone)]
enum Message {
    IgnoreInput,                        // used at least for TextInput's to be 'read-only', but still can copy the values
    ButtonPressed(usize),
    ChangeChannel(i32),
    ClearChannelRow(ChannelDataRow),
}

impl TableApp {
    fn init_data(&mut self) {
        // Initialization of channel infos
        for i in 0..CHANNELS_COUNT {
            let mut rng = thread_rng();
            let generated_int = rng.gen_range(1..100) as u32;
            let is_suspicious = generated_int > 75; // just for ex.
            let channel_info = ChannelInfo {
                integer_value: generated_int,
                is_suspicious,
            };
            self.channel_data[i] = channel_info;
        }
    }
}

impl Sandbox for TableApp {
    type Message = Message;

    fn new() -> Self {
        let mut app = TableApp {
            channel_data: Default::default(),
            // previous_channel_number: INVALID_CHANNEL_INDEX,
            current_channel_number: INVALID_CHANNEL_INDEX,
            ..Self::default()
        };

        // TODO it might be separated button, Initialize
        app.init_data();
        app.update(Message::ButtonPressed(1));

        app
    }

    fn title(&self) -> String {
        String::from("Table App with Channels") // TODO rename based on struct renaming
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IgnoreInput => {}
            Message::ButtonPressed(index) => {
                if self.current_channel_number != INVALID_CHANNEL_INDEX {
                    let channel_info = &self.channel_data[self.current_channel_number];
                    self.prev_value = ChannelInfoUIAdapter::get_value_as_text(channel_info);
                    self.prev_suspicious =
                        ChannelInfoUIAdapter::get_suspicious_as_text(channel_info);
                    self.prev_channel = (self.current_channel_number + 1).to_string();

                    // self.previous_channel_number = self.current_channel_number;
                }

                self.current_channel_number = index - 1;

                let channel_info = &self.channel_data[self.current_channel_number];
                self.current_value = ChannelInfoUIAdapter::get_value_as_text(channel_info);
                self.current_suspicious =
                    ChannelInfoUIAdapter::get_suspicious_as_text(channel_info);
                self.current_channel = index.to_string();
            }
            Message::ChangeChannel(change) => {
                assert!(self.current_channel_number != INVALID_CHANNEL_INDEX);

                // Update previous values with current values
                let channel_info = &self.channel_data[self.current_channel_number];
                self.prev_value = ChannelInfoUIAdapter::get_value_as_text(channel_info);
                self.prev_suspicious = ChannelInfoUIAdapter::get_suspicious_as_text(channel_info);
                self.prev_channel = (self.current_channel_number + 1).to_string();

                // self.previous_channel_number = self.current_channel_number;
                let new_channel_index =
                    ((self.current_channel_number as i32 + change).rem_euclid(9)) as usize;
                self.current_channel_number = new_channel_index;

                let channel_info_ = &self.channel_data[self.current_channel_number];
                self.current_value = ChannelInfoUIAdapter::get_value_as_text(channel_info_);
                self.current_suspicious =
                    ChannelInfoUIAdapter::get_suspicious_as_text(channel_info_);
                self.current_channel = (self.current_channel_number + 1).to_string();
            }
            Message::ClearChannelRow(selected_row) => {
                if selected_row == ChannelDataRow::Previous {
                    self.prev_value = String::new();
                    self.prev_suspicious = String::new();
                    self.prev_channel = String::new();

                    // self.previous_channel_number = INVALID_CHANNEL_INDEX;
                } else if selected_row == ChannelDataRow::Current {
                    self.current_value = String::new();
                    self.current_suspicious = String::new();
                    self.current_channel = String::new();

                    self.current_channel_number = INVALID_CHANNEL_INDEX;
                } else {
                    panic!("Unexpected ChannelDataRow value!!")
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let table = Row::new()
            .spacing(5)
            .push(
                Column::new()
                    .width(Length::FillPortion(1)) // Ensure equal width
                    .spacing(10)
                    .align_items(iced::Alignment::End)
                    .push(Space::with_height(Length::FillPortion(1)))
                    .push(Text::new("Previous:").height(Length::FillPortion(2)))
                    .push(Text::new("Current:").height(Length::FillPortion(2))),
            )
            .push(
                Column::new()
                    .width(Length::FillPortion(2)) // Ensure equal width
                    .spacing(10)
                    .align_items(iced::Alignment::Center)
                    .push(Text::new("Value").height(Length::FillPortion(1)))
                    .push(
                        Container::new(
                            TextInput::new("Previous Value", &self.prev_value)
                                .on_input(move |_| Message::IgnoreInput) // to be in 'enabled' state
                        )
                        .height(Length::FillPortion(2))
                        .width(Length::Fill),
                    )
                    .push(
                        Container::new(
                            TextInput::new("Current Value", &self.current_value)
                            .on_input(move |_| Message::IgnoreInput) // to be in 'enabled' state
                        )
                        .height(Length::FillPortion(2))
                        .width(Length::Fill),
                    ),
            )
            .push(
                Column::new()
                    .width(Length::FillPortion(2)) // Ensure equal width
                    .spacing(10)
                    .align_items(iced::Alignment::Center)
                    .push(Text::new("Suspicious").height(Length::FillPortion(1)))
                    .push(
                        Container::new(
                            TextInput::new("Suspicious?", &self.prev_suspicious)
                            .on_input(move |_| Message::IgnoreInput) // to be in 'enabled' state
                        )
                        .height(Length::FillPortion(2))
                        .width(Length::Fill),
                    )
                    .push(
                        Container::new(
                            TextInput::new("Suspicious?", &self.current_suspicious)
                            .on_input(move |_| Message::IgnoreInput) // to be in 'enabled' state
                        )
                        .height(Length::FillPortion(2))
                        .width(Length::Fill),
                    ),
            )
            .push(
                Column::new()
                    .width(Length::FillPortion(2)) // Ensure equal width
                    .spacing(10)
                    .align_items(iced::Alignment::Center)
                    .push(Text::new("Channel").height(Length::FillPortion(1)))
                    .push(
                        Container::new(
                            TextInput::new("Channel", &self.prev_channel)
                            .on_input(move |_| Message::IgnoreInput) // to be in 'enabled' state
                        )
                        .height(Length::FillPortion(2))
                        .width(Length::Fill),
                    )
                    .push(
                        Container::new(
                            TextInput::new("Channel", &self.current_channel)
                            .on_input(move |_| Message::IgnoreInput) // to be in 'enabled' state
                        )
                        .height(Length::FillPortion(2))
                        .width(Length::Fill),
                    ),
            )
            .push(
                Column::new()
                    .width(Length::FillPortion(2)) // Ensure equal width
                    .spacing(10)
                    .align_items(iced::Alignment::Center)
                    .push(Text::new("Actions").height(Length::FillPortion(1)))
                    .push(
                        Button::new(Text::new("Clear Previous"))
                            .on_press(Message::ClearChannelRow(ChannelDataRow::Previous))
                            .height(Length::FillPortion(1)),
                    )
                    .push(Space::with_height(Length::FillPortion(1)))
                    .push(
                        Button::new(Text::new("Clear Current"))
                            .on_press(Message::ClearChannelRow(ChannelDataRow::Current))
                            .height(Length::FillPortion(1)),
                    )
                    .push(Space::with_height(Length::FillPortion(1))),
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
            .spacing(10)
            .push(Button::new(Text::new("Wide Button 1")))
            .push(Button::new(Text::new("Wide Button 2")))
            .push(Button::new(Text::new("Wide Button 3")));

        let arrows = Row::new()
            .spacing(10)
            .push(Button::new(Text::new("<")).on_press(Message::ChangeChannel(-1)))
            .push(Button::new(Text::new(">")).on_press(Message::ChangeChannel(1)));

        let content = Column::new()
            .align_items(iced::Alignment::Center)
            .spacing(10)
            .padding(80)
            .push(table.height(Length::FillPortion(2)))
            .push(separator)
            .push(buttons_row.height(Length::FillPortion(1)))
            .push(arrows.height(Length::FillPortion(1)))
            .push(wider_buttons.height(Length::FillPortion(1)));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

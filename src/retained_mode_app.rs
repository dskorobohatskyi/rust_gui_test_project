// #[cfg(feature = "retained-mode")] // This file is compiled only for `retained-mode`

// Useful links:
// https://iced.rs/

use crate::common::{
    ChannelInfo, Tab, BACKUP_CHANNEL_INDEX, CHANNELS_COUNT, INVALID_CHANNEL_INDEX,
};

use iced::{
    widget::{button, Button, Column, Container, Row, Rule, Space, Text, TextInput},
    Element, Length, Sandbox, Settings,
};

use rand::{thread_rng, Rng};

pub fn run() -> iced::Result {
    ChannelBasedApp::run(Settings::default())
}

#[derive(Default)]
pub struct RetainedModeApp {
    #[allow(unused)]
    active_tab: Tab,
}

struct ChannelInfoUIAdapter {}

impl ChannelInfoUIAdapter {
    fn get_value_as_text(info: &ChannelInfo) -> String {
        info.integer_value.to_string()
    }

    fn get_suspicious_as_text(info: &ChannelInfo) -> String {
        (if info.is_suspicious { "Yes" } else { "No" }).to_string()
    }
}

// TODOs:
// Add some tabs
// Play with stretching the window

#[derive(Default)]
struct ChannelBasedApp {
    prev_value_text: String,
    prev_suspicious_text: String,
    prev_channel_text: String,
    current_value_text: String,
    current_suspicious_text: String,
    current_channel_text: String,

    // previous_channel_index: usize, // Looks unuseful, but let's see during implementation
    current_channel_index: usize,
    channel_data: [ChannelInfo; CHANNELS_COUNT],
}

#[derive(Debug, Clone, PartialEq)]
enum ChannelDataRow {
    Previous,
    Current,
}

#[derive(Debug, Clone)]
enum Message {
    IgnoreInput, // used at least for TextInput's to be 'read-only', but still can copy the values
    ButtonPressed(usize),
    ChangeChannel(i32),
    ClearChannelRow(ChannelDataRow),
}

impl ChannelBasedApp {
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

impl Sandbox for ChannelBasedApp {
    type Message = Message;

    fn new() -> Self {
        let mut app = ChannelBasedApp {
            channel_data: Default::default(),
            // previous_channel_index: INVALID_CHANNEL_INDEX,
            current_channel_index: INVALID_CHANNEL_INDEX,
            ..Self::default()
        };

        // TODO it might be separated button, Initialize
        app.init_data();
        app.update(Message::ButtonPressed(BACKUP_CHANNEL_INDEX + 1));

        app
    }

    fn title(&self) -> String {
        String::from("Some App with Channels")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IgnoreInput => {}
            Message::ButtonPressed(index) => {
                if self.current_channel_index != INVALID_CHANNEL_INDEX {
                    let channel_info = &self.channel_data[self.current_channel_index];
                    self.prev_value_text = ChannelInfoUIAdapter::get_value_as_text(channel_info);
                    self.prev_suspicious_text =
                        ChannelInfoUIAdapter::get_suspicious_as_text(channel_info);
                    self.prev_channel_text = (self.current_channel_index + 1).to_string();

                    // self.previous_channel_index = self.current_channel_index;
                }

                self.current_channel_index = index - 1;

                let channel_info = &self.channel_data[self.current_channel_index];
                self.current_value_text = ChannelInfoUIAdapter::get_value_as_text(channel_info);
                self.current_suspicious_text =
                    ChannelInfoUIAdapter::get_suspicious_as_text(channel_info);
                self.current_channel_text = index.to_string();
            }
            Message::ChangeChannel(change) => {
                if self.current_channel_index == INVALID_CHANNEL_INDEX {
                    self.update(Message::ButtonPressed(BACKUP_CHANNEL_INDEX + 1));
                    return;
                }

                // Update previous values with current values
                let channel_info = &self.channel_data[self.current_channel_index];
                self.prev_value_text = ChannelInfoUIAdapter::get_value_as_text(channel_info);
                self.prev_suspicious_text =
                    ChannelInfoUIAdapter::get_suspicious_as_text(channel_info);
                self.prev_channel_text = (self.current_channel_index + 1).to_string();

                // self.previous_channel_index = self.current_channel_index;
                let new_channel_index =
                    ((self.current_channel_index as i32 + change).rem_euclid(9)) as usize;
                self.current_channel_index = new_channel_index;

                let channel_info_ = &self.channel_data[self.current_channel_index];
                self.current_value_text = ChannelInfoUIAdapter::get_value_as_text(channel_info_);
                self.current_suspicious_text =
                    ChannelInfoUIAdapter::get_suspicious_as_text(channel_info_);
                self.current_channel_text = (self.current_channel_index + 1).to_string();
            }
            Message::ClearChannelRow(selected_row) => {
                if selected_row == ChannelDataRow::Previous {
                    self.prev_value_text = String::new();
                    self.prev_suspicious_text = String::new();
                    self.prev_channel_text = String::new();

                    // self.previous_channel_index = INVALID_CHANNEL_INDEX;
                } else if selected_row == ChannelDataRow::Current {
                    self.current_value_text = String::new();
                    self.current_suspicious_text = String::new();
                    self.current_channel_text = String::new();

                    self.current_channel_index = INVALID_CHANNEL_INDEX;
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
                            TextInput::new("Previous Value", &self.prev_value_text)
                                .on_input(move |_| Message::IgnoreInput), // to be in 'enabled' state
                        )
                        .height(Length::FillPortion(2))
                        .width(Length::Fill),
                    )
                    .push(
                        Container::new(
                            TextInput::new("Current Value", &self.current_value_text)
                                .on_input(move |_| Message::IgnoreInput), // to be in 'enabled' state
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
                            TextInput::new("Suspicious?", &self.prev_suspicious_text)
                                .on_input(move |_| Message::IgnoreInput), // to be in 'enabled' state
                        )
                        .height(Length::FillPortion(2))
                        .width(Length::Fill),
                    )
                    .push(
                        Container::new(
                            TextInput::new("Suspicious?", &self.current_suspicious_text)
                                .on_input(move |_| Message::IgnoreInput), // to be in 'enabled' state
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
                            TextInput::new("Channel", &self.prev_channel_text)
                                .on_input(move |_| Message::IgnoreInput), // to be in 'enabled' state
                        )
                        .height(Length::FillPortion(2))
                        .width(Length::Fill),
                    )
                    .push(
                        Container::new(
                            TextInput::new("Channel", &self.current_channel_text)
                                .on_input(move |_| Message::IgnoreInput), // to be in 'enabled' state
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
                .padding(if self.current_channel_index == i {
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

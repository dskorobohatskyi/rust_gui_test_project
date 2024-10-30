// #[cfg(feature = "retained-mode")] // This file is compiled only for `retained-mode`

// Useful links:
// https://iced.rs/

use crate::common::{
    ApplicationTab, ChannelInfo, BACKUP_CHANNEL_INDEX, CHANNELS_COUNT, HIGH_INTEGER_LIMIT,
    INVALID_CHANNEL_INDEX, LOW_INTEGER_LIMIT, SUSPICIOUS_LIMIT,
};

use iced::{
    widget::{button, slider, text, text_input, Button, Column, Container, Row, Rule, Space},
    Element, Length, Sandbox, Settings,
};

use rand::{thread_rng, Rng};

pub fn run() -> iced::Result {
    ChannelBasedApp::run(Settings::default())
}

trait ChannelInfoUIExt {
    fn value_as_text(&self) -> String;
    fn suspicious_as_text(&self) -> String;
}

impl ChannelInfoUIExt for ChannelInfo {
    fn value_as_text(&self) -> String {
        self.integer_value.to_string()
    }
    fn suspicious_as_text(&self) -> String {
        (if self.is_suspicious { "Yes" } else { "No" }).to_string()
    }
}

// TODOs:
// Play with stretching the window

#[derive(Default)]
struct ChannelBasedApp {
    // TODO do we really need to keep these strings?
    prev_value_text: String,
    prev_suspicious_text: String,
    prev_channel_text: String,
    current_value_text: String,
    current_suspicious_text: String,
    current_channel_text: String,

    previous_channel_index: usize,
    current_channel_index: usize,
    channel_data: [ChannelInfo; CHANNELS_COUNT],
    current_suspicious_limit: u32,

    active_tab: ApplicationTab,
}

#[derive(Debug, Clone, PartialEq)]
enum ChannelDataRow {
    Previous,
    Current,
}

#[derive(Debug, Clone)]
enum Message {
    IgnoreInput, // used at least for TextInput's to be 'read-only', but still can copy the values
    TabSelected(ApplicationTab),
    ButtonPressed(usize),
    ChangeChannel(i32),
    ClearChannelRow(ChannelDataRow),
    ModifyingSuspiciosValue(u32),
    ReleasedSuspiciousSlider,
}

impl ChannelBasedApp {
    fn init_data(&mut self) {
        // Initialization of channel infos
        for i in 0..CHANNELS_COUNT {
            let mut rng = thread_rng();
            let generated_int = rng.gen_range(LOW_INTEGER_LIMIT..=HIGH_INTEGER_LIMIT) as u32;
            let is_suspicious = generated_int > self.current_suspicious_limit;
            let channel_info = ChannelInfo {
                integer_value: generated_int,
                is_suspicious,
            };
            self.channel_data[i] = channel_info;
        }
    }

    fn update_suspicious(&mut self) {
        for data in &mut self.channel_data {
            data.is_suspicious = data.integer_value > self.current_suspicious_limit;
        }
    }

    fn tab_button<'a>(&self, label: &'a str, tab: &ApplicationTab) -> Button<'a, Message> {
        let is_active_tab = tab == &self.active_tab;
        let button = button(text(label))
            .on_press(Message::TabSelected(tab.clone()))
            .padding(if is_active_tab { 8 } else { 10 });

        let style = if is_active_tab {
            iced::theme::Button::Primary
        } else {
            iced::theme::Button::Secondary
        };
        button.style(style)
    }
}

impl Sandbox for ChannelBasedApp {
    type Message = Message;

    fn new() -> Self {
        let mut app = ChannelBasedApp {
            previous_channel_index: INVALID_CHANNEL_INDEX,
            current_channel_index: INVALID_CHANNEL_INDEX,
            channel_data: Default::default(),
            active_tab: ApplicationTab::Home,
            current_suspicious_limit: SUSPICIOUS_LIMIT,
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
            Message::TabSelected(tab) => {
                self.active_tab = tab;
            }
            Message::ButtonPressed(index) => {
                if self.current_channel_index != INVALID_CHANNEL_INDEX {
                    let channel_info = &self.channel_data[self.current_channel_index];
                    self.prev_value_text = channel_info.value_as_text();
                    self.prev_suspicious_text = channel_info.suspicious_as_text();
                    self.prev_channel_text = (self.current_channel_index + 1).to_string();

                    self.previous_channel_index = self.current_channel_index;
                }

                self.current_channel_index = index - 1;

                let channel_info = &self.channel_data[self.current_channel_index];
                self.current_value_text = channel_info.value_as_text();
                self.current_suspicious_text = channel_info.suspicious_as_text();
                self.current_channel_text = index.to_string();
            }
            Message::ChangeChannel(change) => {
                if self.current_channel_index == INVALID_CHANNEL_INDEX {
                    self.update(Message::ButtonPressed(BACKUP_CHANNEL_INDEX + 1));
                    return;
                }

                // Update previous values with current values
                let channel_info = &self.channel_data[self.current_channel_index];
                self.prev_value_text = channel_info.value_as_text();
                self.prev_suspicious_text = channel_info.suspicious_as_text();
                self.prev_channel_text = (self.current_channel_index + 1).to_string();

                self.previous_channel_index = self.current_channel_index;
                let new_channel_index =
                    ((self.current_channel_index as i32 + change).rem_euclid(9)) as usize;
                self.current_channel_index = new_channel_index;

                let channel_info_ = &self.channel_data[self.current_channel_index];
                self.current_value_text = channel_info_.value_as_text();
                self.current_suspicious_text = channel_info_.suspicious_as_text();
                self.current_channel_text = (self.current_channel_index + 1).to_string();
            }
            Message::ClearChannelRow(selected_row) => {
                if selected_row == ChannelDataRow::Previous {
                    self.prev_value_text = String::new();
                    self.prev_suspicious_text = String::new();
                    self.prev_channel_text = String::new();

                    self.previous_channel_index = INVALID_CHANNEL_INDEX;
                } else if selected_row == ChannelDataRow::Current {
                    self.current_value_text = String::new();
                    self.current_suspicious_text = String::new();
                    self.current_channel_text = String::new();

                    self.current_channel_index = INVALID_CHANNEL_INDEX;
                } else {
                    panic!("Unexpected ChannelDataRow value!!")
                }
            }
            Message::ModifyingSuspiciosValue(new_value) => {
                self.current_suspicious_limit = new_value;
            }
            Message::ReleasedSuspiciousSlider => {
                self.update_suspicious();

                if self.previous_channel_index != INVALID_CHANNEL_INDEX {
                    self.prev_suspicious_text =
                        self.channel_data[self.previous_channel_index].suspicious_as_text();
                }
                if self.current_channel_index != INVALID_CHANNEL_INDEX {
                    self.current_suspicious_text =
                        self.channel_data[self.current_channel_index].suspicious_as_text();
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
                    .push(text("Previous:").height(Length::FillPortion(2)))
                    .push(text("Current:").height(Length::FillPortion(2))),
            )
            .push(
                Column::new()
                    .width(Length::FillPortion(2)) // Ensure equal width
                    .spacing(10)
                    .align_items(iced::Alignment::Center)
                    .push(text("Value").height(Length::FillPortion(1)))
                    .push(
                        Container::new(
                            text_input("Previous Value", &self.prev_value_text)
                                .on_input(move |_| Message::IgnoreInput), // to be in 'enabled' state
                        )
                        .height(Length::FillPortion(2))
                        .width(Length::Fill),
                    )
                    .push(
                        Container::new(
                            text_input("Current Value", &self.current_value_text)
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
                    .push(text("Suspicious").height(Length::FillPortion(1)))
                    .push(
                        Container::new(
                            text_input("Suspicious?", &self.prev_suspicious_text)
                                .on_input(move |_| Message::IgnoreInput), // to be in 'enabled' state
                        )
                        .height(Length::FillPortion(2))
                        .width(Length::Fill),
                    )
                    .push(
                        Container::new(
                            text_input("Suspicious?", &self.current_suspicious_text)
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
                    .push(text("Channel").height(Length::FillPortion(1)))
                    .push(
                        Container::new(
                            text_input("Channel", &self.prev_channel_text)
                                .on_input(move |_| Message::IgnoreInput), // to be in 'enabled' state
                        )
                        .height(Length::FillPortion(2))
                        .width(Length::Fill),
                    )
                    .push(
                        Container::new(
                            text_input("Channel", &self.current_channel_text)
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
                    .push(text("Actions").height(Length::FillPortion(1)))
                    .push(
                        button(text("Clear Previous"))
                            .on_press(Message::ClearChannelRow(ChannelDataRow::Previous))
                            .height(Length::FillPortion(1)),
                    )
                    .push(Space::with_height(Length::FillPortion(1)))
                    .push(
                        button(text("Clear Current"))
                            .on_press(Message::ClearChannelRow(ChannelDataRow::Current))
                            .height(Length::FillPortion(1)),
                    )
                    .push(Space::with_height(Length::FillPortion(1))),
            );

        let separator = Rule::horizontal(20);

        let mut buttons_row = Row::new().spacing(10);
        for i in 0..CHANNELS_COUNT {
            let label = (i + 1).to_string();
            let button = button(text(label))
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
            .push(button(text("Wide Button 1")))
            .push(button(text("Wide Button 2")));

        let suspicios_limit_slider = slider(
            LOW_INTEGER_LIMIT..=HIGH_INTEGER_LIMIT,
            self.current_suspicious_limit,
            Message::ModifyingSuspiciosValue,
        )
        .step(1)
        .on_release(Message::ReleasedSuspiciousSlider);

        let suspicious_limit_section = Row::new()
            .push(
                Column::new()
                    .push(text("Current suspicious limit:"))
                    .spacing(10),
            )
            .push(Space::with_width(10))
            .push(Column::new().push(text(self.current_suspicious_limit.to_string())))
            .push(Space::with_width(10))
            .push(Column::new().push(suspicios_limit_slider).spacing(10));

        let arrows = Row::new()
            .spacing(10)
            .push(button(text("<")).on_press(Message::ChangeChannel(-1)))
            .push(button(text(">")).on_press(Message::ChangeChannel(1)));

        let main_content = Column::new()
            .align_items(iced::Alignment::Center)
            .spacing(10)
            .padding(80)
            .push(table.height(Length::FillPortion(2)))
            .push(separator)
            .push(buttons_row.height(Length::FillPortion(1)))
            .push(arrows.height(Length::FillPortion(1)))
            .push(wider_buttons.height(Length::FillPortion(1)))
            .push(suspicious_limit_section)
            .height(Length::FillPortion(1));

        let tab_row = Row::new()
            .spacing(10)
            .width(Length::Fill) // Make the row take the full width
            .align_items(iced::Alignment::Start)
            .push(self.tab_button("Main", &ApplicationTab::Home))
            .push(self.tab_button("Dummy", &ApplicationTab::Settings))
            .push(self.tab_button("About", &ApplicationTab::About));

        let content = match self.active_tab {
            ApplicationTab::Home => main_content,
            ApplicationTab::Settings => Column::new().push(text("Dummy Tab Content")),
            ApplicationTab::About => Column::new().push(text("About Tab Content")),
        };

        Container::new(
            Column::new()
                .push(
                    Container::new(tab_row)
                        .width(Length::Fill) // Make sure the tab row takes full width
                        .align_y(iced::alignment::Vertical::Top)
                        .center_x(),
                )
                .push(content)
                .spacing(20)
                .padding(20),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

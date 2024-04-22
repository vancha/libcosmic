use chrono::{Local, NaiveDate};
use cosmic::app::{Command, Core, Settings};
use cosmic::iced_core::Size;
use cosmic::widget::{calendar, dropdown, Button, Column,Image, Row, Text};
use cosmic::{executor, ApplicationExt, Element};

struct GarbageApp {
    core: Core,
    state: State,
    selected_date: NaiveDate,
    dropdown_selected: Option<usize>,
    dropdown_options: Vec<&'static str>,
}

#[derive(Default)]
enum State {
    #[default]
    Default,
    EditMode,
}

#[derive(Clone, Debug)]
struct Reminder {
    description: String,
}

#[derive(Clone, Debug)]
enum Message {
    EditModeStart,
    EditModeStop,
    AddReminder(Reminder),
    DateSelected(NaiveDate),
    DropdownSelected(usize),
}

impl cosmic::Application for GarbageApp {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    const APP_ID: &'static str = "com.github.cosmicgarbage";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut app = GarbageApp {
            core: core,
            state: State::Default,
            selected_date: NaiveDate::from(Local::now().naive_local()),
            dropdown_selected: Some(0),
            dropdown_options: vec!["Grijze bak", "Groene bak", "Sorti bak"],
        };
        app.set_header_title(String::from("Actual garbage app"));
        (app, Command::none())
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::EditModeStart => {
                println!("Now entering edit mode");
                self.state = State::EditMode;
                cosmic::Command::none()
            }
            Message::EditModeStop => {
                println!("Now exiting edit mode");
                self.state = State::Default;
                cosmic::Command::none()
            }
            Message::AddReminder(reminder) => {
                println!("Atempting to add reminder");
                cosmic::Command::none()
            }
            Message::DateSelected(date) => {
                self.selected_date = date;
                println!("Selected date is:  {:?}", date);
                cosmic::Command::none()
            }
            Message::DropdownSelected(index) => {
                println!("Dropdown selected idx: {:?}", index);
                self.dropdown_selected = Some(index);
                cosmic::Command::none()
            }
        }
    }
    fn view(&self) -> Element<Self::Message> {
        Element::from(
            Column::with_children(vec![match self.state {
                State::Default => Column::with_children(vec![
                    Text::new("Reminders:").into(),
                    Button::new("Edit").on_press(Message::EditModeStart).into(),
                ])
                .width(cosmic::iced::Length::Fill)
                .align_items(cosmic::iced::Alignment::Center)
                .into(),
                State::EditMode => Column::with_children(vec![
                    Row::with_children(
                        vec![
                            match self.dropdown_selected {
                                Some(0) => {
                                    Image::new("grijs.png")
                                },
                                Some(1) => {
                                    Image::new("groen.png")
                                },
                                Some(2) => {
                                    Image::new("sorti.png")
                                },
                                _ => {
                                    Image::new("grijs.png")
                                }
                            }
                            .width(cosmic::iced::Length::FillPortion(1))
                            .into(),
                            dropdown(
                                &self.dropdown_options,
                                self.dropdown_selected,
                                Message::DropdownSelected,
                            )
                            .width(cosmic::iced::Length::FillPortion(1))
                            .into()
                        ]
                    ).into(),
                    calendar(&self.selected_date, |date| Message::DateSelected(date)).into(),
                    Button::new("Add reminder")
                        .on_press(Message::EditModeStop)
                        .into(),
                ])
                .width(cosmic::iced::Length::Fill)
                .align_items(cosmic::iced::Alignment::Center)
                .into(),
            }])
            .padding(cosmic::iced::Padding::new(32.))
            .align_items(cosmic::iced::Alignment::Center),
        )
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::default().size(Size::new(800., 600.));
    let flags = ();
    cosmic::app::run::<GarbageApp>(settings, flags)?;
    Ok(())
}

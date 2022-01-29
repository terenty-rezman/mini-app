use std::f32::consts::PI;

use iced::{
    button, text_input, Align, Background, Color, Column, Container, Font, Length,
    Row, Rule, Sandbox, Settings, Space, Text, TextInput, Vector,
};

const LIGHT_GRAY: [f32; 3] = [0.1, 0.1, 0.1];

/// Основные вычисления
fn try_calc_q(dp: &str, ds: &str, A: &str, freq: &str) -> Result<(f32, f32), &'static str> {
    let dp: u32 = dp.parse().or(Err("Диаметр поршня: неверное значение"))?;
    let ds: u32 = ds.parse().or(Err("Диаметр штока: неверное значение"))?;
    let a: u32 = A.parse().or(Err("Амплитуда сигнала: неверное значение"))?;
    let freq: f32 = freq.parse().or(Err("Частота сигнала: неверное значение"))?;

    // Перевод в систему СИ:
    let a_meters = a as f32 / 1000.0; // амплитуда: мм -> м
    let freq = 2.0 * PI * freq; // частота: Гц -> рад/c
    let dp_meters = dp as f32 / 1000.0; // площадь поршня: мм -> м
    let ds_meters = ds as f32 / 1000.0; // площадь штока: мм -> м

    // Площадь рабочей поверхности:
    let s_work_area = (PI / 4.0) * (dp_meters * dp_meters - ds_meters * ds_meters); // в м^2

    // Основная формула:
    let q_m3s = freq * a_meters * s_work_area; // расход в ед. СИ
    let q_litrmin = q_m3s * (1000.0 * 60.0); // расход в л/мин

    Ok((q_litrmin, q_m3s))
}

#[derive(Debug, Clone)]
enum Message {
    PistonDiameterChanged(String),
    RodDiameterChanged(String),
    AmplitudeChanged(String),
    FrequencyChanged(String),
    CalcButtonPressed,
    DoNothing,
}

struct MiniApp {
    piston_diameter: String,
    piston_widget_state: text_input::State,

    rod_diameter: String,
    rod_widget_state: text_input::State,

    amplitude: String,
    amplitude_widget_state: text_input::State,

    frequency: String,
    frequency_widget_state: text_input::State,

    main_result: String,
    main_result_widget_state: text_input::State,

    secondary_result: String,
    secondary_result_widget_state: text_input::State,

    calc_button_state: button::State,

    last_error: String,
}

impl Sandbox for MiniApp {
    type Message = Message;

    fn new() -> Self {
        MiniApp {
            piston_diameter: Default::default(),
            piston_widget_state: Default::default(),

            rod_diameter: Default::default(),
            rod_widget_state: Default::default(),

            amplitude: Default::default(),
            amplitude_widget_state: Default::default(),

            frequency: Default::default(),
            frequency_widget_state: Default::default(),

            main_result: Default::default(),
            main_result_widget_state: Default::default(),

            secondary_result: Default::default(),
            secondary_result_widget_state: Default::default(),

            calc_button_state: Default::default(),

            last_error: " ".to_string(), // workaround to not hide error_widget on gui
        }
    }

    fn title(&self) -> String {
        "Q app".to_string()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::PistonDiameterChanged(s) => {
                if s.parse::<u32>().is_ok() || s.is_empty() {
                    self.piston_diameter = s;
                }
            }

            Message::RodDiameterChanged(s) => {
                if s.parse::<u32>().is_ok() || s.is_empty() {
                    self.rod_diameter = s;
                }
            }

            Message::AmplitudeChanged(s) => {
                if s.parse::<u32>().is_ok() || s.is_empty() {
                    self.amplitude = s;
                }
            }

            Message::FrequencyChanged(s) => {
                if s.parse::<f32>().is_ok() || s.is_empty() {
                    self.frequency = s;
                }
            }

            Message::CalcButtonPressed => {
                match try_calc_q(
                    &self.piston_diameter,
                    &self.rod_diameter,
                    &self.amplitude,
                    &self.frequency,
                ) {
                    Ok((main_result, secondary_result)) => {
                        self.main_result = main_result.to_string();
                        self.secondary_result = secondary_result.to_string();
                        self.last_error = " ".to_string(); // to not hide error widget on gui
                    }
                    Err(e) => {
                        self.last_error = e.to_string();
                    }
                }
            }

            Message::DoNothing => (),
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let piston_widget = TextInput::new(
            &mut self.piston_widget_state,
            "",
            &self.piston_diameter.to_string(),
            Message::PistonDiameterChanged,
        )
        .padding(5)
        .width(Length::Units(100))
        .on_submit(Message::CalcButtonPressed);

        let rod_widget = TextInput::new(
            &mut self.rod_widget_state,
            "",
            &self.rod_diameter,
            Message::RodDiameterChanged,
        )
        .padding(5)
        .width(Length::Units(100))
        .on_submit(Message::CalcButtonPressed);

        let amplitude_widget = TextInput::new(
            &mut self.amplitude_widget_state,
            "",
            &self.amplitude,
            Message::AmplitudeChanged,
        )
        .padding(5)
        .width(Length::Units(100))
        .on_submit(Message::CalcButtonPressed);

        let frequency_widget = TextInput::new(
            &mut self.frequency_widget_state,
            "",
            &self.frequency,
            Message::FrequencyChanged,
        )
        .padding(5)
        .width(Length::Units(100))
        .on_submit(Message::CalcButtonPressed);

        let calc_button_widget =
            button::Button::new(&mut self.calc_button_state, Text::new("Вычислить"))
                .padding(10)
                .style(MyButtonStyle)
                .on_press(Message::CalcButtonPressed);

        let error_message_widget = Text::new(&self.last_error).color([1.0, 0.0, 0.0]).size(17);

        let input_layout = Column::new()
            .spacing(10)
            .align_items(Align::End)
            .push(
                Row::new()
                    .spacing(10)
                    .push(Text::new("Исходные данные").color(Color::from_rgb8(188, 195, 206)))
                    .push(Space::new(Length::Units(10), Length::Units(1))),
            )
            .push(Space::new(Length::Units(1), Length::Units(5)))
            .push(
                Row::new()
                    .spacing(10)
                    .align_items(Align::Center)
                    .push(Text::new("Диаметр поршня:").color(LIGHT_GRAY))
                    .push(piston_widget)
                    .push(Text::new("мм").color(LIGHT_GRAY).width(Length::Units(30))),
            )
            .push(
                Row::new()
                    .spacing(10)
                    .align_items(Align::Center)
                    .push(Text::new("Диаметр штока:").color(LIGHT_GRAY))
                    .push(rod_widget)
                    .push(Text::new("мм").color(LIGHT_GRAY).width(Length::Units(30))),
            )
            .push(Space::new(Length::Units(1), Length::Units(20)))
            .push(
                Row::new()
                    .spacing(10)
                    .align_items(Align::Center)
                    .push(Text::new("Амплитуда сигнала:").color(LIGHT_GRAY))
                    .push(amplitude_widget)
                    .push(Text::new("мм").color(LIGHT_GRAY).width(Length::Units(30))),
            )
            .push(
                Row::new()
                    .spacing(10)
                    .align_items(Align::Center)
                    .push(Text::new("Частота сигнала:").color(LIGHT_GRAY))
                    .push(frequency_widget)
                    .push(Text::new("Гц").color(LIGHT_GRAY).width(Length::Units(30))),
            )
            .push(Space::new(Length::Units(1), Length::Units(20)))
            .push(
                Row::new()
                    .spacing(10)
                    .align_items(Align::Center)
                    .push(calc_button_widget)
                    .push(Space::new(Length::Units(30), Length::Units(1))),
            )
            .push(Space::new(Length::Units(1), Length::Units(20)))
            .push(error_message_widget);

        let main_result_widget = TextInput::new(
            &mut self.main_result_widget_state,
            "результат",
            &self.main_result,
            // use as output only
            |_s| Message::DoNothing,
        )
        .size(30)
        .padding(0)
        .style(MyTextInputStyle)
        .width(Length::Units(200));

        let secondary_result_widget = TextInput::new(
            &mut self.secondary_result_widget_state,
            "результат",
            &self.secondary_result,
            // use as output only
            |_s| Message::DoNothing,
        )
        .size(30)
        .padding(0)
        .style(MyTextInputStyle)
        .width(Length::Units(200));

        let result_layout = Column::new()
            .spacing(10)
            .align_items(Align::Start)
            // .push(Text::new("Результат"))
            // .push(Space::new(Length::Units(1), Length::Units(5)))
            .push(Text::new("Полученный расход Q[л/мин]:").color(LIGHT_GRAY))
            .push(main_result_widget)
            .push(Space::new(Length::Units(1), Length::Units(20)))
            .push(Text::new("Полученный расход Q[м^3/с]:").color(LIGHT_GRAY))
            .push(secondary_result_widget)
            .push(Space::new(Length::Units(1), Length::Units(40)));

        let content = Row::new()
            .spacing(40)
            .padding(20)
            .align_items(Align::Center)
            .push(input_layout)
            .push(Rule::vertical(10))
            .push(result_layout)
            .height(Length::Units(500));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

struct MyButtonStyle;

impl button::StyleSheet for MyButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb8(87, 85, 217))),
            shadow_offset: Vector::new(0.0, 0.0),
            text_color: Color::WHITE,
            border_radius: 3.0,
            border_width: 0.0,
            border_color: [0.7, 0.7, 0.7].into(),
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb8(75, 72, 214))),
            ..self.active()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb8(58, 56, 210))),
            ..self.active()
        }
    }
}

struct MyTextInputStyle;

impl text_input::StyleSheet for MyTextInputStyle {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(Color::WHITE),
            border_radius: 0.0,
            border_width: 1.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
        }
    }

    fn focused(&self) -> text_input::Style {
        text_input::Style {
            border_color: Color::from_rgb(0.5, 0.5, 0.5),
            ..self.active()
        }
    }

    fn placeholder_color(&self) -> Color {
        Color::from_rgb(0.7, 0.7, 0.7)
    }

    fn value_color(&self) -> Color {
        Color::from_rgb8(87, 85, 217)
    }

    fn selection_color(&self) -> Color {
        Color::from_rgb(0.8, 0.8, 1.0)
    }
}

fn main() {
    MiniApp::run(Settings::default());
}

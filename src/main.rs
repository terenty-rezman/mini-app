use iced::{
    tooltip, button, text_input, 
    Align, Background, Color, Column, Container, Length, Row, Rule, Sandbox,
    Settings, Space, Text, TextInput, Vector,
};

const LIGHT_GRAY: [f32; 3] = [0.1, 0.1, 0.1];

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

            main_result: "123".to_string(),
            main_result_widget_state: Default::default(),

            secondary_result: "123".to_string(),
            secondary_result_widget_state: Default::default(),

            calc_button_state: Default::default(),
        }
    }

    fn title(&self) -> String {
        "Mini App".to_string()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::PistonDiameterChanged(s) => self.piston_diameter = s,
            Message::RodDiameterChanged(s) => self.rod_diameter = s,
            Message::AmplitudeChanged(s) => self.amplitude = s,
            Message::FrequencyChanged(s) => self.frequency = s,
            Message::CalcButtonPressed => (),
            Message::DoNothing => (),
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let piston_widget = TextInput::new(
            &mut self.piston_widget_state,
            "",
            &self.piston_diameter,
            Message::PistonDiameterChanged,
        )
        .padding(5)
        .width(Length::Units(100));

        let rod_widget = TextInput::new(
            &mut self.rod_widget_state,
            "",
            &self.rod_diameter,
            Message::RodDiameterChanged,
        )
        .padding(5)
        .width(Length::Units(100));

        let amplitude_widget = TextInput::new(
            &mut self.amplitude_widget_state,
            "",
            &self.amplitude,
            Message::AmplitudeChanged,
        )
        .padding(5)
        .width(Length::Units(100));

        let frequency_widget = TextInput::new(
            &mut self.frequency_widget_state,
            "",
            &self.frequency,
            Message::FrequencyChanged,
        )
        .padding(5)
        .width(Length::Units(100));

        let calc_button_widget =
            button::Button::new(&mut self.calc_button_state, Text::new("Вычислить"))
                .padding(10)
                .style(MyButtonStyle)
                .on_press(Message::CalcButtonPressed);

        let input_layout = Column::new()
            .spacing(10)
            .align_items(Align::End)
            .push(
                Row::new()
                .spacing(10)
                .push(Text::new("Исходные данные").color(Color::from_rgb8(188, 195, 206)))
                .push(Space::new(Length::Units(10), Length::Units(1)))
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
            );

        let main_result_widget = TextInput::new(
            &mut self.main_result_widget_state,
            "result",
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
            "result",
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
            .push(secondary_result_widget);

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
            border_width: 0.0,
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

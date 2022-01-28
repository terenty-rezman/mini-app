use iced::{
    text_input, Align, Column, Command, Container, Length, Row, Sandbox, Settings, Space, Text,
    TextInput, Rule
};

const LIGHT_GRAY: [f32; 3] = [0.5, 0.5, 0.5];

#[derive(Debug, Clone)]
enum Message {
    PistonDiameterChanged(String),
    RodDiameterChanged(String),
    AmplitudeChanged(String),
    FrequencyChanged(String),
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
            Message::DoNothing => (),
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let piston_widget = TextInput::new(
            &mut self.piston_widget_state,
            "",
            &self.piston_diameter,
            // use as output only
            Message::PistonDiameterChanged,
        )
        .padding(5)
        .width(Length::Units(100));

        let rod_widget = TextInput::new(
            &mut self.rod_widget_state,
            "",
            &self.rod_diameter,
            // use as output only
            Message::RodDiameterChanged,
        )
        .padding(5)
        .width(Length::Units(100));

        let amplitude_widget = TextInput::new(
            &mut self.amplitude_widget_state,
            "",
            &self.amplitude,
            // use as output only
            Message::AmplitudeChanged,
        )
        .padding(5)
        .width(Length::Units(100));

        let frequency_widget = TextInput::new(
            &mut self.frequency_widget_state,
            "",
            &self.frequency,
            // use as output only
            Message::FrequencyChanged,
        )
        .padding(5)
        .width(Length::Units(100));

        let input_layout = Column::new()
            .spacing(10)
            .align_items(Align::End)
            .push(Text::new("Исходные данные"))
            .push(Space::new(Length::Units(1), Length::Units(5)))
            .push(
                Row::new()
                    .spacing(10)
                    .align_items(Align::Center)
                    .push(Text::new("Диаметр поршня:").color(LIGHT_GRAY))
                    .push(piston_widget)
                    .push(Text::new("мм").color(LIGHT_GRAY).width(Length::Units(30)))
            )
            .push(
                Row::new()
                    .spacing(10)
                    .align_items(Align::Center)
                    .push(Text::new("Диаметр штока:").color(LIGHT_GRAY))
                    .push(rod_widget)
                    .push(Text::new("мм").color(LIGHT_GRAY).width(Length::Units(30)))
            )
            .push(Space::new(Length::Units(1), Length::Units(20)))
            .push(
                Row::new()
                    .spacing(10)
                    .align_items(Align::Center)
                    .push(Text::new("Амплитуда сигнала:").color(LIGHT_GRAY))
                    .push(amplitude_widget)
                    .push(Text::new("мм").color(LIGHT_GRAY).width(Length::Units(30)))
            )
            .push(
                Row::new()
                    .spacing(10)
                    .align_items(Align::Center)
                    .push(Text::new("Частота сигнала:").color(LIGHT_GRAY))
                    .push(frequency_widget)
                    .push(Text::new("Гц").color(LIGHT_GRAY).width(Length::Units(30)))
            );

        let main_result_widget = TextInput::new(
            &mut self.main_result_widget_state,
            "result",
            &self.main_result,
            // use as output only
            |_s| Message::DoNothing,
        )
        .size(30)
        .padding(10)
        .width(Length::Units(200));

        let secondary_result_widget = TextInput::new(
            &mut self.secondary_result_widget_state,
            "result",
            &self.secondary_result,
            // use as output only
            |_s| Message::DoNothing,
        )
        .padding(5)
        .width(Length::Units(200));

        let result_layout = Column::new()
            .spacing(10)
            .align_items(Align::Start)
            .push(Text::new("Результат"))
            .push(Space::new(Length::Units(1), Length::Units(5)))
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

fn main() {
    MiniApp::run(Settings::default());
}

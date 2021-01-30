mod custom_plot_backend;
mod models;
mod load_avg_view;

use iced::{Application, Command, Container, Element, Length, Settings, Subscription, executor};
use load_avg_view::{LoadAvgView, LoadAvgMessage};
use models::GraphView;
use sysinfo::SystemExt;

fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.antialiasing = true;

    App::run(settings)
}

struct App {
    system: sysinfo::System,
    now: std::time::Instant,
    load_avg_view: LoadAvgView
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(std::time::Instant),
    LoadAvgMessage(LoadAvgMessage)
}

impl Application for App {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            App {
                system: Self::gen_system_info(),
                now: std::time::Instant::now(),
                load_avg_view: LoadAvgView::default()
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("")
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(1000))
            .map(|instant| {
                Message::Tick(instant)
            })
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(instant) => {
                self.update_tick(&instant);
                self.update_load_avg();
            }

            Message::LoadAvgMessage(msg) => {
                self.load_avg_view.update(msg);
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let content = self.load_avg_view.view().map(|msg| {
            Message::LoadAvgMessage(msg)
        });
        
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(4)
            .into()
    }
}

impl App {
    fn gen_system_info() -> sysinfo::System {
        let mut system = sysinfo::System::new_all();
        system.refresh_all();

        system
    }

    fn update_tick(&mut self, instant: &std::time::Instant) {
        self.now = *instant;
    }

    fn update_load_avg(&mut self) {
        let load_avg = self.system.get_load_average();
        self.load_avg_view.push_load_avg(&load_avg);
        self.load_avg_view.clear_canvas_cache();
    }
}
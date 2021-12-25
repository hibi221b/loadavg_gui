pub trait GraphView {
    type Message;
    fn clear_canvas_cache(&mut self);
    fn update(&mut self, msg: Self::Message);
    fn view(&mut self) -> iced::Element<Self::Message>;
}
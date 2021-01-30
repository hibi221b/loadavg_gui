pub trait GraphView<EnumMessage> 
where 
    EnumMessage: std::fmt::Debug + Clone + Copy
{
    fn clear_canvas_cache(&mut self);
    fn update(&mut self, msg: EnumMessage);
    fn view(&mut self) -> iced::Element<EnumMessage>;
}
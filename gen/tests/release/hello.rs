use gen_macros::{Event, Props};
#[derive(Live, LiveHook, Widget)]
pub struct MyProps {
    pub label1: String,
}
#[derive(DefaultNone, Clone, Debug)]
pub enum Events {
    Clicked(String),
    None,
}

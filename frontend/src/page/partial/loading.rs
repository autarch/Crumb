use crate::page_styles;
use seed::{prelude::*, *};

#[derive(Debug)]
pub enum Msg {
    Dummy,
}

pub fn view() -> Node<Msg> {
    section![C![page_styles()], div!["Loading"]]
}

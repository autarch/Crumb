use crate::{generated::css_classes::C, page_styles};
use seed::{prelude::*, *};

#[derive(Clone, Debug)]
pub enum Msg {
    Dummy,
}

#[derive(Debug)]
pub struct Model {}

pub fn init(_url: Url, _orders: &mut impl Orders<Msg>) -> Model {
    Model {}
}

#[allow(clippy::too_many_lines)]
pub fn view(_: &Model) -> Node<Msg> {
    section![C![page_styles()], div![C![C.flex_grow], "Home page"]]
}

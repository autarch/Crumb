use crate::{
    components::{self, album_image},
    generated::css_classes::C,
    icons, page_styles, Queue, QueueItem,
};
use seed::{prelude::*, *};
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

#[derive(Clone, Debug)]
pub struct Model {
    queue: Option<Rc<RefCell<Queue>>>,
}

#[derive(Debug)]
pub enum Msg {
    QueueFetched(Rc<RefCell<Queue>>),

    ComponentsMsg(components::Msg),
    IconsMsg(icons::Msg),
    LoadingMsg(crate::page::partial::loading::Msg),
}

pub fn init(_url: Url, _orders: &mut impl Orders<Msg>) -> Model {
    // log!("queue::init()");
    Model { queue: None }
}

pub fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    // log!("queue::update()");
    match msg {
        Msg::QueueFetched(queue) => model.queue = Some(queue),
        _ => (),
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    match &model.queue {
        Some(queue) => {
            // log!("queue is Some");
            view_queue(queue.borrow())
        }
        None => {
            // log!("queue is None");
            crate::page::partial::loading::view().map_msg(Msg::LoadingMsg)
        }
    }
}

fn view_queue(queue: Ref<Queue>) -> Node<Msg> {
    // log!(format!("queue.len() = {}", queue.len()));
    match queue.is_empty() {
        true => view_empty_queue(),
        false => view_populated_queue(queue),
    }
}

fn view_empty_queue() -> Node<Msg> {
    section![C![page_styles()], div!["Queue is empty."]]
}

fn view_populated_queue(queue: Ref<Queue>) -> Node<Msg> {
    section![
        C![page_styles()],
        div![
            C![C.flex, C.flex_col],
            div![
                C![C.flex, C.flex_row, C.h_fit_in_viewport],
                view_queue_items(&queue),
                view_current_album_cover(&queue),
            ],
        ],
    ]
}

fn view_queue_items(queue: &Ref<Queue>) -> Node<Msg> {
    // If we got here we know that this will be Some, not None.
    match queue.visible_items() {
        None => Node::Empty,
        Some(items) => {
            // Items in queue
            div![
                C![
                    C.flex_grow,
                    C.flex_shrink,
                    C.flex_col,
                    C.w_full,
                    C.lg__w_2of5,
                    C.overflow_y_scroll,
                    C.scrollbar_on_hover,
                    C.mx_2,
                ],
                items
                    .enumerate()
                    .map(|(i, item)| view_queue_item(i == 0, item)),
            ]
        }
    }
}

fn view_queue_item(is_first: bool, item: QueueItem) -> Node<Msg> {
    let client = &crate::OUR_CLIENT;
    let album = client.album_by_id(&item.track.album_id).unwrap();
    let artist = client.artist_by_id(&album.artist_id).unwrap();

    div![
        C![
            C.flex,
            C.flex_row,
            C.items_center,
            C.p_1,
            C.border_t,
            C.last__border_b,
            C.border_opacity_50,
            C.border_gray_500,
            C.text_sm,
            IF!( is_first => C.bg_indigo_100 ),
        ],
        div![C![C.w_8, C.text_xl], item.position,],
        album_image(artist, album, Some(&[C.h_10, C.w_10, C.mr_4])).map_msg(Msg::ComponentsMsg),
        div![
            C![C.flex_grow],
            p![&item.track.title],
            p![
                a![attrs! { At::Href => artist.url }, &artist.name],
                " - ",
                a![attrs! { At::Href => album.url }, &album.title]
            ],
        ],
        view_queue_item_icon(
            icons::x_circle()
                .title("Remove from queue")
                .build()
                .into_svg()
        ),
        view_queue_item_icon(
            icons::dots_vertical()
                .title("More actions")
                .build()
                .into_svg()
        ),
    ]
}

fn view_queue_item_icon(icon: Node<icons::Msg>) -> Node<Msg> {
    button![C![C.ml_1_p_5], icon.map_msg(Msg::IconsMsg)]
}

fn view_current_album_cover(queue: &Ref<Queue>) -> Node<Msg> {
    match queue.has_visible_tracks() {
        true => div![
            C![C.flex_shrink, C.w_0, C.hidden, C.lg__block, C.lg__w_3of5],
            album_image(
                &queue.current_artist,
                &queue.current_album,
                Some(&[C.object_contain, C.h_fit_in_viewport, C.w_full]),
            )
            .map_msg(Msg::ComponentsMsg),
        ],
        false => Node::Empty,
    }
}

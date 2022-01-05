use crate::{
    client::{get_artist_response, Client},
    components::{self, release_image},
    generated::css_classes::C,
    icons, page_styles, view_error, Queue, QueueItem,
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
                view_current_release_cover(&queue),
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
    // This needs to be redone so data is loaded asynchronously.
    return Node::Empty;

    // let client = Client::new();
    // let release = client.get_release(&item.track.release_id).unwrap();
    // let response = client
    //     .get_artist(&release.core.as_ref().unwrap().primary_artist_id)
    //     .await;
    // let artist = match response {
    //     Ok(Some(get_artist_response::ResponseEither::Artist(a))) => a,
    //     Ok(Some(get_artist_response::ResponseEither::Error(e))) => return view_error(&e),
    //     Ok(None) => {
    //         log!("Empty response for GetArtist request!");
    //         return Node::Empty;
    //     },
    //     // XXX - need to do something with error
    //     Err(_) => return Node::Empty,
    // };

    // div![
    //     C![
    //         C.flex,
    //         C.flex_row,
    //         C.items_center,
    //         C.p_1,
    //         C.border_t,
    //         C.last__border_b,
    //         C.border_opacity_50,
    //         C.border_gray_500,
    //         C.text_sm,
    //         IF!( is_first => C.bg_indigo_100 ),
    //     ],
    //     div![C![C.w_8, C.text_xl], item.position,],
    //     release_image(
    //         response.core.as_ref().unwrap(),
    //         release.core.as_ref().unwrap(),
    //         Some(&[C.h_10, C.w_10, C.mr_4]),
    //     )
    //     .map_msg(Msg::ComponentsMsg),
    //     div![
    //         C![C.flex_grow],
    //         p![&item.track.display_title],
    //         p![
    //             a![
    //                 attrs! { At::Href => response.core.as_ref().unwrap().url() },
    //                 &response.core.as_ref().unwrap().name,
    //             ],
    //             " - ",
    //             a![
    //                 attrs! { At::Href => release.core.as_ref().unwrap().url() },
    //                 &release.core.as_ref().unwrap().display_title,
    //             ]
    //         ],
    //     ],
    //     view_queue_item_icon(
    //         icons::x_circle()
    //             .title("Remove from queue")
    //             .build()
    //             .into_svg()
    //     ),
    //     view_queue_item_icon(
    //         icons::dots_vertical()
    //             .title("More actions")
    //             .build()
    //             .into_svg()
    //     ),
    // ]
}

fn view_queue_item_icon(icon: Node<icons::Msg>) -> Node<Msg> {
    button![C![C.ml_1_p_5], icon.map_msg(Msg::IconsMsg)]
}

fn view_current_release_cover(queue: &Ref<Queue>) -> Node<Msg> {
    if !queue.has_visible_tracks() {
        return Node::Empty;
    }

    let current = queue.current.as_ref().unwrap();
    div![
        C![C.flex_shrink, C.w_0, C.hidden, C.lg__block, C.lg__w_3of5],
        release_image(
            current.artist.core.as_ref().unwrap(),
            current.release.core.as_ref().unwrap(),
            Some(&[C.object_contain, C.h_fit_in_viewport, C.w_full]),
        )
        .map_msg(Msg::ComponentsMsg),
    ]
}

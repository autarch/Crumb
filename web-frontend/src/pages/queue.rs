use crate::{
    client::QueueItem,
    components::{AlbumCover, Color, Table, Td, Tr, UnderlineLink},
    models::Queue,
    prelude::*,
    storage,
    util::{format_time, maybe_plural, new_client},
    QueueRecvResult, QueueUpdate,
};
use dioxus_heroicons::{solid::Shape, IconButton};

#[inline_props]
pub(crate) fn Queue<'a>(
    cx: Scope,
    queue: &'a Option<QueueRecvResult>,
    queue_tx: futures_channel::mpsc::Sender<QueueUpdate>,
    is_playing: &'a bool,
    set_is_playing: &'a UseState<bool>,
) -> Element {
    let content = match queue {
        Some(queue) => match queue {
            Ok(queue) => match queue {
                Ok(queue) => {
                    rsx! {
                        Tracks {
                            queue: queue,
                            queue_tx: queue_tx.clone(),
                        }
                    }
                }
                Err(e) => {
                    log::error!("Error loading queue: {e}");
                    rsx! {
                        div {
                            class: DC![C.fg.col_span_7],
                            "Error loading queue"
                        },
                    }
                }
            },
            Err(e) => {
                log::error!("Error getting message from channel: {e}");
                rsx! {
                    div {
                        class: DC![C.fg.col_span_7],
                        "Error getting message from channel"
                    },
                }
            }
        },
        None => {
            log::info!("queue is None");
            rsx! {
                div {
                    class: DC![C.fg.col_span_7],
                    "Loading queue ..."
                },
            }
        }
    };

    cx.render(content)
}

#[inline_props]
fn Tracks<'a>(
    cx: Scope,
    queue: &'a Queue,
    queue_tx: futures_channel::mpsc::Sender<QueueUpdate>,
) -> Element {
    let past_items = match queue.past_items() {
        None => None,
        Some(items) => {
            let past_tracks = maybe_plural(items.len() as u32, "past track");
            Some(rsx! {
                Tr {
                    key: "past items",
                    Td {
                        colspan: 4,
                        "{past_tracks}",
                    }
                }
            })
        }
    };
    let table_class = C![C.siz.w_3_of_5];
    let content = match queue.visible_items() {
        None => rsx! { "Queue is empty" },
        Some(items) => rsx! {
            AlbumCover {
                class: C![C.lay.fixed, C.siz.w_2_of_5, C.spc.pr_8],
                uri: items[0].release_cover_uri.as_deref(),
                round: false,
                border: false,
            },
            div {
                class: DC![C.lay.flex, C.fg.flex_row],
                div {
                    class: DC![C.siz.w_2_of_5],
                },
                Table {
                    class: "{table_class}",
                    past_items,
                    items.iter().map(|i| rsx! {
                        OneQueueItem {
                            key: "{i.queue_position}",
                            queue_tx: queue_tx.clone(),
                            item: i,
                        }
                    }),
                },
            },
        },
    };

    cx.render(rsx! {
        content
    })
}

#[inline_props]
fn OneQueueItem<'a>(
    cx: Scope,
    item: &'a QueueItem,
    queue_tx: futures_channel::mpsc::Sender<QueueUpdate>,
) -> Element {
    let track = item.release_track.as_ref().unwrap();
    let release_url = item.release_url();
    let artist_url = item.artist_url();
    let time = format_time(track.length.unwrap_or(0));
    let td_class = C![C.spc.py_2];
    let icon_class = C![C.spc.p_1, C.bg.bg_indigo_400];
    let on_delete_click = move |_| {
        to_owned![queue_tx];
        let s = cx
            .consume_context::<storage::Store>()
            .expect("Could not get Store from context");
        let queue_position = item.queue_position.clone();
        cx.spawn(async move {
            let new_queue = new_client(*s).remove_from_queue(vec![queue_position]).await;
            if let Err(e) = queue_tx.try_send(QueueUpdate(new_queue, false)) {
                log::error!("Error sending remove_from_queue result to channel: {e}");
            }
        });
    };
    cx.render(rsx! {
        Tr {
            Td {
                AlbumCover {
                    uri: item.release_cover_uri.as_deref(),
                    size: 30,
                    border: false,
                },
            },
            Td {
                class: "{td_class}",
                strong { "{track.display_title}" },
                br { },
                UnderlineLink {
                    color: Color::Indigo,
                    to: "{release_url}",
                    "{item.release_display_title}",
                },
                " • ",
                UnderlineLink {
                    color: Color::Indigo,
                    to: "{artist_url}",
                    "{item.artist_display_name}",
                },
            },
            Td {
                "{time}",
            },
            Td {
                IconButton {
                    class: "{icon_class}",
                    fill: "white",
                    title: "Remove from queue",
                    size: 20,
                    icon: Shape::X,
                    onclick: on_delete_click,
                },
            },
            Td {
                IconButton {
                    title: "More actions",
                    icon: Shape::DotsVertical,
                    size: 20,
                },
            },
        },
    })
}

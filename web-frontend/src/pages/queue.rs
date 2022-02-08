use crate::{
    client::QueueItem,
    components::{AlbumCover, PageTitle}, //, Table, Td, Tr},
    models::Queue,
    prelude::*,
    storage,
    util::{format_time, maybe_plural, new_client},
    QueueRecvResult,
    QueueUpdate,
};
use dioxus::router::{use_route, Link};
use dioxus_heroicons::{solid::Shape, IconButton};

#[inline_props]
pub(crate) fn Queue<'a>(
    cx: Scope,
    queue: &'a Option<QueueRecvResult>,
    queue_tx: async_channel::Sender<QueueUpdate>,
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
                        }
                    }
                }
                Err(e) => {
                    log::error!("Error loading queue: {}", e);
                    rsx! {
                        div {
                            class: DC![C.fg.col_span_7],
                            "Error loading queue"
                        },
                    }
                }
            },
            Err(e) => {
                log::error!("Error getting message from channel: {}", e);
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

    cx.render(rsx! {
        PageTitle { "Queue" },
        content,
    })
}

#[inline_props]
fn Tracks<'a>(cx: Scope, queue: &'a Queue) -> Element {
    let past_items = match queue.past_items() {
        None => None,
        Some(items) => {
            let past_tracks = maybe_plural(items.len() as u32, "past track");
            Some(rsx! {
                "x"
                // Tr {
                //     key: "past items",
                //     Td {
                //         "colspan": "4",
                //         "{past_tracks}",
                //     }
                // }
            })
        }
    };
    let content = match queue.visible_items() {
        None => rsx! { "Queue is empty" },
        Some(items) => rsx! {
            "x"
            // Table {
            //     past_items,
            //     items.iter().map(|i| rsx! {
            //         OneQueueItem {
            //             key: "{i.queue_position}",
            //             item: i,
            //         }
            //     })
            // }
        },
    };

    cx.render(rsx! {
        content
    })
}

#[inline_props]
fn OneQueueItem<'a>(cx: Scope, item: &'a QueueItem) -> Element {
    let track = item.release_track.as_ref().unwrap();
    let release_url = item.release_url();
    let artist_url = item.artist_url();
    let time = format_time(track.length.unwrap_or(0));
    cx.render(rsx! {
        "item"
        // Tr {
        //     Td {
        //         AlbumCover {
        //             uri: item.release_cover_uri.as_deref(),
        //             size: 30,
        //             ring: false,
        //         },
        //     },
        //     Td {
        //         "{track.display_title}",
        //         br { },
        //         Link {
        //             to: "{release_url}",
        //             "{item.release_display_title}",
        //         },
        //         " • ",
        //         Link {
        //             to: "{artist_url}",
        //             "{item.artist_display_name}",
        //         },
        //     },
        //     Td {
        //         "{time}",
        //     },
        //     Td {
        //         IconButton {
        //             class: "p-1 bg-indigo-400",
        //             fill: "white",
        //             title: "Delete from queue",
        //             size: 15,
        //             icon: Shape::Trash,
        //         },
        //     },
        // },
    })
}

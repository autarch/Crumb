use crate::{
    client::QueueItem,
    components::AlbumCover,
    css,
    icons::{IconButton, Shape},
    QueueFetchResult, QueueRecvResult,
};
use dioxus::prelude::*;

#[inline_props]
pub(crate) fn NowPlaying<'a>(
    cx: Scope,
    queue: &'a Option<QueueRecvResult>,
    queue_tx: async_channel::Sender<QueueFetchResult>,
    is_playing: &'a UseState<bool>,
) -> Element {
    let classes = css::Classes::builder()
        .classes("fixed inset-x-0 bottom-0 h-24 w-screen bg-indigo-500 text-white")
        .with_standard_padding(true)
        .build();
    cx.render(rsx! {
        div{
            class: format_args!("{}", classes),
            audio {
                class: "bg-black h-1 w-full",
            },
            div {
                class: "px-8 py-4",
                div {
                    class: "grid grid-cols-7 place-items-center text-white",
                    div {
                        class: "col-span-3",
                        CurrentTrack {
                            queue: queue,
                            is_playing: is_playing,
                        },
                    },
                    PrevPlayPauseNextButtons { },
                    div {
                        class: "col-span-3",
                        AdditionalButtons { },
                    },
                },
            },
        },
    })
}

#[inline_props]
fn CurrentTrack<'a>(
    cx: Scope,
    queue: &'a Option<QueueRecvResult>,
    is_playing: &'a UseState<bool>,
) -> Element {
    // let content = match queue.value() {
    //     Some(queue) => match queue {
    //         Ok(queue) => match queue {
    //             Ok(queue) => match queue.is_empty() {
    //                 true => rsx! {
    //                     div {
    //                         class: "col-span-6",
    //                         "Queue is empty",
    //                     },
    //                 },
    //                 false => {
    //                     let item = queue.current_item();
    //                     rsx! {
    //                         div {
    //                             AlbumCover {
    //                                 uri: item.release_cover_uri.as_deref(),
    //                                 size: 50,
    //                             },
    //                         },
    //                         div {
    //                             class: "col-span-3",
    //                             CurrentTrackItem { item: item },
    //                         },
    //                         div {
    //                             class: "col-span-2",
    //                             ThumbButtons { },
    //                         },
    //                     }
    //                 }
    //             },
    //             Err(e) => {
    //                 log::error!("Error loading queue: {}", e);
    //                 rsx! {
    //                     div {
    //                         class: "col-span-6",
    //                         "Error loading queue"
    //                     },
    //                 }
    //             }
    //         },
    //         Err(e) => {
    //             log::error!("Error getting message from channel: {}", e);
    //             rsx! {
    //                 div {
    //                     class: "col-span-6",
    //                     "Error getting message from channel"
    //                 },
    //             }
    //         }
    //     },
    //     None => {
    //         rsx! {
    //             div {
    //                 class: "col-span-6",
    //                 "Loading queue ..."
    //             },
    //         }
    //     }
    // };

    cx.render(rsx! {
        div {
            class: "grid grid-cols-6 gap-6 place-content-center items-center",
            "content",
        }
    })
}

#[inline_props]
fn CurrentTrackItem<'a>(cx: Scope, item: &'a QueueItem) -> Element<'a> {
    let track = item
        .release_track
        .as_ref()
        .expect("Queue item release track was somehow None");
    let artist_url = item.artist_url();
    let release_url = item.release_url();
    cx.render(rsx! {
        div {
            class: "truncate",
            "{track.display_title}",
            br{ },
            a {
                href: "{artist_url}",
                "{item.artist_display_name}",
            },
            " - ",
            a {
                href: "{release_url}",
                "{item.release_display_title}",
            },
        }
    })
}

fn ThumbButtons(cx: Scope) -> Element {
    let disabled = true;
    cx.render(rsx! {
        IconButton {
            class: "pr-4",
            title: "I like it",
            disabled: disabled,
            size: 30,
            shape: Shape::ThumbsUp,
        },
        IconButton {
            title: "I don't like it",
            disabled: disabled,
            size: 30,
            shape: Shape::ThumbsDown,
        },
    })
}

fn PrevPlayPauseNextButtons(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            PreviousButton { }
            PlayPauseButton { }
            NextButton { }
        }
    })
}

fn PreviousButton(cx: Scope) -> Element {
    let disabled = true;
    cx.render(rsx! {
        IconButton {
            title: "Previous track",
            disabled: disabled,
            size: 50,
            shape: Shape::Rewind,
        },
    })
}

fn PlayPauseButton(cx: Scope) -> Element {
    let disabled = true;
    let is_playing = false;
    cx.render(rsx! {
        IconButton {
            title: if is_playing { "Pause" } else { "Play" },
            disabled: disabled,
            size: 50,
            shape: if is_playing { Shape::Pause } else { Shape::Play },
        },
    })
}

fn NextButton(cx: Scope) -> Element {
    let disabled = true;
    cx.render(rsx! {
        IconButton {
            title: "Next track",
            disabled: disabled,
            size: 50,
            shape: Shape::FastForward,
        },
    })
}

fn AdditionalButtons(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "grid grid-cols-6 place-items-center",
            div {
                IconButton {
                    title: "Unmute",
                    shape: Shape::VolumeMute,
                    size: 30,
                },
            },
            div {
                class: "col-span-2",
                input {
                    id: "volume",
                    r#type: "range",
                    min: "0",
                    max: "100",
                    step: "1",
                    title: "volume",
                },
            },
            div {
                IconButton {
                    title: "More actions",
                    shape: Shape::DotsVertical,
                    size: 30,
                },
            },
        }
    })
}

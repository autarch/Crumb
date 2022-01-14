use crate::{
    client,
    components::AlbumCover,
    css,
    icons::{IconButton, Shape},
    models::Queue,
};
use dioxus::prelude::*;

#[inline_props]
pub(crate) fn NowPlaying<'a>(
    cx: Scope,
    queue: &'a UseFuture<Result<Queue, client::Error>>,
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
                    class: "grid grid-cols-3 justify-center items-center text-white",
                    CurrentTrack { queue: queue },
                    PrevPlayPauseNextButtons { },
                    AdditionalButtons { },
                },
            },
        },
    })
}

#[inline_props]
fn CurrentTrack<'a>(cx: Scope, queue: &'a UseFuture<Result<Queue, client::Error>>) -> Element {
    let content = match queue.value() {
        Some(Ok(queue)) => match queue.is_empty() {
            true => rsx! {
                div {
                    class: "col-span-5",
                    "Queue is empty",
                },
            },
            false => match &queue.current {
                Some(c) => {
                    let artist = &c.artist;
                    let release = &c.release;
                    let track = queue.current_track().unwrap();
                    rsx! {
                        AlbumCover {
                            class: "mr-6",
                            uri: artist.album_cover_uri.as_deref(),
                            size: 16,
                        },
                        div {
                            class: "col-span-4",
                            "{track.display_title}",
                            br{ },
                            a {
                                href: format_args!("{}", artist.url()),
                                "{artist.display_name}",
                            },
                            " - ",
                            a {
                                href: format_args!("{}", release.url()),
                                "{release.display_title}",
                            },
                        },
                    }
                }
                None => rsx! {
                    div {
                        class: "col-span-5",
                        "Nothing is currently playing"
                    },
                },
            },
        },
        Some(Err(e)) => {
            log::error!("Error loading queue: {}", e);
            rsx! {
                div {
                    class: "col-span-5",
                    "Error loading queue"
                },
            }
        }
        None => {
            rsx! {
                div {
                    class: "col-span-5",
                    "Loading queue",
                },
            }
        }
    };

    cx.render(rsx! {
        div {
            class: "grid grid-cols-5 items-center",
            content
        }
    })
}

// fn view_current_track(queue: &Ref<Queue>) -> Node<Msg> {
//     let current = match &queue.current {
//         Some(c) => c,
//         None => return Node::Empty,
//     };
//     let artist = current.artist.core.as_ref().unwrap();
//     let release = current.release.core.as_ref().unwrap();
//     let contents = match queue.is_empty() {
//         true => vec![div![C![C.col_span_5], "Queue is empty"]],
//         false => match &queue.current {
//             None => vec![div![C![C.col_span_5], "Nothing is currently playing."]],
//             Some(current) => {
//                 let current_track = queue.current_track().unwrap();
//                 vec![
//                     components::release_image(artist, release, Some(&[C.h_auto, C.w_16, C.mr_6]))
//                         .map_msg(Msg::ComponentsMsg),
//                     div![
//                         C![C.col_span_4],
//                         &current_track.display_title,
//                         br![],
//                         a![attrs! { At::Href => artist.url() }, &artist.display_name,],
//                         " - ",
//                         a![attrs! { At::Href => release.url() }, &release.display_title,]
//                     ],
//                 ]
//             }
//         },
//     };

//     div![C![C.grid, C.grid_cols_5, C.items_center], contents]
// }

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
            size: 35,
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
            size: 35,
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
            size: 35,
            shape: Shape::FastForward,
        },
    })
}

fn AdditionalButtons(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "grid grid-cols-6 items-center justify-center",
            div {
                class: "flex flex-1",
                IconButton {
                    title: "Unmute",
                    shape: Shape::VolumeMute,
                    size: 30,
                },
            },
            div {
                class: "col-span-2 flex flex-1",
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
                class: "flex flex-1",
                IconButton {
                    title: "More actions",
                    shape: Shape::DotsVertical,
                    size: 30,
                },
            },
            ThumbButtons { },
        }
    })
}

fn ThumbButtons(cx: Scope) -> Element {
    let disabled = true;
    cx.render(rsx! {
        div {
            class: "flex flex-1",
            IconButton {
                title: "I like it",
                disabled: disabled,
                size: 30,
                shape: Shape::ThumbsUp,
            },
        },
        div {
            class: "flex flex-1",
            IconButton {
                title: "I don't like it",
                disabled: disabled,
                size: 30,
                shape: Shape::ThumbsDown,
            },
        },
    })
}

// fn view_thumb_buttons(queue: &Ref<Queue>) -> Vec<Node<Msg>> {
//     let disabled = queue.current_track().is_none();
//     let mut thumbs_up = icons::thumbs_up()
//         .size(30)
//         .title("I like it")
//         .build()
//         .into_svg()
//         .map_msg(Msg::IconsMsg);
//     let mut thumbs_down = icons::thumbs_down()
//         .size(30)
//         .title("I don't like it")
//         .build()
//         .into_svg()
//         .map_msg(Msg::IconsMsg);
//     if disabled {
//         disable_icon(&mut thumbs_up);
//         disable_icon(&mut thumbs_down);
//     }

//     vec![
//         div![
//             C![C.flex, C.flex_1, C.items_center, C.justify_center],
//             button![IF![ disabled => attrs!{ At::Disabled => 1 } ], thumbs_up],
//         ],
//         div![
//             C![C.flex, C.flex_1, C.items_center, C.justify_center],
//             button![IF![ disabled => attrs!{ At::Disabled => 1 } ], thumbs_down],
//         ],
//     ]
// }

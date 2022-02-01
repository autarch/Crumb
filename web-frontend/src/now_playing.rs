use crate::{
    client::QueueItem, components::AlbumCover, css, models::Queue, storage, util::new_client,
    QueueFetchResult, QueueRecvResult,
};
use dioxus::{core::UiEvent, events::FormData, prelude::*};
use dioxus_heroicons::{solid::Shape, IconButton};
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlAudioElement};

const AUDIO_PLAYER_ID: &str = "audio-player";
const DEFAULT_VOLUME: u32 = 500;

#[inline_props]
pub(crate) fn NowPlaying<'a>(
    cx: Scope,
    queue: &'a Option<QueueRecvResult>,
    queue_tx: async_channel::Sender<QueueFetchResult>,
    is_playing: &'a bool,
    set_is_playing: &'a UseState<bool>,
) -> Element {
    let (volume, set_volume) = use_state(&cx, || {
        let store = *cx
            .consume_context::<storage::Store>()
            .expect("Could not get Store from context");
        store
            .get::<u32>("volume")
            .expect("Error getting volume from storage")
            .unwrap_or_else(|| {
                store
                    .set::<u32>("volume", DEFAULT_VOLUME)
                    .expect("error setting volume in storage");
                DEFAULT_VOLUME
            })
    });

    let content = match queue {
        Some(queue) => match queue {
            Ok(queue) => match queue {
                Ok(queue) => {
                    rsx! {
                        div {
                            class: "col-span-3 text-left",
                            CurrentTrack {
                                queue: queue,
                            },
                        },
                        div {
                            class: "text-center",
                            PrevPlayPauseNextButtons {
                                queue: queue,
                                queue_tx: queue_tx.clone(),
                                is_playing: is_playing,
                                set_is_playing: set_is_playing,
                            },
                        },
                        div {
                            class: "col-span-3 text-right",
                            AdditionalButtons {
                                volume: volume,
                                set_volume: set_volume,
                            },
                        },
                    }
                }
                Err(e) => {
                    log::error!("Error loading queue: {}", e);
                    rsx! {
                        div {
                            class: "col-span-7",
                            "Error loading queue"
                        },
                    }
                }
            },
            Err(e) => {
                log::error!("Error getting message from channel: {}", e);
                rsx! {
                    div {
                        class: "col-span-7",
                        "Error getting message from channel"
                    },
                }
            }
        },
        None => {
            rsx! {
                div {
                    class: "col-span-7",
                    "Loading queue ..."
                },
            }
        }
    };

    let audio_src = queue
        .as_ref()
        .and_then(|q| q.as_ref().ok())
        .and_then(|q| q.as_ref().ok())
        .and_then(|q| q.current_item())
        .and_then(|i| {
            Some(
                i.release_track
                    .as_ref()
                    .unwrap()
                    .track_audio_uri
                    .to_string(),
            )
        })
        .unwrap_or(String::new());
    let classes = css::Classes::builder()
        .classes("fixed inset-x-0 bottom-0 h-24 w-screen bg-indigo-500 text-white")
        .with_standard_padding(true)
        .build();
    cx.render(rsx! {
        div{
            class: "{classes}",
            audio {
                id: "{AUDIO_PLAYER_ID}",
                class: "bg-black h-1 w-full",
                autoplay: format_args!("{}", if **is_playing { "true" } else { "false" }),
                preload: format_args!("{}", if **is_playing { "auto" } else { "none" }),
                src: "{audio_src}",
            },
            div {
                class: "px-8 py-4",
                div {
                    class: "grid grid-cols-7 items-center text-white",
                    content,
                },
            },
        },
    })
}

#[inline_props]
fn CurrentTrack<'a>(cx: Scope, queue: &'a Queue) -> Element {
    let content = match queue.is_empty() {
        true => rsx! {
            div {
                class: "col-span-6",
                "Queue is empty",
            },
        },
        false => {
            let item = queue.current_item().unwrap();
            rsx! {
                div {
                    AlbumCover {
                        uri: item.release_cover_uri.as_deref(),
                        size: 50,
                    },
                },
                div {
                    class: "col-span-2",
                    CurrentTrackItem {
                        item: item,
                    },
                },
                div {
                    class: "col-span-3",
                    ThumbButtons {
                        queue: queue,
                    },
                },
            }
        }
    };

    cx.render(rsx! {
        div {
            class: "grid grid-cols-6 items-center gap-6",
            content,
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

#[inline_props]
fn ThumbButtons<'a>(cx: Scope, queue: &'a Queue) -> Element {
    let disabled = queue.is_empty();

    let up_onclick = move |_| {
        if disabled {
            return;
        }
        let current_track_id = queue
            .current_item()
            .as_ref()
            .unwrap()
            .release_track
            .as_ref()
            .unwrap()
            .track_id
            .clone();
        let store = cx
            .consume_context::<storage::Store>()
            .expect("Could not get Store from context");
        cx.spawn(async move {
            let response = new_client(*store).like_track(current_track_id).await;
        });
    };

    let down_onclick = move |_| {
        if disabled {
            return;
        }
        let current_track_id = queue
            .current_item()
            .as_ref()
            .unwrap()
            .release_track
            .as_ref()
            .unwrap()
            .track_id
            .clone();
        let store = cx
            .consume_context::<storage::Store>()
            .expect("Could not get Store from context");
        cx.spawn(async move {
            let response = new_client(*store).dislike_track(current_track_id).await;
        });
    };

    cx.render(rsx! {
        IconButton {
            onclick: up_onclick,
            class: "pr-4",
            title: "I like it",
            disabled: disabled,
            size: 30,
            icon: Shape::ThumbUp,
        },
        IconButton {
            onclick: down_onclick,
            title: "I don't like it",
            disabled: disabled,
            size: 30,
            icon: Shape::ThumbDown,
        },
    })
}

#[inline_props]
fn PrevPlayPauseNextButtons<'a>(
    cx: Scope<'a>,
    queue: &'a Queue,
    queue_tx: async_channel::Sender<QueueFetchResult>,
    is_playing: &'a bool,
    set_is_playing: &'a UseState<bool>,
) -> Element<'a> {
    cx.render(rsx! {
        div {
            PreviousButton {
                queue: queue,
                queue_tx: queue_tx.clone(),
                is_playing: is_playing,
            }
            PlayPauseButton {
                queue: queue,
                is_playing: is_playing,
                set_is_playing: set_is_playing,
            }
            NextButton {
                queue: queue,
                queue_tx: queue_tx.clone(),
                is_playing: is_playing,
            }
        }
    })
}

#[inline_props]
fn PreviousButton<'a>(
    cx: Scope<'a>,
    queue: &'a Queue,
    queue_tx: async_channel::Sender<QueueFetchResult>,
    is_playing: &'a bool,
) -> Element<'a> {
    let disabled = !queue.can_move_to_previous();
    let onclick = move |_| {
        to_owned![queue_tx];
        let should_play = **is_playing;
        let store = cx
            .consume_context::<storage::Store>()
            .expect("Could not get Store from context");
        cx.spawn(async move {
            let new_queue = new_client(*store).move_queue_backward().await;
            if let Err(e) = queue_tx.send(new_queue).await {
                log::error!("Error sending move queue backward result to channel: {}", e);
            }
            play_or_pause_audio(should_play);
        });
    };
    cx.render(rsx! {
        IconButton {
            onclick: onclick,
            title: "Previous track",
            disabled: disabled,
            size: 50,
            icon: Shape::Rewind,
        },
    })
}

#[inline_props]
fn PlayPauseButton<'a>(
    cx: Scope<'a>,
    queue: &'a Queue,
    is_playing: &'a bool,
    set_is_playing: &'a UseState<bool>,
) -> Element<'a> {
    let disabled = !queue.can_play();
    let onclick = move |_| {
        let new_state = !*is_playing;
        set_is_playing(new_state);
        play_or_pause_audio(new_state);
    };
    cx.render(rsx! {
        IconButton {
            onclick: onclick,
            title: if **is_playing { "Pause" } else { "Play" },
            disabled: disabled,
            size: 50,
            icon: if **is_playing { Shape::Pause } else { Shape::Play },
        },
    })
}

#[inline_props]
fn NextButton<'a>(
    cx: Scope<'a>,
    queue: &'a Queue,
    queue_tx: async_channel::Sender<QueueFetchResult>,
    is_playing: &'a bool,
) -> Element<'a> {
    let disabled = !queue.can_move_to_next();
    let onclick = move |_| {
        to_owned![queue_tx];
        let should_play = **is_playing;
        let mut client = new_client(
            *cx.consume_context::<storage::Store>()
                .expect("Could not get Store from context"),
        );
        cx.spawn(async move {
            let new_queue = client.move_queue_forward().await;
            if let Err(e) = queue_tx.send(new_queue).await {
                log::error!("Error sending move queue forward result to channel: {}", e);
            }
            play_or_pause_audio(should_play);
        });
    };
    cx.render(rsx! {
        IconButton {
            onclick: onclick,
            title: "Next track",
            disabled: disabled,
            size: 50,
            icon: Shape::FastForward,
        },
    })
}

#[inline_props]
fn AdditionalButtons<'a>(cx: Scope, volume: &'a u32, set_volume: &'a UseState<u32>) -> Element {
    let mute_onclick = move |_| {
        to_owned![set_volume];
        let store = cx
            .consume_context::<storage::Store>()
            .expect("Could not get Store from context");
        if **volume > 0 {
            store
                .set("volume", **volume)
                .expect("could not set volume in storage");
            update_volume(0, &set_volume);
        } else {
            let stored = store
                .get("volume")
                .expect("could not get volume from storage");
            update_volume(stored.unwrap_or(DEFAULT_VOLUME), &set_volume);
        }
    };
    let volume_onchange = move |e: UiEvent<FormData>| {
        to_owned![set_volume];
        let store = cx
            .consume_context::<storage::Store>()
            .expect("Could not get Store from context");
        let new_volume = e
            .data
            .value
            .parse::<u32>()
            .expect("could not parse volume input as a u32");
        store
            .set("volume", new_volume)
            .expect("could not set volume in storage");
        update_volume(new_volume, &set_volume);
    };

    cx.render(rsx! {
        div {
            class: "grid grid-cols-4 items-center text-center",
            div {
                IconButton {
                    onclick: mute_onclick,
                    title: if **volume > 0 { "Mute" } else { "Unmute" },
                    icon: if **volume > 0 { Shape::VolumeUp } else { Shape::VolumeOff },
                    size: 30,
                },
            },
            div {
                input {
                    onchange: volume_onchange,
                    id: "volume",
                    r#type: "range",
                    min: "0",
                    max: "1000",
                    step: "1",
                    title: "volume",
                    value: format_args!("{}", volume),
                },
            },
            div {
                IconButton {
                    title: "More actions",
                    icon: Shape::DotsVertical,
                    size: 30,
                },
            },
        }
    })
}

fn update_volume(new_volume: u32, set_volume: &UseState<u32>) {
    // We don't interact with the store here, because we don't want to write
    // the volume to the store every time it's changed. That's because we use
    // the store to _save_ the volume when the mute button is clicked. Then we
    // read it from the store on unmute. If we stored it here, then we'd store
    // the 0 volume from the mute, which means that unmute wouldn't be able to
    // get the previous volume back.
    set_volume(new_volume);
    get_audio_element()
        .expect("Could not get audio element")
        .set_volume(new_volume as f64 / 1000.0);
}

fn play_or_pause_audio(should_play: bool) {
    if let Some(audio) = get_audio_element() {
        if should_play {
            if let Err(e) = audio.play() {
                log::error!("Could not call play() on audio element: {:?}", e);
            }
        } else {
            if let Err(e) = audio.pause() {
                log::error!("Could not call pause() on audio element: {:?}", e);
            }
        }
    }
}

fn get_audio_element() -> Option<HtmlAudioElement> {
    match window() {
        Some(w) => match w.document() {
            Some(d) => match d.get_element_by_id(AUDIO_PLAYER_ID) {
                Some(a) => match a.dyn_into::<HtmlAudioElement>() {
                    Ok(a) => return Some(a),
                    Err(e) => {
                        log::error!("Could not cast audio player element into an HtmlAudioElement");
                    }
                },
                None => {
                    log::error!(
                        "Could not get {} element from web-sys document",
                        AUDIO_PLAYER_ID
                    );
                }
            },
            None => {
                log::error!("Could not get document from web-sys window");
            }
        },
        None => {
            log::error!("Could not get window from web-sys");
        }
    }

    None
}

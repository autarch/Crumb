use crate::{
    grpc::crumb::QueueItem,
    components::{AlbumCover, Color, ContextMenu, UnderlineLink},
    css,
    models::Queue,
    prelude::*,
    storage,
    util::{get_element, new_client},
    ContextMenus, QueueRecvResult, QueueUpdate,
};
use dioxus::events::FormEvent;
use dioxus_heroicons::{solid::Shape, IconButton};
use web_sys::HtmlAudioElement;

const AUDIO_PLAYER_ID: &str = "audio-player";
const DEFAULT_VOLUME: u32 = 500;

#[inline_props]
pub(crate) fn NowPlaying<'a>(
    cx: Scope,
    queue: &'a Option<QueueRecvResult>,
    queue_tx: futures_channel::mpsc::Sender<QueueUpdate>,
    is_playing: UseState<bool>,
) -> Element {
    let volume = use_state(&cx, || {
        let store = cx
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
                            class: DC![C.fg.col_span_3, C.typ.text_left],
                            CurrentTrack {
                                queue: queue,
                            },
                        },
                        div {
                            class: DC![C.typ.text_center],
                            PrevPlayPauseNextButtons {
                                queue: queue,
                                queue_tx: queue_tx.clone(),
                                is_playing: is_playing.clone(),
                            },
                        },
                        div {
                            class: DC![C.fg.col_span_3, C.typ.text_right],
                            AdditionalButtons {
                                volume: volume.clone(),
                            },
                        },
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
            rsx! {
                div {
                    class: DC![C.fg.col_span_7],
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
        .and_then(|i| Some(i.release_track.as_ref().unwrap().track_audio_uri.as_str()))
        .unwrap_or("");
    let classes = css::Classes::builder()
        .classes(C![
            C.lay.fixed,
            C.lay.inset_x_0,
            C.lay.bottom_0,
            C.siz.h_24,
            C.siz.w_screen,
            C.bg.bg_indigo_500,
            C.typ.text_white,
        ])
        .with_standard_padding(true)
        .build();
    cx.render(rsx! {
        div{
            class: "{classes}",
            audio {
                id: "{AUDIO_PLAYER_ID}",
                class: DC![C.bg.bg_black, C.siz.h_1, C.siz.w_full],
                autoplay: format_args!("{}", if *is_playing.current() { "true" } else { "false" }),
                preload: format_args!("{}", if *is_playing.current() { "auto" } else { "none" }),
                src: "{audio_src}",
            },
            div {
                class: DC![C.spc.px_8, C.spc.py_4],
                div {
                    class: DC![C.lay.grid, C.fg.grid_cols_7, C.fg.items_center, C.typ.text_white],
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
                class: DC![C.fg.col_span_6],
                "Queue is empty",
            },
        },
        false => {
            let item = queue.current_item().unwrap();
            rsx! {
                div {
                    AlbumCover {
                        uri: item.release_cover_uri.as_deref().unwrap(),
                        size: 50,
                    },
                },
                div {
                    class: DC![C.fg.col_span_4],
                    CurrentTrackItem {
                        item: item,
                    },
                },
                div {
                    ThumbButtons {
                        queue: queue,
                    },
                },
            }
        }
    };

    cx.render(rsx! {
        div {
            class: DC![C.lay.grid, C.fg.grid_cols_6, C.fg.items_center],
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
            class: DC![C.typ.truncate],
            strong { "{track.display_title}" },
            br{ },
            UnderlineLink {
                color: Color::White,
                to: "{artist_url}",
                "{item.artist_display_name}",
            },
            " - ",
            UnderlineLink {
                color: Color::White,
                to: "{release_url}",
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
            let response = new_client(store).like_track(current_track_id).await;
            match response {
                Ok(_) => (),
                Err(e) => log::error!("Error from liking track: {e}"),
            }
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
            let response = new_client(store).dislike_track(current_track_id).await;
            match response {
                Ok(_) => (),
                Err(e) => log::error!("Error from disliking track: {e}"),
            }
        });
    };

    let class = C![C.spc.pr_4];
    cx.render(rsx! {
        IconButton {
            onclick: up_onclick,
            class: "{class}",
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
    queue_tx: futures_channel::mpsc::Sender<QueueUpdate>,
    is_playing: UseState<bool>,
) -> Element<'a> {
    cx.render(rsx! {
        div {
            PreviousButton {
                queue: queue,
                queue_tx: queue_tx.clone(),
                is_playing: is_playing.clone(),
            }
            PlayPauseButton {
                queue: queue,
                is_playing: is_playing.clone(),
            }
            NextButton {
                queue: queue,
                queue_tx: queue_tx.clone(),
                is_playing: is_playing.clone(),
            }
        }
    })
}

#[inline_props]
fn PreviousButton<'a>(
    cx: Scope<'a>,
    queue: &'a Queue,
    queue_tx: futures_channel::mpsc::Sender<QueueUpdate>,
    is_playing: UseState<bool>,
) -> Element<'a> {
    let disabled = !queue.can_move_to_previous();
    let onclick = move |_| {
        to_owned![queue_tx];
        let should_play = *is_playing.current();
        let store = cx
            .consume_context::<storage::Store>()
            .expect("Could not get Store from context");
        cx.spawn(async move {
            let new_queue = new_client(store).move_queue_backward().await;
            if let Err(e) = queue_tx.try_send(QueueUpdate(new_queue, false)) {
                log::error!("Error sending move queue backward result to channel: {e}");
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
fn PlayPauseButton<'a>(cx: Scope<'a>, queue: &'a Queue, is_playing: UseState<bool>) -> Element<'a> {
    let disabled = !queue.can_play();
    let onclick = move |_| {
        let new_state = !*is_playing.current();
        is_playing.set(new_state);
        play_or_pause_audio(new_state);
    };
    cx.render(rsx! {
        IconButton {
            onclick: onclick,
            title: if *is_playing.current() { "Pause" } else { "Play" },
            disabled: disabled,
            size: 50,
            icon: if *is_playing.current() { Shape::Pause } else { Shape::Play },
        },
    })
}

#[inline_props]
fn NextButton<'a>(
    cx: Scope<'a>,
    queue: &'a Queue,
    queue_tx: futures_channel::mpsc::Sender<QueueUpdate>,
    is_playing: UseState<bool>,
) -> Element<'a> {
    let disabled = !queue.can_move_to_next();
    let onclick = move |_| {
        to_owned![queue_tx];
        let should_play = *is_playing.current();
        let mut client = new_client(
            cx.consume_context::<storage::Store>()
                .expect("Could not get Store from context"),
        );
        cx.spawn(async move {
            let new_queue = client.move_queue_forward().await;
            if let Err(e) = queue_tx.try_send(QueueUpdate(new_queue, false)) {
                log::error!("Error sending move queue forward result to channel: {e}");
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
fn AdditionalButtons(cx: Scope, volume: UseState<u32>) -> Element {
    let mute_onclick = move |_| {
        to_owned![volume];
        let store = cx
            .consume_context::<storage::Store>()
            .expect("Could not get Store from context");
        let current_volume = *volume.current();
        if current_volume > 0 {
            store
                .set("volume", current_volume)
                .expect("could not set volume in storage");
            update_volume(0, volume);
        } else {
            let stored = store
                .get("volume")
                .expect("could not get volume from storage");
            update_volume(stored.unwrap_or(DEFAULT_VOLUME), volume);
        }
    };

    let volume_onchange = move |e: FormEvent| {
        to_owned![volume];
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
        update_volume(new_volume, volume);
    };

    let cm_id = "now-playing-more-actions";
    let context_menus = use_context::<ContextMenus>(&cx).unwrap();
    (*context_menus.write_silent()).register(cm_id);

    cx.render(rsx! {
        div {
            class: DC![C.lay.grid, C.fg.grid_cols_4, C.fg.items_center, C.typ.text_center],
            div {
                IconButton {
                    onclick: mute_onclick,
                    title: if *volume.current() > 0 { "Mute" } else { "Unmute" },
                    icon: if *volume.current() > 0 { Shape::VolumeUp } else { Shape::VolumeOff },
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
                ContextMenu {
                    id: "{cm_id}",
                    div {
                        IconButton {
                            title: "More actions",
                            icon: Shape::DotsVertical,
                            size: 30,
                        },
                    },
                    ul {
                        li { "Delete track" },
                        li { "item 2" },
                    },
                },
            },
        }
    })
}

fn update_volume(new_volume: u32, volume: UseState<u32>) {
    // We don't interact with the store here, because we don't want to write
    // the volume to the store every time it's changed. That's because we use
    // the store to _save_ the volume when the mute button is clicked. Then we
    // read it from the store on unmute. If we stored it here, then we'd store
    // the 0 volume from the mute, which means that unmute wouldn't be able to
    // get the previous volume back.
    volume.set(new_volume);
    get_element::<HtmlAudioElement>(AUDIO_PLAYER_ID)
        .expect("Could not get audio element")
        .set_volume(new_volume as f64 / 1000.0);
}

fn play_or_pause_audio(should_play: bool) {
    if let Some(audio) = get_element::<HtmlAudioElement>(AUDIO_PLAYER_ID) {
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

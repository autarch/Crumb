use crate::{
    client::{get_release_response, ReleaseItem, ReleaseTrack},
    components::{AlbumCover, PageTitle, SubTitle, Table, Td, Tr, UnderlineLink, UserFacingError},
    prelude::*,
    storage,
    util::{format_time, maybe_plural, new_client},
    QueueUpdate,
};
use dioxus::router::use_route;
use dioxus_heroicons::{solid::Shape, IconButton};
use std::collections::HashMap;

#[derive(Props)]
pub(crate) struct ReleaseProps<'a> {
    queue_tx: futures_channel::mpsc::Sender<QueueUpdate>,
    // This is here because the Sender type does not implement PartialEq, so
    // we cannot use it with #[inline_props], and if we don't have a reference
    // in our props the Props macro complains that the Sender is not
    // comparable.
    #[props(optional)]
    _cache_breaker: Option<&'a ()>,
}

pub(crate) fn Release<'a>(cx: Scope<'a, ReleaseProps<'a>>) -> Element<'a> {
    let release_id = use_route(&cx)
        .segment::<String>("release_id")
        .expect("release_id parameter was not found in path somehow")
        .expect("release_id parameter could not be parsed as a String");
    cx.render(rsx! {
        ReleaseFromRoute {
            release_id: release_id,
            queue_tx: cx.props.queue_tx.clone(),
        }
    })
}

#[derive(Props)]
pub(crate) struct ReleaseFromRouteProps<'a> {
    release_id: String,
    queue_tx: futures_channel::mpsc::Sender<QueueUpdate>,
    #[props(optional)]
    _cache_breaker: Option<&'a ()>,
}

fn ReleaseFromRoute<'a>(cx: Scope<'a, ReleaseFromRouteProps<'a>>) -> Element {
    let release_id = cx.props.release_id.clone();
    let release = use_future(&cx, || {
        let s = cx
            .consume_context::<storage::Store>()
            .expect("Could not get Store from context");
        async move { new_client(*s).get_release(&release_id).await }
    });

    cx.render(rsx! {
        match release.value() {
            Some(Ok(response)) => {
                match &response.response_either {
                    Some(get_release_response::ResponseEither::Release(release)) => rsx! {
                        LoadedRelease {
                            release: release,
                            queue_tx: cx.props.queue_tx.clone(),
                        }
                    },
                    Some(get_release_response::ResponseEither::Error(e)) => rsx! {
                        UserFacingError {
                            error: e
                        }
                    },
                    None => {
                        log::error!("Empty response for GetRelease request!");
                        rsx! {
                            "Error loading release"
                        }
                    }
                }
            },
            Some(Err(e)) => {
                log::error!("Error loading release: {}", e);
                rsx! {
                    "Error loading release"
                }
            },
            None => {
                rsx! {
                    "Loading release",
                }
            },
        }
    })
}

#[inline_props]
fn LoadedRelease<'a>(
    cx: Scope,
    release: &'a ReleaseItem,
    queue_tx: futures_channel::mpsc::Sender<QueueUpdate>,
) -> Element {
    let core = release.core.as_ref().unwrap();
    let artist = release.artist.as_ref().unwrap();
    let artist_url = artist.url();
    let year = core.best_release_year("");
    let track_count = maybe_plural(release.tracks.len() as u32, "track");
    let len: u32 = release.tracks.iter().map(|t| t.length.unwrap_or(0)).sum();
    let time = format_time(len);

    let play_onclick = move |_| {
        to_owned![queue_tx];
        let s = cx
            .consume_context::<storage::Store>()
            .expect("Could not get Store from context");
        let track_ids = release
            .tracks
            .iter()
            .map(|t| t.track_id.clone())
            .collect::<Vec<_>>();
        cx.spawn(async move {
            let new_queue = new_client(*s).replace_queue(track_ids).await;
            if let Err(e) = queue_tx.try_send(QueueUpdate(new_queue, true)) {
                log::error!("Error sending replace_queue result to channel: {e}");
            }
        });
    };
    let enqueue_onclick = move |_| {
        to_owned![queue_tx];
        let s = cx
            .consume_context::<storage::Store>()
            .expect("Could not get Store from context");
        let track_ids = release
            .tracks
            .iter()
            .map(|t| t.track_id.clone())
            .collect::<Vec<_>>();
        cx.spawn(async move {
            let new_queue = new_client(*s).add_to_queue(track_ids).await;
            if let Err(e) = queue_tx.try_send(QueueUpdate(new_queue, false)) {
                log::error!("Error sending add_to_queue result to channel: {e}");
            }
        });
    };

    let play_class = C![
        C.spc.pt_1,
        C.spc.pb_2,
        C.spc.pl_1,
        C.spc.pr_3,
        C.spc.mr_2,
        C.bg.bg_indigo_400,
        C.typ.font_bold,
        C.typ.text_white,
        C.typ.text_lg,
    ];
    let icon_class = C![C.lay.inline_block, C.typ.align_middle];
    let play_span_class = C![C.spc.px_1, C.spc.mr_1];
    let enqueue_class = C![C.spc.p_2, C.bg.bg_indigo_400];

    cx.render(rsx! {
        div {
            class: DC![C.lay.flex, C.fg.flex_col, C.spc.pl_8],
            div {
                class: DC![C.lay.flex, C.fg.flex_row],
                div {
                    AlbumCover {
                        uri: core.release_cover_uri.as_deref(),
                        size: 250,
                    },
                },
                div {
                    class: DC![C.lay.flex, C.fg.flex_col, C.spc.pl_8],
                    div {
                        PageTitle {
                            "{core.display_title}"
                        },
                        SubTitle {
                            UnderlineLink {
                                to: "{artist_url}",
                                "{artist.display_name}",
                            },
                            " • ",
                            "{year}",
                        },
                        "{track_count} • {time}",
                    },
                    div {
                        class: DC![C.spc.mt_1],
                        IconButton {
                            onclick: play_onclick,
                            class: "{play_class}",
                            icon_class: "{icon_class}",
                            span_class: "{play_span_class}",
                            size: 25,
                            icon: Shape::Play,
                            fill: "white",
                            title: "Play release immediately, replacing the current queue",
                            "Play",
                        },
                        IconButton {
                            onclick: enqueue_onclick,
                            class: "{enqueue_class}",
                            icon_class: "{icon_class}",
                            size: 25,
                            icon: Shape::InboxIn,
                            fill: "white",
                            title: "Add release to the end of the queue",
                        },
                    },
                },
            },
            Tracks {
                tracks: &release.tracks,
            },
        },
    })
}

#[inline_props]
fn Tracks<'a>(cx: Scope<'a>, tracks: &'a [ReleaseTrack]) -> Element {
    let show_more_states: &UseRef<HashMap<String, bool>> = use_ref(&cx, || {
        let mut sms = HashMap::new();
        for t in tracks.iter() {
            sms.insert(t.track_id.clone(), false);
        }
        sms
    });

    let class = C![C.spc.mt_4, C.spc.ml_2];
    cx.render(rsx! {
        "tracks"
        Table {
            class: "{class}",
            tracks.iter().map(|t| rsx! {
                OneTrack {
                    key: "{t.track_id}",
                    track: t,
                    show_more_states: show_more_states,
                }
            })
        }
    })
}

#[inline_props]
fn OneTrack<'a>(
    cx: Scope,
    track: &'a ReleaseTrack,
    show_more_states: &'a UseRef<HashMap<String, bool>>,
) -> Element {
    let time = format_time(track.length.unwrap_or(0));
    let onclick = move |_| {
        let mut show_more_states = show_more_states.write();
        for v in show_more_states.values_mut() {
            *v = false;
        }
        if let Some(v) = show_more_states.get_mut(&track.track_id) {
            *v = true;
        }
    };

    let mut show_more_classes = vec![
        C.lay.fixed,
        C.spc.py_1,
        C.spc.px_3,
        C.bg.bg_indigo_600,
        C.typ.text_white,
    ];
    if *(show_more_states
        .read()
        .get(&track.track_id)
        .unwrap_or(&false))
    {
        show_more_classes.push(C.lay.visible);
    } else {
        show_more_classes.push(C.lay.hidden);
    };
    cx.render(rsx! {
        Tr {
            Td {
                "{track.position}."
            },
            Td {
                "{track.display_title}",
            },
            Td {
                "{time}",
            },
            Td {
                div {
                    class: DC![show_more_classes],
                    ul {
                        li { "Delete track" },
                        li { "item 2" },
                    },
                },
                IconButton {
                    onclick: onclick,
                    title: "More actions",
                    icon: Shape::DotsVertical,
                    size: 15,
                },
            },
        },
    })
}

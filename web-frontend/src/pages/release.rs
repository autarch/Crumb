use crate::{
    client::{get_release_response, ReleaseItem, ReleaseTrack},
    components::{AlbumCover, PageTitle, SubTitle, UserFacingError},
    storage,
    util::{format_time, maybe_plural, new_client},
    QueueFetchResult,
};
use dioxus::{
    prelude::*,
    router::{use_route, Link},
};
use dioxus_heroicons::{solid::Shape, IconButton};

#[derive(Props)]
pub(crate) struct ReleaseProps<'a> {
    queue_tx: async_channel::Sender<QueueFetchResult>,
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
        .expect("id parameter was not found in path somehow")
        .expect("id parameter could not be parsed as a String");
    let release = use_future(&cx, || {
        let s = cx
            .consume_context::<storage::Store>()
            .expect("Could not get Store from context");
        async move { new_client(*s).get_release(&release_id).await }
    });
    let queue_tx = cx.props.queue_tx.clone();
    cx.render(rsx! {
        match release.value() {
            Some(Ok(response)) => {
                match &response.response_either {
                    Some(get_release_response::ResponseEither::Release(release)) => rsx! {
                        LoadedRelease {
                            release: release,
                            queue_tx: queue_tx,
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
    queue_tx: async_channel::Sender<QueueFetchResult>,
) -> Element {
    let core = release.core.as_ref().unwrap();
    let artist = release.artist.as_ref().unwrap();
    let artist_url = artist.url();
    let year = core.best_release_year("");
    let track_count = maybe_plural(release.tracks.len() as u32, "track");
    let len: u32 = release.tracks.iter().map(|t| t.length.unwrap_or(0)).sum();
    let time = format_time(len);
    let onclick = move |_| {
        to_owned![queue_tx];
        let track_ids = release
            .tracks
            .iter()
            .map(|t| t.track_id.clone())
            .collect::<Vec<_>>();
        let s = cx
            .consume_context::<storage::Store>()
            .expect("Could not get Store from context");
        cx.spawn(async move {
            let new_queue = new_client(*s).add_to_queue(track_ids).await;
            if let Err(e) = queue_tx.send(new_queue).await {
                log::error!("Error sending add to queue result to channel: {}", e);
            }
        });
    };
    cx.render(rsx! {
        div {
            class: "flex flex-col pl-8",
            div {
                class: "flex flex-row",
                div {
                    AlbumCover {
                        uri: core.release_cover_uri.as_deref(),
                        size: 250,
                    },
                },
                div {
                    class: "flex flex-col pl-8",
                    div {
                        PageTitle {
                            "{core.display_title}"
                        },
                        SubTitle {
                            Link {
                                to: "{artist_url}",
                                "{artist.display_name}",
                            },
                            " • ",
                            "{year}",
                        },
                        "{track_count} • {time}",
                    },
                    div {
                        class: "mt-1",
                        IconButton {
                            onclick: onclick,
                            class: "pt-1 pb-2 pl-1 pr-3 bg-indigo-400 font-bold text-white text-lg",
                            icon_class: "inline-block align-middle",
                            span_class: "px-1 mr-1",
                            size: 25,
                            icon: Shape::Play,
                            "Play",
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
fn Tracks<'a>(cx: Scope, tracks: &'a [ReleaseTrack]) -> Element {
    cx.render(rsx! {
        table {
            class: "mt-4 ml-2",
            tracks.iter().map(|t| rsx! {
                OneTrack {
                    key: "{t.track_id}",
                    track: t,
                }
            })
        }
    })
}

#[inline_props]
fn OneTrack<'a>(cx: Scope, track: &'a ReleaseTrack) -> Element {
    let time = format_time(track.length.unwrap_or(0));
    cx.render(rsx! {
        tr {
            td {
                "{track.position}."
            },
            td {
                "{track.display_title}",
            },
            td {
                "{time}",
            }
        },
    })
}

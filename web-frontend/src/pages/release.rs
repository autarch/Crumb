use crate::{
    client::{get_release_response, ReleaseTrack},
    components::{AlbumCover, PageTitle, SubTitle, UserFacingError},
    icons::{IconButton, Shape},
    util::{format_time, maybe_plural, new_client},
    QueueFetchResult, QueueReceiveUseFuture,
};
use dioxus::{
    prelude::*,
    router::{use_route, Link},
};

#[derive(Props)]
pub(crate) struct ReleaseProps<'a> {
    queue: &'a QueueReceiveUseFuture,
    queue_tx: &'a async_channel::Sender<QueueFetchResult>,
}

pub(crate) fn Release<'a>(cx: Scope<'a, ReleaseProps<'a>>) -> Element<'a> {
    let release_id = use_route(&cx)
        .segment::<String>("release_id")
        .expect("id parameter was not found in path somehow")
        .expect("id parameter could not be parsed as a String");
    let release = use_future(&cx, || async move {
        new_client().get_release(&release_id).await
    });
    cx.render(rsx! {
        match release.value() {
            Some(Ok(response)) => {
                match &response.response_either {
                    Some(get_release_response::ResponseEither::Release(release)) => {
                        let core = release.core.as_ref().unwrap();
                        let artist = release.artist.as_ref().unwrap();
                        let artist_url = artist.url();
                        let year = core.best_release_year("");
                        let track_count = maybe_plural(release.tracks.len() as u32, "track");
                        let len: u32 = release.tracks.iter().map(|t| t.length.unwrap_or(0)).sum();
                        let time = format_time(len);
                        let onclick = move |_| {
                            let queue = cx.props.queue.clone();
                            let queue_tx = cx.props.queue_tx.clone();
                            let track_ids = release
                                .tracks
                                .iter()
                                .map(|t| t.track_id.clone())
                                .collect::<Vec<_>>();
                            log::debug!("add to queue: {:?}", track_ids);
                            cx.spawn(async move {
                                log::debug!("calling add_to_queue");
                                let new_queue = new_client().add_to_queue(track_ids).await;
                                log::debug!("got queue back from add_to_queue");
                                queue_tx.send(new_queue).await;
                                queue.restart();
                            });
                        };
                        rsx! {
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
                                            "{track_count}",
                                            " • ",
                                            "{time}",
                                        },
                                        div {
                                            class: "mt-1",
                                            IconButton {
                                                onclick: onclick,
                                                is_inline: true,
                                                size: 25,
                                                shape: Shape::Play,
                                                "Play",
                                            },
                                        },
                                    },
                                },
                                Tracks {
                                    tracks: &release.tracks,
                                },
                            },
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

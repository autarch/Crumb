use crate::{
    client::{get_artist_response, ReleaseListItem},
    components::{AlbumCover, PageTitle, UserFacingError},
    storage,
    util::{maybe_plural, new_client},
};
use dioxus::{
    prelude::*,
    router::{use_route, Link},
};

pub(crate) fn Artist<'a>(cx: Scope) -> Element {
    let artist_id = use_route(&cx)
        .segment::<String>("artist_id")
        .expect("id parameter was not found in path somehow")
        .expect("id parameter could not be parsed as a String");
    let artist = use_future(&cx, || {
        let mut client = new_client(
            *cx.consume_context::<storage::Store>()
                .expect("Could not get Store from context"),
        );
        async move { client.get_artist(&artist_id).await }
    });
    cx.render(rsx! {
        match artist.value() {
            Some(Ok(response)) => {
                match &response.response_either {
                    Some(get_artist_response::ResponseEither::Artist(artist)) => {
                        let core = artist.core.as_ref().unwrap();
                        rsx! {
                            div {
                                class: "text-center",
                                PageTitle {
                                    "{core.display_name}"
                                },
                            },
                            div {
                                class: "flex flex-row flex-wrap place-content-center",
                                artist.releases.iter().map(|r| rsx!{
                                    OneRelease {
                                        key: "{r.release_id}",
                                        release: r,
                                    }
                                }),
                            },
                        }
                    },
                    Some(get_artist_response::ResponseEither::Error(e)) => rsx! {
                        UserFacingError {
                            error: e
                        }
                    },
                    None => {
                        log::error!("Empty response for GetArtist request!");
                        rsx! {
                            "Error loading artist"
                        }
                    }
                }
            },
            Some(Err(e)) => {
                log::error!("Error loading artist: {}", e);
                rsx! {
                    "Error loading artist"
                }
            },
            None => {
                rsx! {
                    "Loading artist",
                }
            },
        }
    })
}

#[inline_props]
fn OneRelease<'a>(cx: Scope, release: &'a ReleaseListItem) -> Element {
    let release_url = release.url();
    let year = release.best_release_year("Unknown");
    let track_count = maybe_plural(release.track_count, "track");
    cx.render(rsx! {
        div {
            class: "h-auto w-32 md:w-40 lg:w-48 m-6 md:m-8 lg:m-10",
            div {
                class: "object-contain mb-4",
                Link {
                    to: "{release_url}",
                    AlbumCover {
                        uri: release.release_cover_uri.as_deref(),
                    },
                },
            },
            div {
                class: "text-center",
                Link {
                    class: "text-lg",
                    to: "{release_url}",
                    "{release.display_title}",
                },
                br { },
                "{year}",
                br { },
                "{track_count}",
            },
        },
    })
}

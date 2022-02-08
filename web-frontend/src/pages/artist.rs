use crate::{
    client::{get_artist_response, ReleaseListItem},
    components::{AlbumCover, PageTitle, UserFacingError},
    page_div_classes,
    prelude::*,
    ring_flex_item_classes, storage,
    util::{maybe_plural, new_client},
};
use dioxus::router::{use_route, Link};

pub(crate) fn Artist(cx: Scope) -> Element {
    let artist_id = use_route(&cx)
        .segment::<String>("artist_id")
        .expect("artist_id parameter was not found in path somehow")
        .expect("artist_id parameter could not be parsed as a String");

    cx.render(rsx! {
        ArtistFromRoute {
            artist_id: artist_id,
        }
    })
}

#[inline_props]
fn ArtistFromRoute(cx: Scope, artist_id: String) -> Element {
    let artist = use_future(&cx, || {
        to_owned![artist_id];
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
                                class: DC![C.typ.text_center],
                                PageTitle {
                                    "{core.display_name}"
                                },
                            },
                            div {
                                class: format_args!("{}", page_div_classes()),
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
    let link_class = C![C.typ.text_lg];
    cx.render(rsx! {
        div {
            class: format_args!("{}", ring_flex_item_classes()),
            div {
                class: DC![C.lay.object_contain, C.spc.mb_4],
                Link {
                    to: "{release_url}",
                    AlbumCover {
                        uri: release.release_cover_uri.as_deref(),
                    },
                },
            },
            div {
                class: DC![C.typ.text_center],
                Link {
                    class: "{link_class}",
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

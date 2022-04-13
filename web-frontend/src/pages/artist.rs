use crate::{
    components::{AlbumCover, PageTitle, UserFacingError},
    grpc::crumb::{get_artist_response, ReleaseListItem},
    page_div_classes,
    prelude::*,
    ring_flex_item_classes, storage,
    util::{join_with_rsx, maybe_plural, new_client},
};
use dioxus::router::{use_route, Link};

pub(crate) fn Artist(cx: Scope) -> Element {
    let artist_id = use_route(&cx)
        .segment("artist_id")
        .expect("artist_id parameter was not found in path somehow");
    cx.render(rsx! {
        ArtistFromRoute {
            artist_id: artist_id,
        }
    })
}

#[inline_props]
fn ArtistFromRoute<'a>(cx: Scope<'a>, artist_id: &'a str) -> Element {
    let artist = use_future(&cx, (), |_| {
        let artist_id = artist_id.to_string();
        let mut client = new_client(
            cx.consume_context::<storage::Store>()
                .expect("Could not get Store from context"),
        );
        async move { client.get_artist(&artist_id).await }
    });

    let content = match artist.value() {
        Some(Ok(response)) => match &response.response_either {
            Some(get_artist_response::ResponseEither::Artist(artist)) => {
                let core = artist.core.as_ref().unwrap();
                let names = join_with_rsx(core.names().collect(), || {
                    rsx! { br { } }
                });
                rsx! {
                    div {
                        class: DC![C.typ.text_center],
                        PageTitle {
                            names
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
            }
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
        },
        Some(Err(e)) => {
            log::error!("Error loading artist: {}", e);
            rsx! {
                "Error loading artist"
            }
        }
        None => {
            rsx! {
                "Loading artist",
            }
        }
    };

    cx.render(rsx! { content })
}

#[inline_props]
fn OneRelease<'a>(cx: Scope, release: &'a ReleaseListItem) -> Element {
    let release_url = release.url();
    let year = release.best_release_year("Unknown");
    let track_count = maybe_plural(release.track_count, "track");
    let link_class = C![C.typ.text_lg];
    let titles = join_with_rsx(release.titles().collect(), || {
        rsx! { br { } }
    });
    cx.render(rsx! {
        div {
            class: format_args!("{}", ring_flex_item_classes()),
            div {
                class: DC![C.lay.object_contain, C.spc.mb_4],
                Link {
                    to: "{release_url}",
                    AlbumCover {
                        uri: release.release_cover_uri.as_deref().unwrap(),
                    },
                },
            },
            div {
                class: DC![C.typ.text_center],
                Link {
                    class: "{link_class}",
                    to: "{release_url}",
                    titles,
                },
                br { },
                "{year}",
                br { },
                "{track_count}",
            },
        },
    })
}

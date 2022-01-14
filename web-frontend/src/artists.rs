use crate::{
    client::{ArtistListItem, Client},
    components::AlbumCover,
};
use dioxus::{prelude::*, router::Link};

pub(crate) fn Artists(cx: Scope) -> Element {
    let artists = use_future(&cx, || async move { Client::new().get_artists().await });
    cx.render(rsx! {
        crate::Crumb {
            match artists.value() {
                Some(Ok(artists)) => rsx! {
                    div {
                        class: "flex flex-row flex-wrap justify-center",
                        artists.iter().map(|a| rsx!{
                            OneArtist {
                                key: "{a.artist_id}",
                                artist: a,
                            }
                        }),
                    },
                },
                Some(Err(e)) => {
                    log::error!("Error loading artists: {}", e);
                    rsx! {
                        "Error loading artists"
                    }
                },
                None => {
                    rsx! {
                        "Loading artists",
                    }
                },
            }
        }
    })
}

#[inline_props]
fn OneArtist<'a>(cx: Scope, artist: &'a ArtistListItem) -> Element {
    let release_count = crate::util::maybe_plural(artist.release_count, "release");
    let track_count = crate::util::maybe_plural(artist.track_count, "track");
    let artist_url = artist.url();
    cx.render(rsx! {
        div {
            class: "h-auto w-32 md:w-40 lg:w-48 m-6 md:m-8 lg:m-10",
            div {
                class: "object-contain mb-4",
                Link {
                    to: "{artist_url}",
                    AlbumCover {
                        uri: artist.album_cover_uri.as_deref(),
                        ring: true,
                    },
                },
            },
            div {
                class: "text-center",
                Link {
                    class: "text-lg",
                    to: "{artist_url}",
                    "{artist.display_name}",
                },
                [
                    rsx!{ br {} },
                    rsx!{ "{release_count}" },
                    rsx!{ br {} },
                    rsx!{ "{track_count}" },
                ]
            },
        },
    })
}

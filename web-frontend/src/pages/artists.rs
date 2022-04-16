use crate::{
    components::AlbumCover,
    grpc::crumb::ArtistListItem,
    page_div_classes,
    prelude::*,
    ring_flex_item_classes, storage,
    util::{join_with_rsx, maybe_plural, new_client},
};
use dioxus::router::Link;

pub(crate) fn Artists(cx: Scope) -> Element {
    let artists = use_future(&cx, (), |_| {
        let mut client = new_client(
            cx.consume_context::<storage::Store>()
                .expect("Could not get Store from context"),
        );
        async move { client.get_artists().await }
    });
    cx.render(rsx! {
        match artists.value() {
            Some(Ok(artists)) => rsx! {
                div {
                    class: format_args!("{}", page_div_classes()),
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
    })
}

#[inline_props]
fn OneArtist<'a>(cx: Scope, artist: &'a ArtistListItem) -> Element {
    let release_count = maybe_plural(artist.release_count, "release");
    let track_count = maybe_plural(artist.track_count, "track");
    let artist_url = artist.url();
    let link_class = C![C.typ.text_lg];
    let names = join_with_rsx(artist.names().collect(), || {
        rsx! { br { } }
    });
    cx.render(rsx! {
        div {
            class: format_args!("{}", ring_flex_item_classes()),
            div {
                class: DC![C.lay.object_contain, C.spc.mb_4],
                Link {
                    to: "{artist_url}",
                    AlbumCover {
                        uri: artist.release_cover_uri.as_deref(),
                        lazy: true,
                    },
                },
            },
            div {
                class: DC![C.typ.text_center],
                Link {
                    class: "{link_class}",
                    to: "{artist_url}",
                    names,
                },
                br { },
                "{release_count}",
                br { },
                "{track_count}",
            },
        },
    })
}

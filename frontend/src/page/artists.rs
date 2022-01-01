use crate::{
    client::{ArtistItem, Client},
    generated::css_classes::C,
    image_src, page_styles, RemoteData,
};
use seed::{prelude::*, *};

#[derive(Debug)]
pub struct Model {
    artists: RemoteData<Vec<ArtistItem>>,
}

#[derive(Debug)]
pub enum Msg {
    ArtistsFetched(Result<Vec<ArtistItem>, crate::client::Error>),

    LoadingMsg(crate::page::partial::loading::Msg),
}

pub fn init(_url: Url, orders: &mut impl Orders<Msg>) -> Model {
    let client = Client::new();
    orders.perform_cmd(async { Msg::ArtistsFetched(client.load_artists().await) });

    Model {
        artists: RemoteData::Loading,
    }
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ArtistsFetched(Ok(artists)) => model.artists = RemoteData::Loaded(artists),
        Msg::ArtistsFetched(Err(e)) => {
            log!("error loading artists: {}", e)
        }
        _ => (),
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    match &model.artists {
        RemoteData::Loaded(artists) => view_artists(artists),
        _ => crate::page::partial::loading::view().map_msg(Msg::LoadingMsg),
    }
}

fn view_artists(artists: &[ArtistItem]) -> Node<Msg> {
    section![
        C![page_styles()],
        div![
            C![C.flex, C.flex_row, C.flex_wrap, C.justify_center],
            artists.iter().map(|a| one_artist(a)),
        ]
    ]
}

fn one_artist(artist: &ArtistItem) -> Node<Msg> {
    let client = Client::new();
    div![
        C![C.h_auto, C.w_32, C.md__w_40, C.lg__w_48, C.m_6, C.md__m_8, C.lg__m_10],
        div![
            C![C.object_contain, C.mb_4],
            a![
                attrs! {
                    At::Href => &artist.url,
                },
                img![
                    C![
                        C.rounded_full,
                        C.ring_4,
                        C.ring_indigo_500,
                        C.ring_opacity_50,
                    ],
                    attrs! {
                        At::Src => image_src("Siip-cover.jpg"),
                    }
                ]
            ],
        ],
        div![
            C![C.text_center],
            a![
                C![C.text_lg],
                attrs! {
                    At::Href => &artist.url,
                },
                &artist.name,
            ],
            br![],
            crate::maybe_plural(artist.releases.len(), "release"),
            br![],
            crate::maybe_plural(
                artist
                    .releases
                    .values()
                    .map(|release_id| { client.release_by_id(release_id).unwrap().tracks.len() })
                    .reduce(|a, b| a + b)
                    .unwrap(),
                "track",
            )
        ]
    ]
}

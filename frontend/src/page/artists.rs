use crate::{
    album_cover,
    client::{ArtistListItem, Client},
    generated::css_classes::C,
    page_styles, RemoteData,
};
use seed::{prelude::*, *};

#[derive(Debug)]
pub struct Model {
    artists: RemoteData<Vec<ArtistListItem>>,
}

#[derive(Debug)]
pub enum Msg {
    ArtistsFetched(Result<Vec<ArtistListItem>, crate::client::Error>),
    LoadingMsg(crate::page::partial::loading::Msg),
    DummyMsg,
}

pub fn init(_url: Url, orders: &mut impl Orders<Msg>) -> Model {
    let mut client = Client::new();
    orders.perform_cmd(async move { Msg::ArtistsFetched(client.get_artists().await) });

    Model {
        artists: RemoteData::Loading,
    }
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    println!("update {:#?}", msg);
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

fn view_artists(artists: &[ArtistListItem]) -> Node<Msg> {
    section![
        C![page_styles()],
        div![
            C![C.flex, C.flex_row, C.flex_wrap, C.justify_center],
            artists.iter().map(|a| one_artist(a))
        ]
    ]
}

fn one_artist(artist: &ArtistListItem) -> Node<Msg> {
    div![
        C![C.h_auto, C.w_32, C.md__w_40, C.lg__w_48, C.m_6, C.md__m_8, C.lg__m_10],
        div![
            C![C.object_contain, C.mb_4],
            a![
                attrs! {
                    At::Href => &artist.url(),
                },
                album_cover(artist.album_cover_uri.as_deref()).map_msg(|_| Msg::DummyMsg),
            ],
        ],
        div![
            C![C.text_center],
            a![
                C![C.text_lg],
                attrs! {
                    At::Href => &artist.url(),
                },
                &artist.display_name,
            ],
            br![],
            crate::maybe_plural(artist.release_count, "release"),
            br![],
            crate::maybe_plural(artist.track_count, "track")
        ]
    ]
}

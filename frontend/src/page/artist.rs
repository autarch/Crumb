use crate::{
    generated::css_classes::C,
    image_src,
    models::{Album, Artist, ArtistId},
    page_styles, RemoteData,
};
use seed::{prelude::*, *};
use std::cmp::Ordering;
use uuid::Uuid;

#[derive(Debug)]
pub struct Model {
    artist: RemoteData<Option<Artist>>,
}

#[derive(Debug)]
pub enum Msg {
    ArtistFetched(Result<Option<Artist>, crate::client::Error>),

    LoadingMsg(crate::page::partial::loading::Msg),
}

pub fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Model {
    let client = &crate::OUR_CLIENT;

    orders.perform_cmd(async move {
        let artist_id = ArtistId::new(Uuid::parse_str(url.next_path_part().unwrap()).unwrap());
        Msg::ArtistFetched(client.load_artist_by_id(&artist_id).await)
    });

    Model {
        artist: RemoteData::Loading,
    }
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ArtistFetched(Ok(data)) => model.artist = RemoteData::Loaded(data),
        Msg::ArtistFetched(Err(e)) => {
            log!("error loading artist: {}", e)
        }
        _ => (),
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    match &model.artist {
        RemoteData::Loaded(artist) => match &artist {
            Some(artist) => view_artist(artist),
            None => Node::Empty,
        },
        _ => crate::page::partial::loading::view().map_msg(Msg::LoadingMsg),
    }
}

fn view_artist(artist: &Artist) -> Node<Msg> {
    let client = &crate::OUR_CLIENT;
    let mut albums = artist
        .albums
        .values()
        .filter_map(|album_id| client.album_by_id(album_id))
        .collect::<Vec<&Album>>();
    albums.sort_by(|a, b| {
        let cmp = a.release_year.partial_cmp(&b.release_year);
        match cmp {
            Some(Ordering::Equal) => a.title.partial_cmp(&b.title).unwrap(),
            Some(c) => c,
            // I don't think this can happen.
            None => Ordering::Equal,
        }
    });
    //    albums.sort
    section![
        C![page_styles()],
        h1![C![C.text_center, C.text_2xl], &artist.name],
        div![
            C![C.flex, C.flex_row, C.flex_wrap, C.justify_center],
            albums.iter().map(|&a| one_album(a)),
        ]
    ]
}

fn one_album(album: &Album) -> Node<Msg> {
    div![
        C![C.h_auto, C.w_32, C.md__w_40, C.lg__w_48, C.m_6, C.md__m_8, C.lg__m_10],
        div![
            C![C.object_contain, C.mb_4],
            a![
                attrs! {
                    At::Href => &album.url,
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
                    At::Href => &album.url,
                },
                h2![&album.title],
            ],
            format!("{}", album.release_year),
            br![],
            crate::maybe_plural(album.tracks.len(), "track",)
        ],
    ]
}

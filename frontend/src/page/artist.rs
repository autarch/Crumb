use crate::{
    client::{ArtistItem, Client, ReleaseItem},
    generated::css_classes::C,
    image_src, page_styles, RemoteData,
};
use seed::{prelude::*, *};
use uuid::Uuid;

#[derive(Debug)]
pub struct Model {
    artist: RemoteData<Option<ArtistItem>>,
}

#[derive(Debug)]
pub enum Msg {
    ArtistFetched(Result<Option<ArtistItem>, crate::client::Error>),

    LoadingMsg(crate::page::partial::loading::Msg),
}

pub fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Model {
    let client = Client::new();

    orders.perform_cmd(async move {
        let artist_id = Uuid::parse_str(url.next_path_part().unwrap()).unwrap();
        Msg::ArtistFetched(client.load_artist_by_id(&artist_id.to_string()).await)
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

fn view_artist(artist: &ArtistItem) -> Node<Msg> {
    let releases: Vec<ReleaseItem> = vec![];
    section![
        C![page_styles()],
        h1![C![C.text_center, C.text_2xl], &artist.name],
        div![
            C![C.flex, C.flex_row, C.flex_wrap, C.justify_center],
            releases.iter().map(|r| one_release(r)),
        ]
    ]
}

fn one_release(release: &ReleaseItem) -> Node<Msg> {
    div![
        C![C.h_auto, C.w_32, C.md__w_40, C.lg__w_48, C.m_6, C.md__m_8, C.lg__m_10],
        div![
            C![C.object_contain, C.mb_4],
            a![
                attrs! {
                    At::Href => &release.url,
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
                    At::Href => &release.url,
                },
                h2![&release.title],
            ],
            format!("{}", release.release_year),
            br![],
            crate::maybe_plural(release.tracks.len(), "track")
        ],
    ]
}

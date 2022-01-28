use crate::{
    release_cover,
    client::{get_artist_response, ArtistItem, Client, GetArtistResponse, ReleaseListItem},
    generated::css_classes::C,
    page_styles, view_error, RemoteData,
};
use seed::{prelude::*, *};
use uuid::Uuid;

#[derive(Debug)]
pub struct Model {
    artist: RemoteData<GetArtistResponse>,
}

#[derive(Debug)]
pub enum Msg {
    ArtistFetched(Result<GetArtistResponse, crate::client::Error>),
    LoadingMsg(crate::page::partial::loading::Msg),
    ErrorMsg,
    DummyMsg,
}

pub fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Model {
    let mut client = Client::new();
    orders.perform_cmd(async move {
        let artist_id = Uuid::parse_str(url.next_path_part().unwrap()).unwrap();
        Msg::ArtistFetched(client.get_artist(&artist_id.to_string()).await)
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
        RemoteData::Loaded(response) => match &response.response_either {
            Some(get_artist_response::ResponseEither::Artist(a)) => view_artist(a),
            Some(get_artist_response::ResponseEither::Error(e)) => {
                view_error(&e).map_msg(|_| Msg::ErrorMsg)
            }
            None => {
                log!("Empty response for GetArtist request!");
                return Node::Empty;
            }
        },
        _ => crate::page::partial::loading::view().map_msg(Msg::LoadingMsg),
    }
}

fn view_artist(artist: &ArtistItem) -> Node<Msg> {
    section![
        C![page_styles()],
        h1![
            C![C.text_center, C.text_2xl],
            &artist.core.as_ref().unwrap().display_name,
        ],
        div![
            C![C.flex, C.flex_row, C.flex_wrap, C.justify_center],
            artist.releases.iter().map(|r| one_release(r)),
        ]
    ]
}

fn one_release(release: &ReleaseListItem) -> Node<Msg> {
    div![
        C![C.h_auto, C.w_32, C.md__w_40, C.lg__w_48, C.m_6, C.md__m_8, C.lg__m_10],
        div![
            C![C.object_contain, C.mb_4],
            a![
                attrs! {
                    At::Href => &release.url(),
                },
                release_cover(release.release_cover_uri.as_deref()).map_msg(|_| Msg::DummyMsg),
            ],
        ],
        div![
            C![C.text_center],
            a![
                C![C.text_lg],
                attrs! {
                    At::Href => &release.url(),
                },
                h2![&release.display_title],
            ],
            release.best_release_year(),
            br![],
            crate::maybe_plural(release.track_count, "track")
        ],
    ]
}

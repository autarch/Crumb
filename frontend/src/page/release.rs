use crate::{
    album_cover,
    client::{
        get_release_response, Client, GetReleaseResponse, ReleaseItem, ReleaseListItem,
        ReleaseTrack,
    },
    generated::css_classes::C,
    page_styles, view_error, RemoteData,
};
use seed::{prelude::*, *};
use uuid::Uuid;

#[derive(Debug)]
pub struct Model {
    release: RemoteData<GetReleaseResponse>,
}

#[derive(Debug)]
pub enum Msg {
    ReleaseFetched(Result<GetReleaseResponse, crate::client::Error>),
    LoadingMsg(crate::page::partial::loading::Msg),
    ErrorMsg,
    DummyMsg,
}

pub fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Model {
    let mut client = Client::new();
    orders.perform_cmd(async move {
        let release_id = Uuid::parse_str(url.next_path_part().unwrap()).unwrap();
        Msg::ReleaseFetched(client.get_release(&release_id.to_string()).await)
    });

    Model {
        release: RemoteData::Loading,
    }
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ReleaseFetched(Ok(data)) => model.release = RemoteData::Loaded(data),
        Msg::ReleaseFetched(Err(e)) => {
            log!("error loading release: {}", e)
        }
        _ => (),
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    match &model.release {
        RemoteData::Loaded(response) => match &response.response_either {
            Some(get_release_response::ResponseEither::Release(a)) => view_release(a),
            Some(get_release_response::ResponseEither::Error(e)) => {
                view_error(&e).map_msg(|_| Msg::ErrorMsg)
            }
            None => {
                log!("Empty response for GetRelease request!");
                return Node::Empty;
            }
        },
        _ => crate::page::partial::loading::view().map_msg(Msg::LoadingMsg),
    }
}

fn view_release(release: &ReleaseItem) -> Node<Msg> {
    let core = release.core.as_ref().unwrap();
    let release_year = if let Some(year) = core.original_year.or(core.release_year) {
        year.to_string()
    } else {
        String::new()
    };
    section![
        C![page_styles()],
        div![
            C![C.flex, C.flex_row, C.flex_wrap, C.justify_center],
            album_cover(core.album_cover_uri.as_deref()).map_msg(|_| Msg::DummyMsg),
            div![
                h1![C![C.text_center, C.text_2xl], &core.display_title,],
                h2![release_year],
            ],
        ],
        div![
            C![C.flex, C.flex_row, C.flex_wrap, C.justify_center],
            release.tracks.iter().map(|t| one_track(core, t)),
        ]
    ]
}

fn one_track(release: &ReleaseListItem, track: &ReleaseTrack) -> Node<Msg> {
    div![
        C![C.h_auto, C.w_32, C.md__w_40, C.lg__w_48, C.m_6, C.md__m_8, C.lg__m_10],
        div![
            C![C.object_contain, C.mb_4],
            //            album_cover(release.album_cover_uri.as_deref()).map_msg(|_| Msg::DummyMsg),
        ],
        div![C![C.text_center, C.text_lg], h2![&track.display_title],],
    ]
}

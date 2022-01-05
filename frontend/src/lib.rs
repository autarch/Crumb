// @TODO: uncomment once https://github.com/rust-lang/rust/issues/54726 stable
//#![rustfmt::skip::macros(class)]

#![allow(
    clippy::used_underscore_binding,
    clippy::non_ascii_literal,
    clippy::enum_glob_use,
    clippy::must_use_candidate,
    clippy::wildcard_imports
)]

mod client;
mod components;
mod generated;
mod icons;
mod page;

use client::{ArtistItem, Client, ReleaseItem, ReleaseTrack};
use generated::css_classes::C;
use seed::{prelude::*, *};
use std::{cell::RefCell, rc::Rc};

const APP_NAME: &str = "Crumb";
// const STATIC_PATH: &str = "static";
const IMAGES_PATH: &str = "static/images";

const ARTISTS: &str = "artists";
const RELEASES: &str = "releases";
const TRACKS: &str = "tracks";
const QUEUE: &str = "queue";

const ARTIST: &str = "artist";
const RELEASE: &str = "release";
// const TRACK: &str = "track";

#[wasm_bindgen(start)]
pub fn run() {
    log!("Starting app...");

    App::start("app", init, update, view);

    log!("App started.");
}

#[derive(Debug)]
pub enum Page {
    Home(page::home::Model),
    Artists(page::artists::Model),
    Artist(page::artist::Model),
    Releases,
    Release(page::release::Model),
    Tracks,
    Queue(page::queue::Model),
    NotFound,
}

impl Page {
    fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Self {
        match url.next_path_part() {
            None => Self::Home(page::home::init(url, &mut orders.proxy(Msg::HomeMsg))),
            Some(ARTISTS) => {
                Self::Artists(page::artists::init(url, &mut orders.proxy(Msg::ArtistsMsg)))
            }
            Some(ARTIST) => {
                Self::Artist(page::artist::init(url, &mut orders.proxy(Msg::ArtistMsg)))
            }
            //            [RELEASES] => Self::Releases(page::releases::init(url, &mut orders.proxy(Msg::ReleasesMsg))),
            Some(RELEASE) => {
                Self::Release(page::release::init(url, &mut orders.proxy(Msg::ReleaseMsg)))
            }
            //            [TRACKS] => Self::Tracks(page::tracks::init(url, &mut orders.proxy(Msg::TracksMsg))),
            Some(QUEUE) => Self::Queue(page::queue::init(url, &mut orders.proxy(Msg::QueueMsg))),
            _ => Self::NotFound,
        }
    }
}

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url()
    }

    pub fn releases(self) -> Url {
        self.base_url().add_path_part(RELEASES)
    }

    pub fn release(self) -> Url {
        self.base_url().add_path_part(RELEASE)
    }

    pub fn artists(self) -> Url {
        self.base_url().add_path_part(ARTISTS)
    }

    pub fn artist(self) -> Url {
        self.base_url().add_path_part(ARTIST)
    }

    pub fn tracks(self) -> Url {
        self.base_url().add_path_part(TRACKS)
    }

    pub fn queue(self) -> Url {
        self.base_url().add_path_part(QUEUE)
    }
}

#[derive(Debug)]
pub struct Model {
    base_url: Url,
    page: Page,
    queue: Option<Rc<RefCell<Queue>>>,
}

#[derive(Debug)]
pub struct Queue {
    tracks: Vec<ReleaseTrack>,
    pub current: Option<CurrentQueueItem>,
    is_playing: bool,
}

#[derive(Debug)]
pub struct CurrentQueueItem {
    idx: usize,
    artist: ArtistItem,
    release: ReleaseItem,
}

#[derive(Debug)]
pub struct QueueItem<'a> {
    position: u32,
    track: &'a ReleaseTrack,
}

impl Queue {
    fn new(tracks: Vec<ReleaseTrack>, current_idx: usize, is_playing: bool) -> Self {
        return Self {
            tracks: vec![],
            current: None,
            is_playing: false,
        };

        // let client = Client::new();
        // let release = client.get_release("XXX").unwrap().clone();
        // let artist = client
        //     .get_artist(&release.core.as_ref().unwrap().primary_artist_id)
        //     .unwrap()
        //     .clone();

        // Self {
        //     tracks,
        //     current: Some(CurrentQueueItem {
        //         idx: current_idx,
        //         artist,
        //         release,
        //     }),
        //     is_playing,
        // }
    }

    fn current_track(&self) -> Option<&ReleaseTrack> {
        match self.current {
            Some(CurrentQueueItem { idx: i, .. }) => Some(&self.tracks[i]),
            None => None,
        }
    }

    fn move_to_next(&mut self) {
        if self.tracks.is_empty() {
            return;
        }

        self.set_current_state(match self.current {
            Some(CurrentQueueItem { idx: i, .. }) => i as i64 + 1,
            None => 1,
        });
    }

    fn move_to_previous(&mut self) {
        if self.tracks.is_empty() {
            return;
        }

        self.set_current_state(match self.current {
            Some(CurrentQueueItem { idx: i, .. }) => i as i64 - 1,
            None => 1,
        });
    }

    fn set_current_state(&mut self, idx: i64) {
        // if idx < 0 {
        //     self.current = None;
        //     return;
        // }

        // if let Some(current_track) = &self.tracks.get(idx as usize) {
        //     let client = Client::new();
        //     let release = client
        //         .get_release(&current_track.release_id)
        //         .unwrap()
        //         .clone();
        //     let artist = client
        //         .get_artist(&release.core.as_ref().unwrap().primary_artist_id)
        //         .unwrap()
        //         .clone();
        //     self.current = Some(CurrentQueueItem {
        //         idx: idx as usize,
        //         release,
        //         artist,
        //     });
        // } else {
        //     self.current = None
        // }
        // if self.current.is_none() {
        //     self.is_playing = false;
        // }
    }

    fn toggle_is_playing(&mut self) {
        self.is_playing = !self.is_playing;
    }

    fn visible_items(&self) -> Option<impl Iterator<Item = QueueItem>> {
        match self.current {
            Some(CurrentQueueItem { idx, .. }) => {
                let range = idx..self.tracks.len();
                Some(
                    self.tracks[range]
                        .iter()
                        .enumerate()
                        .map(move |(i, track)| QueueItem {
                            position: (idx + i + 1) as u32,
                            track,
                        }),
                )
            }
            None => None,
        }
    }

    fn can_move_to_next(&self) -> bool {
        if self.is_empty() {
            return false;
        }
        match self.current {
            Some(CurrentQueueItem { idx, .. }) => idx < self.tracks.len() - 1,
            None => false,
        }
    }

    fn can_move_to_previous(&self) -> bool {
        if self.is_empty() {
            return false;
        }
        match self.current {
            Some(CurrentQueueItem { idx, .. }) => idx != 0,
            None => true,
        }
    }

    fn can_play(&self) -> bool {
        self.has_visible_tracks()
    }

    fn is_empty(&self) -> bool {
        self.tracks.is_empty()
    }

    fn has_visible_tracks(&self) -> bool {
        self.current.is_some() && !self.is_empty()
    }
}

pub enum Msg {
    UrlChanged(subs::UrlChanged),

    QueueFetched(Result<Vec<ReleaseTrack>, client::Error>),

    ArtistsMsg(page::artists::Msg),
    ArtistMsg(page::artist::Msg),
    ReleaseMsg(page::release::Msg),
    HomeMsg(page::home::Msg),
    QueueMsg(page::queue::Msg),
    NotFoundMsg(page::not_found::Msg),

    MenuMsg(page::partial::menu::Msg),
    NowPlayingMsg(page::partial::now_playing::Msg),
}

#[derive(Clone, Debug)]
pub enum RemoteData<T> {
    NotStarted,
    Loading,
    Loaded(T),
}

impl<T> RemoteData<T> {
    pub fn loaded(&self) -> Option<&T> {
        if let Self::Loaded(data) = self {
            Some(data)
        } else {
            None
        }
    }

    pub fn loaded_mut(&mut self) -> Option<&mut T> {
        if let Self::Loaded(data) = self {
            Some(data)
        } else {
            None
        }
    }
}

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);
    let client = Client::new();
    orders.perform_cmd(async move { Msg::QueueFetched(client.fake_queue().await) });

    Model {
        base_url: url.to_base_url(),
        page: Page::init(url.clone(), orders),
        queue: None,
    }
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.page = Page::init(url, orders);
            if let Page::Queue(_) = model.page {
                if let Some(queue) = &model.queue {
                    let queue_orders = &mut orders.proxy(Msg::QueueMsg);
                    let queue_clone = queue.clone();
                    queue_orders.perform_cmd(async { page::queue::Msg::QueueFetched(queue_clone) });
                }
            }
        }
        Msg::QueueFetched(Ok(tracks)) => {
            let queue = Rc::new(RefCell::new(Queue::new(tracks, 0, false)));
            let queue_clone = queue.clone();
            model.queue = Some(queue);

            let queue_orders = &mut orders.proxy(Msg::QueueMsg);
            queue_orders.perform_cmd(async { page::queue::Msg::QueueFetched(queue_clone) });
        }
        Msg::QueueFetched(Err(e)) => {
            log!("error loading queue: {}", e)
        }
        Msg::NowPlayingMsg(page::partial::now_playing::Msg::TogglePlayPause) => {
            let mut queue = match &model.queue {
                None => return,
                Some(q) => q.borrow_mut(),
            };
            queue.toggle_is_playing();
        }
        Msg::NowPlayingMsg(crate::page::partial::now_playing::Msg::PreviousTrack) => {
            let mut queue = match &model.queue {
                None => return,
                Some(q) => q.borrow_mut(),
            };
            queue.move_to_previous();
        }
        Msg::NowPlayingMsg(crate::page::partial::now_playing::Msg::NextTrack) => {
            let mut queue = match &model.queue {
                None => return,
                Some(q) => q.borrow_mut(),
            };
            queue.move_to_next();
        }
        Msg::ArtistsMsg(msg) => {
            if let Page::Artists(model) = &mut model.page {
                page::artists::update(msg, model, &mut orders.proxy(Msg::ArtistsMsg))
            }
        }
        Msg::ArtistMsg(msg) => {
            if let Page::Artist(model) = &mut model.page {
                page::artist::update(msg, model, &mut orders.proxy(Msg::ArtistMsg))
            }
        }
        Msg::ReleaseMsg(msg) => {
            if let Page::Release(model) = &mut model.page {
                page::release::update(msg, model, &mut orders.proxy(Msg::ReleaseMsg))
            }
        }
        Msg::QueueMsg(msg) => {
            if let Page::Queue(model) = &mut model.page {
                page::queue::update(msg, model, &mut orders.proxy(Msg::QueueMsg))
            }
        }
        _ => (),
    }
}

pub fn view(model: &Model) -> impl IntoNodes<Msg> {
    let queue = match &model.queue {
        Some(queue) => Some(queue.borrow()),
        None => None,
    };
    let menu = page::partial::menu::view(&model.page, &model.base_url);
    let main = view_main(&model);
    div![
        C![C.min_h_screen, C.bg_blue_50, C.font_sans],
        menu.map_msg(Msg::MenuMsg),
        main,
        page::partial::now_playing::view(queue).map_msg(Msg::NowPlayingMsg),
    ]
}

fn view_main(model: &Model) -> Node<Msg> {
    main![
        C![
            C.pt_12, // Must match the height of the top menu.
            C.pb_24, // The height of the now playing div.
        ],
        match &model.page {
            Page::Home(model) => page::home::view(model).map_msg(Msg::HomeMsg),
            Page::Artists(model) => page::artists::view(model).map_msg(Msg::ArtistsMsg),
            Page::Artist(model) => page::artist::view(model).map_msg(Msg::ArtistMsg),
            //            Page::Releases(model) => page::releases::view(model),
            Page::Release(model) => page::release::view(model).map_msg(Msg::ReleaseMsg),
            //            Page::Tracks(model) => page::tracks::view(model),
            Page::Queue(model) => page::queue::view(model).map_msg(Msg::QueueMsg),
            Page::NotFound => page::not_found::view().map_msg(Msg::NotFoundMsg),
            _ => page::not_found::view().map_msg(Msg::NotFoundMsg),
        },
    ]
}

pub(crate) fn image_src(image: &str) -> String {
    format!("{}/{}", IMAGES_PATH, image)
}

// pub(crate) fn asset_path(asset: &str) -> String {
//     format!("{}/{}", STATIC_PATH, asset)
// }

pub(crate) fn page_styles() -> Vec<&'static str> {
    vec![C.flex_grow, C.p_4]
}

pub(crate) fn maybe_plural(count: u32, noun: &'static str) -> String {
    match count {
        1 => format!("1 {}", noun),
        _ => format!("{} {}s", count, noun),
    }
}

pub(crate) fn view_error(error: &crate::client::Status) -> Node<Msg> {
    section![
        C![page_styles()],
        h1![C![C.text_center, C.text_2xl], "Error"],
        div![
            C![C.flex, C.flex_row, C.flex_wrap, C.justify_center],
            &error.message,
        ]
    ]
}

pub(crate) fn album_cover(uri: Option<&str>) -> Node<Msg> {
    img![
        C![
            C.rounded_full,
            C.ring_4,
            C.ring_indigo_500,
            C.ring_opacity_50,
        ],
        attrs! {
            At::Src => match uri {
                // All the thumb sized images are JPEGs, I think.
                Some(u) => u.replace(".jpg", "_thumb250.jpg").replace(".png", "_thumb250.jpg").replace(".gif", "_thumb250.jpg"),
                None => "https://dummyimage.com/200x200/fff/aaa".to_string(),
            },
        }
    ]
}

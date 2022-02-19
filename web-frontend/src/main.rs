#![allow(non_snake_case)]

mod client;
mod components;
mod css;
mod menu;
mod models;
mod now_playing;
mod pages;
mod storage;
mod usehighlanders;
mod util;

mod prelude {
    pub(crate) use crate::css::*;
    pub(crate) use dioxus::prelude::*;
}

use crate::prelude::*;
use crate::{pages::*, util::new_client};
use dioxus::router::*;
use futures_util::StreamExt;
use models::Queue;
use usehighlanders::use_highlanders;

type QueueFetchResult = Result<Queue, client::Error>;
type QueueRecvResult = Result<QueueFetchResult, futures_channel::mpsc::TryRecvError>;

struct QueueUpdate(QueueFetchResult, bool);

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    dioxus::web::launch(App)
}

fn App(cx: Scope) -> Element {
    cx.use_hook(|_| cx.provide_context(storage::Store::default()));

    let (is_playing, set_is_playing) = use_state(&cx, || false);
    let (queue, set_queue) = use_state(&cx, || None);
    let queue_tx = cx.use_hook(|_| {
        let (tx, mut rx) = futures_channel::mpsc::channel::<QueueUpdate>(3);
        let mut client = new_client(
            *cx.consume_context::<storage::Store>()
                .expect("Could not get Store from context"),
        );

        cx.spawn({
            to_owned![set_is_playing, set_queue, tx];
            async move {
                let queue = client.get_queue().await;
                if let Err(e) = tx.try_send(QueueUpdate(queue, false)) {
                    log::error!("Error sending initial load of queue to channel: {}", e);
                    return;
                }

                loop {
                    match rx.next().await {
                        Some(QueueUpdate(queue, start_playing)) => {
                            set_queue(Some(Ok(queue)));
                            if start_playing {
                                set_is_playing(true);
                            }
                        }
                        None => {
                            log::error!("Queue update channel was closed");
                            break;
                        }
                    }
                }
            }
        });

        tx
    });

    let context_menus = use_highlanders(&cx);

    let page_classes = css::Classes::builder()
        .classes(C![C.spc.pt_20, C.spc.pb_16, C.siz.h_full])
        .with_standard_padding(true)
        .build();

    cx.render(rsx! {
        Router {
            div {
                class: DC![C.typ.font_sans],
                menu::Menu {},
                section {
                    class: "{page_classes}",
                    div {
                        // This padding is necessary to give the page some
                        // breathing room above the NowPlaying component.
                        class: DC![C.spc.pb_24],
                        CrumbRoutes {
                            queue: queue,
                            queue_tx: queue_tx.clone(),
                            is_playing: is_playing,
                            set_is_playing: set_is_playing,
                        },
                    },
                    now_playing::NowPlaying {
                        queue: queue,
                        queue_tx: queue_tx.clone(),
                        is_playing: is_playing,
                        set_is_playing: set_is_playing,
                        context_menus: context_menus.clone(),
                    },
                },
            },
        },
    })
}

#[inline_props]
fn CrumbRoutes<'a>(
    cx: Scope,
    queue: &'a Option<QueueRecvResult>,
    queue_tx: futures_channel::mpsc::Sender<QueueUpdate>,
    is_playing: &'a bool,
    set_is_playing: &'a UseState<bool>,
) -> Element {
    cx.render(rsx! {
        Route {
            to: "/",
            Home { },
        },
        Route {
            to: "/artists",
            artists::Artists { },
        },
        Route {
            to: "/artist/:artist_id",
            artist::Artist { },
        },
        Route {
            to: "/releases",
            Releases { },
        },
        Route {
            to: "/release/:release_id",
            release::Release {
                queue_tx: queue_tx.clone(),
            },
        },
        Route {
            to: "/tracks",
            Tracks { },
        },
        Route {
            to: "/queue",
            queue::Queue {
                queue: queue,
                queue_tx: queue_tx.clone(),
                is_playing: is_playing,
                set_is_playing: set_is_playing,
            },
        },
    })
}

fn Home<'a>(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Home" },
    })
}

fn Releases<'a>(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Releases" },
    })
}

fn Tracks<'a>(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Tracks" },
    })
}

fn page_div_classes() -> String {
    C![
        C.lay.flex,
        C.fg.flex_row,
        C.fg.flex_wrap,
        C.fg.place_content_center,
    ]
}

fn ring_flex_item_classes() -> String {
    C![
        C.siz.h_auto,
        C.siz.w_32,
        M![M.md, C.siz.w_40],
        M![M.lg, C.siz.w_48],
        C.spc.m_6,
        M![M.md, C.spc.m_8],
        M![M.lg, C.spc.m_10],
    ]
}

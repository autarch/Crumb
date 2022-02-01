#![allow(non_snake_case)]

mod client;
mod components;
mod css;
mod menu;
mod models;
mod now_playing;
mod pages;
mod storage;
mod util;

use crate::{pages::*, util::new_client};
use dioxus::{prelude::*, router::*};
use models::Queue;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    dioxus::web::launch(App)
}

type QueueFetchResult = Result<Queue, client::Error>;
type QueueRecvResult = Result<QueueFetchResult, async_channel::RecvError>;

fn App(cx: Scope) -> Element {
    cx.use_hook(|_| cx.provide_context(storage::Store::default()));

    let (queue, set_queue) = use_state(&cx, || None);
    let queue_tx = cx.use_hook(|_| {
        let (tx, rx) = async_channel::bounded::<QueueFetchResult>(3);
        let mut client = new_client(
            *cx.consume_context::<storage::Store>()
                .expect("Could not get Store from context"),
        );

        cx.spawn({
            to_owned![set_queue, tx];
            async move {
                let queue = client.get_queue().await;
                if let Err(e) = tx.send(queue).await {
                    log::error!("Error sending initial load of queue to channel: {}", e);
                    return;
                }

                loop {
                    match rx.recv().await {
                        Ok(msg) => set_queue(Some(Ok(msg))),
                        Err(e) => {
                            log::error!("Channel was closed: {}", e);
                            break;
                        }
                    }
                }
            }
        });

        tx
    });

    let (is_playing, set_is_playing) = use_state(&cx, || false);

    let page_classes = css::Classes::builder()
        .classes("pt-20 pb-16 h-full")
        .with_standard_padding(true)
        .build();
    cx.render(rsx! {
        Router {
            div {
                class: "font-sans",
                menu::Menu {},
                section {
                    class: "{page_classes}",
                    div {
                        // This padding is necessary to give the page some
                        // breathing room above the NowPlaying component.
                        class: "pb-24",
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
    queue_tx: async_channel::Sender<QueueFetchResult>,
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
            Queue {
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

#[inline_props]
fn Queue<'a>(
    cx: Scope,
    queue: &'a Option<QueueRecvResult>,
    queue_tx: async_channel::Sender<QueueFetchResult>,
    is_playing: &'a bool,
    set_is_playing: &'a UseState<bool>,
) -> Element {
    cx.render(rsx! {
        h1 { "Queue" },
    })
}

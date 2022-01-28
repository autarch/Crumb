#![allow(non_snake_case)]

mod client;
mod components;
mod css;
mod icons;
mod menu;
mod models;
mod now_playing;
mod pages;
mod util;

use crate::{pages::*, util::new_client};
use dioxus::{prelude::*, router::*};
use models::Queue;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    dioxus::web::launch(App)
}

pub(crate) type QueueFetchResult = Result<Queue, client::Error>;
pub(crate) type QueueRecvResult = Result<QueueFetchResult, async_channel::RecvError>;

fn App(cx: Scope) -> Element {
    // Is 3 the right number? I have no clue!
    let (tx, rx) = async_channel::bounded::<QueueFetchResult>(3);
    let (_, _tx_holder) = use_state(&cx, || tx.clone());

    let (queue, set_queue) = use_state(&cx, || None);
    use_future(&cx, || {
        to_owned![set_queue];
        async move {
            loop {
                match rx.recv().await {
                    Ok(msg) => set_queue(Some(Ok(msg))),
                    Err(e) => {
                        log::debug!("Channel was closed: {}", e);
                        break;
                    }
                }
            }
        }
    });

    let tx_clone = tx.clone();
    cx.spawn(async move {
        let queue = new_client().get_queue().await;
        if let Err(e) = tx_clone.send(queue).await {
            log::error!("Error sending initial load of queue to channel: {}", e);
        }
    });

    let (_, is_playing) = use_state(&cx, || false);
    cx.render(rsx! {
        Router {
            Route {
                to: "/",
                Crumb {
                    page: Page::Home,
                    queue: queue,
                    queue_tx: tx.clone(),
                    is_playing: is_playing.clone(),
                }
            },
            Route {
                to: "/artists",
                Crumb {
                    page: Page::Artists,
                    queue: queue,
                    queue_tx: tx.clone(),
                    is_playing: is_playing.clone(),
                }
            },
            Route {
                to: "/artist/:artist_id",
                Crumb {
                    page: Page::Artist,
                    queue: queue,
                    is_playing: is_playing.clone(),
                    queue_tx: tx.clone(),
                }
            },
            Route {
                to: "/releases",
                Crumb {
                    page: Page::Releases,
                    queue: queue,
                    queue_tx: tx.clone(),
                    is_playing: is_playing.clone(),
                }
            },
            Route {
                to: "/release/:release_id",
                Crumb {
                    page: Page::Release,
                    queue: queue,
                    queue_tx: tx.clone(),
                    is_playing: is_playing.clone(),
                }
            },
            Route {
                to: "/tracks",
                Crumb {
                    page: Page::Tracks,
                    queue: queue,
                    queue_tx: tx.clone(),
                    is_playing: is_playing.clone(),
                },
            },
            Route {
                to: "/queue",
                Crumb {
                    page: Page::Queue,
                    queue: queue,
                    queue_tx: tx.clone(),
                    is_playing: is_playing.clone(),
                }
            },
        },
    })
}

#[inline_props]
fn Crumb<'a>(
    cx: Scope,
    page: Page,
    queue: &'a Option<QueueRecvResult>,
    queue_tx: async_channel::Sender<QueueFetchResult>,
    is_playing: &'a UseState<bool>,
) -> Element {
    let main = match page {
        Page::Home => rsx! {
            Home {
            },
        },
        Page::Artists => rsx! {
            artists::Artists {
            },
        },
        Page::Artist => rsx! {
            artist::Artist {
            },
        },
        Page::Artists => rsx! {
            artists::Artists {
            },
        },
        Page::Artist => rsx! {
            artist::Artist {
            },
        },
        Page::Releases => rsx! {
            Releases {
            },
        },
        Page::Release => rsx! {
            release::Release {
                queue: queue,
                queue_tx: queue_tx.clone(),
            },
        },
        Page::Tracks => rsx! {
            Tracks {
            },
        },
        Page::Queue => rsx! {
            Queue {
                queue: queue,
                queue_tx: queue_tx.clone(),
                is_playing: is_playing,
            },
        },
    };

    let page_classes = css::Classes::builder()
        .classes("pt-20 pb-16 h-full")
        .with_standard_padding(true)
        .build();
    cx.render(rsx! {
        div {
            class: "font-sans",
            menu::Menu {},
            section {
                class: format_args!("{}", page_classes),
                div {
                    // This padding is necessary to give the page some
                    // breathing room above the NowPlaying component.
                    class: "pb-24",
                    main,
               },
            },
            now_playing::NowPlaying {
                queue: queue,
                queue_tx: queue_tx.clone(),
                is_playing: is_playing,
            },
        }
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
    is_playing: &'a UseState<bool>,
) -> Element {
    cx.render(rsx! {
        h1 { "Queue" },
    })
}

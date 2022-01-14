#![allow(non_snake_case)]

mod artist;
mod artists;
mod client;
mod components;
mod css;
mod icons;
mod menu;
mod models;
mod now_playing;
mod release;
mod util;

use client::Client;
use dioxus::{prelude::*, router::*};
use models::Queue;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    dioxus::web::launch(App)
}

fn App(cx: Scope) -> Element {
    let queue = use_future(&cx, || async move {
        // Ok(Queue::new(vec![], 0, None, None, false))
        client::Client::new().get_queue().await
    });

    cx.render(rsx! {
        Router {
            Route { to: "/", Home {} }
            Route { to: "/artists", artists::Artists {} },
            Route { to: "/artist/:artist_id", artist::Artist {} },
            Route { to: "/releases", Releases {} },
            Route { to: "/release/:release_id", release::Release {} },
            Route { to: "/tracks", Tracks {} },
            Route { to: "/queue", Queue { } },
        },
        now_playing::NowPlaying {
            queue: queue
        },
    })
}

fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        Crumb {
            h1 { "Home" },
        }
    })
}

fn Releases(cx: Scope) -> Element {
    cx.render(rsx! {
        Crumb {
            h1 { "Releases" },
        }
    })
}

fn Tracks(cx: Scope) -> Element {
    cx.render(rsx! {
        Crumb {
            h1 { "Tracks" },
        }
    })
}

#[inline_props]
fn Queue(cx: Scope, //, queue: &'a UseFuture<Result<Queue, client::Error>>
) -> Element {
    cx.render(rsx! {
        Crumb {
            h1 { "Queue" },
        }
    })
}

#[inline_props]
fn Crumb<'a>(cx: Scope, children: Element<'a>) -> Element {
    let page_classes = css::Classes::builder()
        .classes("bg-blue-50 pt-14 pb-16 h-full")
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
                    children,
                },
            },
        }
    })
}

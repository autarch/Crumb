use crate::{client::Client, prelude::*, storage};
use dioxus::prelude::LazyNodes;
use gloo_utils::document;
use uuid::Uuid;
use wasm_bindgen::JsCast;

pub(crate) fn format_time(time: u32) -> String {
    let hours = time / 3600;
    let min = (time - (hours * 3600)) / 60;
    let sec = time & 60;

    if hours > 0 {
        format!("{hours}:{min:02}:{sec:02}")
    } else {
        format!("{min}:{sec:02}")
    }
}

pub(crate) fn maybe_plural(count: u32, noun: &'static str) -> String {
    match count {
        1 => format!("1 {}", noun),
        _ => format!("{} {}s", count, noun),
    }
}

pub(crate) fn new_client(s: storage::Store) -> Client<grpc_web_client::Client> {
    let id = match s.get("client-id") {
        Ok(Some(id)) => id,
        Ok(None) => {
            let id = new_id();
            s.set("client-id", &id)
                .expect("Could not set client-id key in local storage");
            id
        }
        Err(_) => {
            panic!("unreachable")
        }
    };
    Client::new(id)
}

fn new_id() -> String {
    Uuid::new_v4().to_string()
}

pub(crate) fn get_element<T: JsCast>(id: &str) -> Option<T> {
    match document().get_element_by_id(id) {
        Some(a) => match a.dyn_into::<T>() {
            Ok(a) => Some(a),
            Err(_) => {
                log::error!("Could not cast element with id {} into requested type", id);
                None
            }
        },
        None => {
            log::error!("Could not get {} element from web-sys document", id);
            None
        }
    }
}

pub(crate) fn join_with_rsx<'a>(
    items: Vec<&'a str>,
    joiner: impl Fn() -> LazyNodes<'a, 'a> + 'a,
) -> impl Iterator<Item = LazyNodes<'a, 'a>> {
    let len = items.len();
    items.into_iter().enumerate().map(move |(i, item)| {
        if i == len - 1 {
            rsx! { "{item}"  }
        } else {
            let j = joiner();
            rsx! { "{item}" j }
        }
    })
}

use crate::client::Client;
use gloo_storage::{LocalStorage, Storage};
use uuid::Uuid;

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

pub(crate) fn new_client() -> Client<grpc_web_client::Client> {
    let id = match LocalStorage::get("client-id") {
        Ok(id) => id,
        Err(_) => {
            let id = new_id();
            LocalStorage::set("client-id", &id)
                .expect("Could not set client-id key in local storage");
            id
        }
    };
    Client::new(id)
}

fn new_id() -> String {
    Uuid::new_v4().to_string()
}

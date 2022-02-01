use crate::{client::Client, storage};
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

pub(crate) fn new_client(s: storage::Store) -> Client<grpc_web_client::Client> {
    let id = match s.get("client-id") {
        Ok(Some(id)) => id,
        Ok(None) => {
            let id = new_id();
            s.set("client-id", &id)
                .expect("Could not set client-id key in local storage");
            id
        }
        Err(e) => {
            panic!("unreachable")
        }
    };
    Client::new(id)
}

fn new_id() -> String {
    Uuid::new_v4().to_string()
}

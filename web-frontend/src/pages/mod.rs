pub(crate) mod artist;
pub(crate) mod artists;
pub(crate) mod release;

pub(crate) enum Page {
    Home,
    Artists,
    Artist,
    Releases,
    Release,
    Tracks,
    Queue,
}

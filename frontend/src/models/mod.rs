use std::{collections::HashMap, fmt};
use uuid::Uuid;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ArtistId(Uuid);
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct AlbumId(Uuid);
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TrackId(Uuid);

impl ArtistId {
    pub fn new(id: Uuid) -> ArtistId {
        ArtistId(id)
    }
}

impl fmt::Display for ArtistId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AlbumId {
    pub fn new(id: Uuid) -> AlbumId {
        AlbumId(id)
    }
}

impl fmt::Display for AlbumId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TrackId {
    pub fn new(id: Uuid) -> TrackId {
        TrackId(id)
    }
}

impl fmt::Display for TrackId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct Artist {
    pub artist_id: ArtistId,
    pub name: String,
    pub url: String,
    pub albums: HashMap<String, AlbumId>,
}

#[derive(Clone, Debug)]
pub struct Album {
    pub album_id: AlbumId,
    pub title: String,
    pub url: String,
    pub release_year: u16,
    pub artist_id: ArtistId,
    pub tracks: Vec<Track>,
}

#[derive(Clone, Debug)]
pub struct Track {
    pub track_id: TrackId,
    pub title: String,
    pub track_num: u16,
    pub album_id: AlbumId,
}

impl Album {
    pub fn cover_image_url(&self) -> String {
        format!("{}-cover.jpg", self.title)
    }
}

mod data;

use crate::models::*;
use rand::Rng;
use serde::Deserialize;
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

#[derive(Deserialize)]
struct ParsedArtist {
    albums: Vec<String>,
}

struct RawArtist {
    name: String,
    albums: Vec<String>,
}

#[derive(Deserialize)]
struct RawAlbum {
    artist: String,
    release_date: String,
    tracks: Vec<RawTrack>,
}

#[derive(Deserialize)]
struct RawTrack {
    title: String,
    track_num: u16,
}

#[derive(Debug)]
pub struct Client {
    artists: HashMap<ArtistId, Artist>,
    albums: HashMap<AlbumId, Album>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("could not load data from remote host")]
    DataLoadError,
}

impl Client {
    pub fn new() -> Client {
        let raw_artists = parse_artists();
        let albums = parse_albums();

        Client {
            artists: raw_artists
                .into_iter()
                .map(|(artist_uuid, artist)| {
                    let artist = inflate_artist(artist_uuid, artist);
                    (artist.artist_id.clone(), artist)
                })
                .collect::<HashMap<ArtistId, Artist>>(),
            albums,
        }
    }

    pub fn artist_by_name(&self, name: &str) -> Option<&Artist> {
        let artist_id = ArtistId::new(artist_uuid(name));
        self.artists.get(&artist_id)
    }

    pub fn artist_by_id(&self, artist_id: &ArtistId) -> Option<&Artist> {
        self.artists.get(artist_id)
    }

    pub async fn load_artist_by_id(&self, artist_id: &ArtistId) -> Result<Option<Artist>, Error> {
        self.delay().await;
        match self.artists.get(artist_id) {
            Some(a) => Ok(Some(a.clone())),
            None => Ok(None),
        }
    }

    pub async fn load_artists(&self) -> Result<Vec<Artist>, Error> {
        self.delay().await;
        let mut artists = self
            .artists
            .values()
            .map(|a| a.clone())
            .collect::<Vec<Artist>>();
        artists.sort_by_key(|a| a.name.to_lowercase());
        Ok(artists)
    }

    pub fn album_by_name(&self, name: &str) -> Option<&Album> {
        let album_id = AlbumId::new(album_uuid(name));
        self.albums.get(&album_id)
    }

    pub fn album_by_id(&self, album_id: &AlbumId) -> Option<&Album> {
        self.albums.get(album_id)
    }

    pub async fn fake_queue(&self) -> Result<Vec<Track>, Error> {
        self.delay().await;

        Ok(vec![
            self.tracks_for("Ryokuoshokushakai", "SINGALONG"),
            self.tracks_for("Siip", "Siip"),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<Track>>())
    }

    fn tracks_for(&self, artist_name: &str, album_title: &str) -> Vec<Track> {
        let artist = self
            .artist_by_name(artist_name)
            .expect(&format!("Could not find artist named {}", artist_name));
        let album = self
            .album_by_id(artist.albums.get(album_title).expect(&format!(
                "Could not find album named {} by {}",
                album_title, artist_name,
            )))
            .expect(&format!("Could not look up {} by album_id", album_title));
        album.tracks.clone()
    }

    async fn delay(&self) {
        fluvio_wasm_timer::Delay::new(std::time::Duration::from_millis(
            rand::thread_rng().gen_range(50..1000),
        ))
        .await
        .unwrap();
    }
}

fn parse_artists() -> HashMap<Uuid, RawArtist> {
    let parsed: HashMap<String, ParsedArtist> =
        serde_json::from_str(data::ARTISTS_JSON).expect("Could not parse artists JSON");
    let mut artists = HashMap::new();
    for (name, p) in parsed {
        let artist_id = artist_uuid(&name);
        artists.insert(
            artist_id,
            RawArtist {
                name,
                albums: p.albums,
            },
        );
    }
    artists
}

fn artist_uuid(name: impl AsRef<str>) -> Uuid {
    Uuid::new_v5(
        &Uuid::new_v5(&Uuid::NAMESPACE_OID, "Artist".as_bytes()),
        name.as_ref().as_bytes(),
    )
}

fn parse_albums() -> HashMap<AlbumId, Album> {
    let parsed: HashMap<String, RawAlbum> =
        serde_json::from_str(data::ALBUMS_JSON).expect("Could not parse albums JSON");
    let mut albums = HashMap::new();
    for (title, p) in parsed {
        let album = inflate_album(title, p);
        albums.insert(album.album_id.clone(), album);
    }
    albums
}

fn album_uuid(name: impl AsRef<str>) -> Uuid {
    Uuid::new_v5(
        &Uuid::new_v5(&Uuid::NAMESPACE_OID, "Album".as_bytes()),
        name.as_ref().as_bytes(),
    )
}

fn inflate_album(title: String, album: RawAlbum) -> Album {
    let album_id = AlbumId::new(album_uuid(&title));
    let tracks = album
        .tracks
        .into_iter()
        .map(|track| inflate_track(track, &album_id))
        .collect::<Vec<Track>>();
    let url = format!("/album/{}", album_id);
    let release_year = album
        .release_date
        .split('-')
        .next()
        .expect("Release date should have at least the year")
        .parse::<u16>()
        .expect("Release date's year should parse as a u16");
    Album {
        album_id,
        title,
        url,
        release_year,
        artist_id: ArtistId::new(artist_uuid(&album.artist)),
        tracks,
    }
}

fn inflate_track(track: RawTrack, album_id: &AlbumId) -> Track {
    Track {
        track_id: TrackId::new(track_uuid(&track.title)),
        title: track.title,
        track_num: track.track_num,
        album_id: album_id.clone(),
    }
}

fn track_uuid(name: impl AsRef<str>) -> Uuid {
    Uuid::new_v5(
        &Uuid::new_v5(&Uuid::NAMESPACE_OID, "Track".as_bytes()),
        name.as_ref().as_bytes(),
    )
}

fn inflate_artist(artist_uuid: Uuid, artist: RawArtist) -> Artist {
    let url = format!("/artist/{}", artist_uuid);
    let mut albums: HashMap<String, AlbumId> = HashMap::new();
    for title in artist.albums {
        let album_id = AlbumId::new(album_uuid(&title));
        albums.insert(title, album_id);
    }
    Artist {
        artist_id: ArtistId::new(artist_uuid),
        name: artist.name,
        url,
        albums,
    }
}

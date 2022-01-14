use crate::client::{ArtistListItem, Client, ReleaseListItem, ReleaseTrack};

#[derive(Debug, Default, PartialEq)]
pub struct Queue {
    tracks: Vec<ReleaseTrack>,
    pub current: Option<CurrentQueueItem>,
    is_playing: bool,
}

#[derive(Debug, PartialEq)]
pub struct CurrentQueueItem {
    idx: usize,
    pub artist: ArtistListItem,
    pub release: ReleaseListItem,
}

#[derive(Debug, PartialEq)]
pub struct QueueItem<'a> {
    position: u32,
    track: &'a ReleaseTrack,
}

impl Queue {
    pub fn new(
        tracks: Vec<ReleaseTrack>,
        current_idx: usize,
        current_artist: Option<ArtistListItem>,
        current_release: Option<ReleaseListItem>,
        is_playing: bool,
    ) -> Self {
        let is_empty = tracks.is_empty();
        Self {
            tracks,
            current: match is_empty {
                true => None,
                false => Some(CurrentQueueItem {
                    idx: current_idx,
                    artist: current_artist.unwrap(),
                    release: current_release.unwrap(),
                }),
            },
            is_playing,
        }
    }

    pub fn current_track(&self) -> Option<&ReleaseTrack> {
        match self.current {
            Some(CurrentQueueItem { idx: i, .. }) => Some(&self.tracks[i]),
            None => None,
        }
    }

    pub fn move_to_next(&mut self) {
        if self.tracks.is_empty() {
            return;
        }

        self.set_current_state(match self.current {
            Some(CurrentQueueItem { idx: i, .. }) => i as i64 + 1,
            None => 1,
        });
    }

    pub fn move_to_previous(&mut self) {
        if self.tracks.is_empty() {
            return;
        }

        self.set_current_state(match self.current {
            Some(CurrentQueueItem { idx: i, .. }) => i as i64 - 1,
            None => 1,
        });
    }

    pub fn set_current_state(&mut self, idx: i64) {
        // if idx < 0 {
        //     self.current = None;
        //     return;
        // }

        // if let Some(current_track) = &self.tracks.get(idx as usize) {
        //     let client = Client::new();
        //     let release = client
        //         .get_release(&current_track.release_id)
        //         .unwrap()
        //         .clone();
        //     let artist = client
        //         .get_artist(&release.core.as_ref().unwrap().primary_artist_id)
        //         .unwrap()
        //         .clone();
        //     self.current = Some(CurrentQueueItem {
        //         idx: idx as usize,
        //         release,
        //         artist,
        //     });
        // } else {
        //     self.current = None
        // }
        // if self.current.is_none() {
        //     self.is_playing = false;
        // }
    }

    pub fn toggle_is_playing(&mut self) {
        self.is_playing = !self.is_playing;
    }

    pub fn visible_items(&self) -> Option<impl Iterator<Item = QueueItem>> {
        match self.current {
            Some(CurrentQueueItem { idx, .. }) => {
                let range = idx..self.tracks.len();
                Some(
                    self.tracks[range]
                        .iter()
                        .enumerate()
                        .map(move |(i, track)| QueueItem {
                            position: (idx + i + 1) as u32,
                            track,
                        }),
                )
            }
            None => None,
        }
    }

    pub fn can_move_to_next(&self) -> bool {
        if self.is_empty() {
            return false;
        }
        match self.current {
            Some(CurrentQueueItem { idx, .. }) => idx < self.tracks.len() - 1,
            None => false,
        }
    }

    pub fn can_move_to_previous(&self) -> bool {
        if self.is_empty() {
            return false;
        }
        match self.current {
            Some(CurrentQueueItem { idx, .. }) => idx != 0,
            None => true,
        }
    }

    pub fn can_play(&self) -> bool {
        self.has_visible_tracks()
    }

    pub fn is_empty(&self) -> bool {
        self.tracks.is_empty()
    }

    pub fn has_visible_tracks(&self) -> bool {
        self.current.is_some() && !self.is_empty()
    }
}

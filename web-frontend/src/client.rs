use crate::models::Queue;
use crumb_client::CrumbClient;
use thiserror::Error;
pub use tonic::codec::Streaming;

tonic::include_proto!("crumb.v1");

#[derive(Debug, Error)]
pub enum Error {
    #[error("could not load data from remote host: {source:}")]
    DataLoadError {
        #[from]
        source: tonic::Status,
    },
    #[error("unexpected error response for {request:} with code {code:}: {message:}")]
    UnexpectedErrorResponse {
        request: &'static str,
        code: tonic::Code,
        message: String,
    },
    #[error("unexpected empty response for {request:}: {message:}")]
    UnexpectedEmptyResponse {
        request: &'static str,
        message: &'static str,
    },
}

#[derive(Debug, Clone)]
pub struct Client<T>(CrumbClient<T>);

impl Client<grpc_web_client::Client> {
    pub fn new() -> Self {
        Self(CrumbClient::new(grpc_web_client::Client::new(
            "http://127.0.0.1:13713".to_string(),
        )))
    }

    pub async fn get_artists(&mut self) -> Result<Vec<ArtistListItem>, Error> {
        let mut stream = self
            .0
            .get_artists(tonic::Request::new(GetArtistsRequest {}))
            .await
            .map_err(|e| Error::from(e))?
            .into_inner();
        let mut artists: Vec<ArtistListItem> = vec![];
        while let Some(a) = stream.message().await? {
            artists.push(a);
        }
        Ok(artists)
    }

    pub async fn get_artist(&mut self, artist_id: &str) -> Result<GetArtistResponse, Error> {
        Ok(self
            .0
            .get_artist(tonic::Request::new(GetArtistRequest {
                artist_id: artist_id.to_string(),
            }))
            .await
            .map_err(|e| Error::from(e))?
            .into_inner())
    }

    pub async fn get_release(&mut self, release_id: &str) -> Result<GetReleaseResponse, Error> {
        Ok(self
            .0
            .get_release(tonic::Request::new(GetReleaseRequest {
                release_id: release_id.to_string(),
            }))
            .await
            .map_err(|e| Error::from(e))?
            .into_inner())
    }

    pub async fn get_queue(&mut self) -> Result<Queue, Error> {
        let mut stream = self
            .0
            .get_queue(tonic::Request::new(GetQueueRequest {}))
            .await
            .map_err(|e| Error::from(e))?
            .into_inner();
        let mut tracks: Vec<ReleaseTrack> = vec![];
        while let Some(t) = stream.message().await? {
            tracks.push(t);
        }
        let current_idx = 1;
        let (current_artist, current_release) =
            self.current_for_queue(&tracks, current_idx).await?;
        Ok(Queue::new(
            tracks,
            current_idx,
            current_artist,
            current_release,
            false,
        ))
    }

    async fn current_for_queue(
        &mut self,
        tracks: &[ReleaseTrack],
        idx: usize,
    ) -> Result<(Option<ArtistListItem>, Option<ReleaseListItem>), Error> {
        if tracks.is_empty() {
            return Ok((None, None));
        }

        let track = &tracks[idx];
        let artist = match self
            .get_artist(&track.primary_artist_id)
            .await?
            .response_either
        {
            Some(get_artist_response::ResponseEither::Artist(a)) => match a.core {
                Some(a) => a,
                None => {
                    return Err(Error::UnexpectedEmptyResponse {
                        request: "GetArtist",
                        message: "response had an empty core field",
                    })
                }
            },
            Some(get_artist_response::ResponseEither::Error(e)) => {
                return Err(Error::UnexpectedErrorResponse {
                    request: "GetArtist",
                    code: tonic::Code::from_i32(e.code),
                    message: e.message,
                })
            }
            None => {
                return Err(Error::UnexpectedEmptyResponse {
                    request: "GetArtist",
                    message: "response was empty",
                })
            }
        };
        let release = match self.get_release(&track.release_id).await?.response_either {
            Some(get_release_response::ResponseEither::Release(r)) => match r.core {
                Some(r) => r,
                None => {
                    return Err(Error::UnexpectedEmptyResponse {
                        request: "GetRelease",
                        message: "response had an empty core field",
                    })
                }
            },
            Some(get_release_response::ResponseEither::Error(e)) => {
                return Err(Error::UnexpectedErrorResponse {
                    request: "GetRelease",
                    code: tonic::Code::from_i32(e.code),
                    message: e.message,
                })
            }
            None => {
                return Err(Error::UnexpectedEmptyResponse {
                    request: "GetRelease",
                    message: "response was empty",
                })
            }
        };
        Ok((Some(artist), Some(release)))
    }
}

impl ArtistListItem {
    pub fn url(&self) -> String {
        format!("/artist/{}", self.artist_id)
    }
}

impl ReleaseListItem {
    pub fn url(&self) -> String {
        format!("/release/{}", self.release_id)
    }

    pub fn best_release_year(&self) -> String {
        match self.original_year {
            Some(y) => format!("{}", y),
            None => match self.release_year {
                Some(y) => format!("{}", y),
                None => "Unknown".to_string(),
            },
        }
    }
}

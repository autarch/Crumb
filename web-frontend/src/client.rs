pub(crate) use crate::{
    grpc::crumb::{crumb_client::CrumbClient, *},
    models::Queue,
};
use thiserror::Error;
pub use tonic::codec::Streaming;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("could not load data from remote host: {source:}")]
    DataLoadError {
        #[from]
        source: tonic::Status,
    },
}

#[derive(Debug, Clone)]
pub(crate) struct Client<T> {
    grpc_client: CrumbClient<T>,
    client_id: String,
}

impl Client<grpc_web_client::Client> {
    pub(crate) fn new(client_id: String) -> Self {
        Self {
            grpc_client: CrumbClient::new(grpc_web_client::Client::new(
                "http://localhost:13713".to_string(),
            )),
            client_id,
        }
    }

    pub(crate) async fn get_artists(&mut self) -> Result<Streaming<ArtistListItem>, Error> {
        Ok(self
            .grpc_client
            .get_artists(tonic::Request::new(GetArtistsRequest {}))
            .await
            .map_err(Error::from)?
            .into_inner())
    }

    pub(crate) async fn get_artist(&mut self, artist_id: &str) -> Result<GetArtistResponse, Error> {
        Ok(self
            .grpc_client
            .get_artist(tonic::Request::new(GetArtistRequest {
                artist_id: artist_id.to_string(),
            }))
            .await
            .map_err(Error::from)?
            .into_inner())
    }

    pub(crate) async fn get_release(
        &mut self,
        release_id: &str,
    ) -> Result<GetReleaseResponse, Error> {
        Ok(self
            .grpc_client
            .get_release(tonic::Request::new(GetReleaseRequest {
                release_id: release_id.to_string(),
            }))
            .await
            .map_err(Error::from)?
            .into_inner())
    }

    pub(crate) async fn get_queue(&mut self) -> Result<Queue, Error> {
        let res = self
            .grpc_client
            .get_queue(tonic::Request::new(GetQueueRequest {
                client_id: self.client_id.clone(),
            }))
            .await;
        self.queue_items_from_stream(res).await
    }

    pub(crate) async fn add_to_queue(&mut self, track_ids: Vec<String>) -> Result<Queue, Error> {
        let res = self
            .grpc_client
            .add_to_queue(tonic::Request::new(AddToQueueRequest {
                client_id: self.client_id.clone(),
                track_ids,
            }))
            .await;
        self.queue_items_from_stream(res).await
    }

    pub(crate) async fn replace_queue(&mut self, track_ids: Vec<String>) -> Result<Queue, Error> {
        let res = self
            .grpc_client
            .replace_queue(tonic::Request::new(ReplaceQueueRequest {
                client_id: self.client_id.clone(),
                track_ids,
            }))
            .await;
        self.queue_items_from_stream(res).await
    }

    pub(crate) async fn remove_from_queue(
        &mut self,
        positions: Vec<String>,
    ) -> Result<Queue, Error> {
        let res = self
            .grpc_client
            .remove_from_queue(tonic::Request::new(RemoveFromQueueRequest {
                client_id: self.client_id.clone(),
                positions,
            }))
            .await;
        self.queue_items_from_stream(res).await
    }

    pub(crate) async fn move_queue_forward(&mut self) -> Result<Queue, Error> {
        let res = self
            .grpc_client
            .move_queue_forward(tonic::Request::new(MoveQueueForwardRequest {
                client_id: self.client_id.clone(),
            }))
            .await;
        self.queue_items_from_stream(res).await
    }

    pub(crate) async fn move_queue_backward(&mut self) -> Result<Queue, Error> {
        let res = self
            .grpc_client
            .move_queue_backward(tonic::Request::new(MoveQueueBackwardRequest {
                client_id: self.client_id.clone(),
            }))
            .await;
        self.queue_items_from_stream(res).await
    }

    pub(crate) async fn like_track(&mut self, track_id: String) -> Result<(), Error> {
        self.grpc_client
            .like_track(tonic::Request::new(LikeTrackRequest { track_id }))
            .await?;
        Ok(())
    }

    pub(crate) async fn dislike_track(&mut self, track_id: String) -> Result<(), Error> {
        self.grpc_client
            .dislike_track(tonic::Request::new(DislikeTrackRequest { track_id }))
            .await?;
        Ok(())
    }

    async fn queue_items_from_stream(
        &mut self,
        req: Result<tonic::Response<tonic::codec::Streaming<QueueItem>>, tonic::Status>,
    ) -> Result<Queue, Error> {
        let mut stream = req.map_err(Error::from)?.into_inner();
        let mut items: Vec<QueueItem> = vec![];
        while let Some(t) = stream.message().await? {
            items.push(t);
        }
        let current =
            items
                .iter()
                .enumerate()
                .find_map(|(i, item)| if item.is_current { Some(i) } else { None });
        Ok(Queue::new(items, current))
    }
}

impl ArtistListItem {
    pub(crate) fn url(&self) -> String {
        artist_url(&self.artist_id)
    }

    pub(crate) fn names(&self) -> impl Iterator<Item = &str> {
        [self.display_name.as_str()].into_iter().chain(
            [
                self.name.as_str(),
                self.translated_name.as_deref().unwrap_or(""),
                self.transcripted_name.as_deref().unwrap_or(""),
            ]
            .into_iter()
            .filter(|t| !(t.is_empty() || *t == self.display_name.as_str())),
        )
    }
}

impl ReleaseListItem {
    pub(crate) fn url(&self) -> String {
        release_url(&self.release_id)
    }

    pub(crate) fn best_release_year(&self, default: &'static str) -> String {
        match self.original_year {
            Some(y) => format!("{}", y),
            None => match self.release_year {
                Some(y) => format!("{}", y),
                None => default.to_string(),
            },
        }
    }

    pub(crate) fn titles(&self) -> impl Iterator<Item = &str> {
        [self.display_title.as_str()].into_iter().chain(
            [
                self.title.as_str(),
                self.translated_title.as_deref().unwrap_or(""),
                self.transcripted_title.as_deref().unwrap_or(""),
            ]
            .into_iter()
            .filter(|t| !(t.is_empty() || *t == self.display_title.as_str())),
        )
    }
}

impl ReleaseTrack {
    pub(crate) fn titles(&self) -> impl Iterator<Item = &str> {
        [Some(self.display_title.as_str())]
            .into_iter()
            .chain(
                [
                    Some(self.title.as_str()),
                    self.translated_title.as_deref(),
                    self.transcripted_title.as_deref(),
                ]
                .into_iter()
                .filter(|t| t.is_some() && t.unwrap() != self.display_title),
            )
            .flatten()
    }
}

impl QueueItem {
    pub(crate) fn artist_url(&self) -> String {
        artist_url(&self.artist_id)
    }

    pub(crate) fn release_url(&self) -> String {
        release_url(&self.release_id)
    }
}

pub(crate) fn artist_url(id: &str) -> String {
    format!("/artist/{}", id)
}

pub(crate) fn release_url(id: &str) -> String {
    format!("/release/{}", id)
}

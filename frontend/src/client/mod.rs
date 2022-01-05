use crumb_client::CrumbClient;
use thiserror::Error;
pub use tonic::codec::Streaming;

tonic::include_proto!("crumb.v1");

#[derive(Debug, Error)]
pub enum Error {
    #[error("could not load data from remote host")]
    DataLoadError {
        #[from]
        source: tonic::Status,
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

    pub async fn fake_queue(&self) -> Result<Vec<ReleaseTrack>, Error> {
        Ok(vec![
            self.tracks_for("Ryokuoshokushakai", "SINGALONG"),
            self.tracks_for("Siip", "Siip"),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>())
    }

    fn tracks_for(&self, artist_name: &str, release_title: &str) -> Vec<ReleaseTrack> {
        vec![]
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

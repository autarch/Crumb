use crumb_client::CrumbClient;
use thiserror::Error;

tonic::include_proto!("crumb");

#[derive(Debug, Error)]
pub enum Error {
    #[error("could not load data from remote host")]
    DataLoadError {
        #[from]
        source: tonic::Status,
    },
}

#[derive(Debug, Clone)]
pub struct Client;

impl Client {
    pub fn new() -> Self {
        Self{}
    }

    pub fn artist_by_name(&self, name: &str) -> Option<&ArtistItem> {
        None
    }

    pub fn artist_by_id(&self, artist_id: &str) -> Option<&ArtistItem> {
        None
    }

    pub async fn load_artist_by_id(&self, artist_id: &str) -> Result<Option<ArtistItem>, Error> {
        Ok(None)
    }

    pub async fn load_artists(&self) -> Result<Vec<ArtistItem>, Error> {
        let stream = self
            .grpc_client()
            .get_artists(tonic::Request::new(GetArtistsRequest {}))
            .await
            .map_err(|e| Error::from(e))?
            .into_inner();
        let mut artists: Vec<ArtistItem> = vec![];
        Ok(artists)
    }

    pub fn release_by_name(&self, name: &str) -> Option<&ReleaseItem> {
        None
    }

    pub fn release_by_id(&self, release_id: &str) -> Option<&ReleaseItem> {
        None
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

    fn grpc_client(&self) -> CrumbClient<grpc_web_client::Client> {
        CrumbClient::new(grpc_web_client::Client::new(
            "http://127.0.0.1:13713".to_string(),
        ))
    }
}

impl ArtistItem {
    fn foo() {}
}

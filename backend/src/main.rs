use crate::crumb_server::{Crumb, CrumbServer};
use anyhow::Result;
use crumb_db::{User, DB};
use futures::{stream, Stream};
use std::{env, pin::Pin};
use tonic::{transport::Server, Request, Response, Status};
use tracing::{event, Level};
use tracing_subscriber;
use uuid::Uuid;

tonic::include_proto!("crumb");

type GetArtistsResult<T> = Result<Response<T>, Status>;
type GetArtistsResponseStream = Pin<Box<dyn Stream<Item = Result<ArtistItem, Status>> + Send>>;

type GetReleasesForArtistResult<T> = Result<Response<T>, Status>;
type GetReleasesForArtistResponseStream =
    Pin<Box<dyn Stream<Item = Result<ReleaseItem, Status>> + Send>>;

type GetTracksForReleaseResult<T> = Result<Response<T>, Status>;
type GetTracksForReleaseResponseStream =
    Pin<Box<dyn Stream<Item = Result<ReleaseTrack, Status>> + Send>>;

#[derive(Debug)]
struct MyCrumb {
    db: DB,
}

impl MyCrumb {
    async fn new<U: AsRef<str>>(db_uri: U) -> Result<Self> {
        let db = DB::new(db_uri.as_ref()).await?;
        Ok(Self { db })
    }
}

#[tonic::async_trait]
impl Crumb for MyCrumb {
    type GetArtistsStream = GetArtistsResponseStream;
    type GetReleasesForArtistStream = GetReleasesForArtistResponseStream;
    type GetTracksForReleaseStream = GetTracksForReleaseResponseStream;

    #[tracing::instrument]
    async fn get_artists(
        &self,
        _: Request<GetArtistsRequest>,
    ) -> GetArtistsResult<Self::GetArtistsStream> {
        let user = self.get_user().await?;
        // XXX - getting a vec from the db using fetch_many the crumb_db
        // package instead of just returning a stream is gross, but attempting
        // to return the stream gave me all sorts of lifetime errors.
        let artists = self
            .db
            .artists_for_user(&user)
            .await
            .map_err(|e| {
                event!(
                    Level::ERROR,
                    user_id = %user.user_id.to_string(),
                    error = %e,
                    "error getting artists for user",
                );
                Status::internal("Server error")
            })?
            .into_iter()
            .map(|a| Ok(to_rpc_artist_struct(a)))
            .collect::<Vec<_>>();
        Ok(Response::new(Box::pin(stream::iter(artists))))
    }

    #[tracing::instrument]
    async fn get_releases_for_artist(
        &self,
        req: Request<GetReleasesForArtistRequest>,
    ) -> GetReleasesForArtistResult<Self::GetReleasesForArtistStream> {
        let user = self.get_user().await?;
        let req_artist_id = req.into_inner().artist_id;
        let artist_id = Uuid::parse_str(&req_artist_id).map_err(|e| {
            event!(
                Level::ERROR,
                artist_id = %req_artist_id,
                error = %e,
                "error parsing artist_id as UUID",
            );
            Status::internal("Server error")
        })?;
        // XXX - getting a vec from the db using fetch_many the crumb_db
        // package instead of just returning a stream is gross, but attempting
        // to return the stream gave me all sorts of lifetime errors.
        let releases = self
            .db
            .releases_for_user_by_artist_id(&user, &artist_id)
            .await
            .map_err(|e| {
                event!(
                    Level::ERROR,
                    user_id = %user.user_id.to_string(),
                    artist_id = %artist_id,
                    error = %e,
                    "error getting releases for for artist",
                );
                Status::internal("Server error")
            })?
            .into_iter()
            .map(|a| Ok(to_rpc_release_struct(a)))
            .collect::<Vec<_>>();
        Ok(Response::new(Box::pin(stream::iter(releases))))
    }

    #[tracing::instrument]
    async fn get_tracks_for_release(
        &self,
        req: Request<GetTracksForReleaseRequest>,
    ) -> GetTracksForReleaseResult<Self::GetTracksForReleaseStream> {
        let user = self.get_user().await?;
        let req_release_id = req.into_inner().release_id;
        let release_id = Uuid::parse_str(&req_release_id).map_err(|e| {
            event!(
                Level::ERROR,
                release_id = %req_release_id,
                error = %e,
                "error parsing release_id as UUID",
            );
            Status::internal("Server error")
        })?;
        // XXX - getting a vec from the db using fetch_many the crumb_db
        // package instead of just returning a stream is gross, but attempting
        // to return the stream gave me all sorts of lifetime errors.
        let releases = self
            .db
            .tracks_for_user_by_release_id(&user, &release_id)
            .await
            .map_err(|e| {
                event!(
                    Level::ERROR,
                    user_id = %user.user_id.to_string(),
                    release_id = %release_id,
                    error = %e,
                    "error getting tracks for release",
                );
                Status::internal("Server error")
            })?
            .into_iter()
            .map(|a| Ok(to_rpc_release_track_struct(a)))
            .collect::<Vec<_>>();
        Ok(Response::new(Box::pin(stream::iter(releases))))
    }
}

impl MyCrumb {
    async fn get_user(&self) -> Result<User, Status> {
        // XXX - need to get this from request somehow
        let email = "autarch@urth.org";
        match self.db.get_user(email).await {
            Ok(Some(user)) => Ok(user),
            Ok(None) => {
                return Err(Status::unauthenticated(
                    "No user credentials present in request",
                ))
            }
            Err(e) => {
                event!(
                    Level::ERROR,
                    email = email,
                    error = %e,
                    "error getting user by email",
                );
                return Err(Status::internal("Server error"));
            }
        }
    }
}

fn to_rpc_artist_struct(a: crumb_db::ArtistItem) -> ArtistItem {
    ArtistItem {
        artist_id: a.artist_id.to_string(),
        name: a.name.into_string(),
        display_name: a.display_name.into_string(),
        sortable_name: a.sortable_name.map(|sn| sn.into_string()),
        transcripted_name: a.transcripted_name.map(|n| n.into_string()),
        transcripted_sortable_name: a.transcripted_sortable_name.map(|sn| sn.into_string()),
        translated_name: a.translated_name.map(|n| n.into_string()),
        translated_sortable_name: a.translated_sortable_name.map(|sn| sn.into_string()),
        release_count: a.release_count as u32,
        track_count: a.track_count as u32,
        album_cover_uri: a.album_cover_uri,
    }
}

fn to_rpc_release_struct(r: crumb_db::ReleaseItem) -> ReleaseItem {
    ReleaseItem {
        release_id: r.release_id.to_string(),
        primary_artist_id: r.primary_artist_id.to_string(),
        title: r.title.into_string(),
        display_title: r.display_title.into_string(),
        transcripted_title: r.transcripted_title.map(|t| t.into_string()),
        translated_title: r.translated_title.map(|t| t.into_string()),
        comment: r.comment.map(|c| c.into_string()),
        release_year: r.release_year.map(|y| y as u32),
        release_month: r.release_month.map(|m| m as u32),
        release_day: r.release_day.map(|d| d as u32),
        original_year: r.original_year.map(|y| y as u32),
        original_month: r.original_month.map(|m| m as u32),
        original_day: r.original_day.map(|d| d as u32),
        track_count: r.track_count as u32,
        album_cover_uri: r.album_cover_uri,
    }
}

fn to_rpc_release_track_struct(t: crumb_db::ReleaseTrack) -> ReleaseTrack {
    let last_dollar = t
        .content_hash
        .rfind('$')
        .expect("content_hash should contains a $-delimited hash algorithm name");
    let hash = &t.content_hash[last_dollar + 1..];
    ReleaseTrack {
        track_id: t.track_id.to_string(),
        primary_artist_id: t.primary_artist_id.to_string(),
        title: t.title.into_string(),
        display_title: t.display_title.into_string(),
        transcripted_title: t.transcripted_title.map(|t| t.into_string()),
        translated_title: t.translated_title.map(|t| t.into_string()),
        length: t.length.map(|l| l as u32),
        track_audio_uri: format!(
            "https://localhost:13713/audio/{}/{}/{}",
            &hash[0..1],
            &hash[0..2],
            hash,
        ),
        position: t.position as u32,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:13713".parse().unwrap();
    let db_uri = env::var("DATABASE_URL").expect("DATABASE_URL must be set to start the server");
    let greeter = MyCrumb::new(&db_uri).await?;

    println!("CrumbServer listening on {}", addr);

    tracing_subscriber::fmt::init();

    Server::builder()
        .add_service(CrumbServer::new(greeter).send_gzip().accept_gzip())
        .serve(addr)
        .await?;

    Ok(())
}

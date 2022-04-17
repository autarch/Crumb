use crate::crumb_server::{Crumb, CrumbServer};
use anyhow::Result;
use crumb_db::{DBError, SQLXError, User, DB};
use futures::{
    stream::{self, BoxStream},
    Stream, StreamExt, TryStreamExt,
};
use rust_decimal::Decimal;
use std::{env, pin::Pin, str::FromStr};
use tokio::sync::mpsc::{self, error::SendError, Sender};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status as TonicStatus};
use tracing::{event, Level};
use uuid::Uuid;

mod grpc {
    #[path = "crumb.v1.rs"]
    pub(crate) mod crumb;
}

use grpc::crumb::*;

type GetArtistsResult<T> = Result<Response<T>, TonicStatus>;

type GetArtistResult = Result<Response<GetArtistResponse>, TonicStatus>;

type GetReleasesForArtistResult<T> = Result<Response<T>, TonicStatus>;
type GetReleasesForArtistResponseStream =
    Pin<Box<dyn Stream<Item = Result<ReleaseListItem, TonicStatus>> + Send>>;

type GetReleaseResult = Result<Response<GetReleaseResponse>, TonicStatus>;

type GetTracksForReleaseResult<T> = Result<Response<T>, TonicStatus>;
type GetTracksForReleaseResponseStream =
    Pin<Box<dyn Stream<Item = Result<ReleaseTrack, TonicStatus>> + Send>>;

type GetQueueResult<T> = Result<Response<T>, TonicStatus>;

type AddToQueueResult<T> = Result<Response<T>, TonicStatus>;

type ReplaceQueueResult<T> = Result<Response<T>, TonicStatus>;

type RemoveFromQueueResult<T> = Result<Response<T>, TonicStatus>;

type MoveQueueForwardResult<T> = Result<Response<T>, TonicStatus>;

type MoveQueueBackwardResult<T> = Result<Response<T>, TonicStatus>;

type LikeTrackResult = Result<Response<LikeOrDislikeTrackResponse>, TonicStatus>;
type DislikeTrackResult = Result<Response<LikeOrDislikeTrackResponse>, TonicStatus>;

#[derive(Clone, Debug)]
struct MyCrumb {
    db: DB,
}

#[tonic::async_trait]
impl Crumb for MyCrumb {
    type GetArtistsStream = ReceiverStream<Result<ArtistListItem, TonicStatus>>;
    type GetReleasesForArtistStream = GetReleasesForArtistResponseStream;
    type GetTracksForReleaseStream = GetTracksForReleaseResponseStream;
    type GetQueueStream = ReceiverStream<Result<QueueItem, TonicStatus>>;

    type AddToQueueStream = ReceiverStream<Result<QueueItem, TonicStatus>>;
    type ReplaceQueueStream = ReceiverStream<Result<QueueItem, TonicStatus>>;
    type RemoveFromQueueStream = ReceiverStream<Result<QueueItem, TonicStatus>>;

    type MoveQueueForwardStream = ReceiverStream<Result<QueueItem, TonicStatus>>;
    type MoveQueueBackwardStream = ReceiverStream<Result<QueueItem, TonicStatus>>;

    #[tracing::instrument(skip(self))]
    async fn get_artists(
        &self,
        _: Request<GetArtistsRequest>,
    ) -> GetArtistsResult<Self::GetArtistsStream> {
        let user = self.get_user().await?;

        let (tx, rx) = mpsc::channel(10);
        let self = self.clone();
        tokio::spawn(async move {
            let mut select = String::new();
            let mut artists = self
                .db
                .artists_for_user(&user, &mut select)
                .map(|a| match a {
                    Ok(a) => Ok(to_rpc_artist_list_item_struct(a)),
                    Err(e) => {
                        event!(
                            Level::ERROR,
                            user_id = %user.user_id.to_string(),
                            error = %e,
                            "error in get_artists",
                        );
                        Err(TonicStatus::internal("Server error"))
                    }
                });
            loop {
                let item = match artists.try_next().await {
                    Ok(Some(artist)) => Ok(artist),
                    Ok(None) => break,
                    Err(e) => Err(e),
                };
                let is_err = item.is_err();
                handle_send_error("get_artists", tx.send(item).await);
                if is_err {
                    break;
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    #[tracing::instrument(skip(self))]
    async fn get_artist(&self, req: Request<GetArtistRequest>) -> GetArtistResult {
        let user = self.get_user().await?;
        let req_artist_id = req.into_inner().artist_id;
        let artist_id = Uuid::parse_str(&req_artist_id).map_err(|e| {
            event!(
                Level::ERROR,
                artist_id = %req_artist_id,
                error = %e,
                "error parsing artist_id as UUID",
            );
            TonicStatus::internal("Server error")
        })?;
        let artist = self.db.artist_for_user(&user, &artist_id).await;
        match artist {
            Ok(a) => Ok(Response::new(GetArtistResponse {
                response_either: Some(get_artist_response::ResponseEither::Artist(
                    to_rpc_artist_item_struct(a),
                )),
            })),
            Err(e) => {
                if let DBError::SQLXError(e) = &e {
                    if matches!(e, SQLXError::RowNotFound) {
                        return Ok(Response::new(GetArtistResponse {
                            response_either: Some(get_artist_response::ResponseEither::Error(
                                Status {
                                    code: Code::NotFound as i32,
                                    message: format!(
                                        "No artist matches the given id - {}",
                                        artist_id
                                    ),
                                    details: vec![],
                                },
                            )),
                        }));
                    }
                }
                event!(
                    Level::ERROR,
                    artist_id = %artist_id,
                    user_id = %user.user_id.to_string(),
                    error = %e,
                    "error getting artist for user",
                );
                Err(TonicStatus::internal("Server error"))
            }
        }
    }

    #[tracing::instrument(skip(self))]
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
            TonicStatus::internal("Server error")
        })?;
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
                TonicStatus::internal("Server error")
            })?
            .into_iter()
            .map(|a| Ok(to_rpc_release_list_item_struct(a)))
            .collect::<Vec<_>>();
        Ok(Response::new(Box::pin(stream::iter(releases))))
    }

    #[tracing::instrument(skip(self))]
    async fn get_release(&self, req: Request<GetReleaseRequest>) -> GetReleaseResult {
        let user = self.get_user().await?;
        let req_release_id = req.into_inner().release_id;
        let release_id = Uuid::parse_str(&req_release_id).map_err(|e| {
            event!(
                Level::ERROR,
                release_id = %req_release_id,
                error = %e,
                "error parsing release_id as UUID",
            );
            TonicStatus::internal("Server error")
        })?;
        let release = self.db.release_for_user(&user, &release_id).await;
        match release {
            Ok(r) => Ok(Response::new(GetReleaseResponse {
                response_either: Some(get_release_response::ResponseEither::Release(
                    to_rpc_release_item_struct(r),
                )),
            })),
            Err(e) => {
                if let DBError::SQLXError(e) = &e {
                    if matches!(e, SQLXError::RowNotFound) {
                        return Ok(Response::new(GetReleaseResponse {
                            response_either: Some(get_release_response::ResponseEither::Error(
                                Status {
                                    code: Code::NotFound as i32,
                                    message: format!(
                                        "No release matches the given id - {}",
                                        release_id
                                    ),
                                    details: vec![],
                                },
                            )),
                        }));
                    }
                }
                event!(
                    Level::ERROR,
                    release_id = %release_id,
                    user_id = %user.user_id.to_string(),
                    error = %e,
                    "error getting artist for user",
                );
                Err(TonicStatus::internal("Server error"))
            }
        }
    }

    #[tracing::instrument(skip(self))]
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
            TonicStatus::internal("Server error")
        })?;
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
                TonicStatus::internal("Server error")
            })?
            .into_iter()
            .map(|a| Ok(to_rpc_release_track_struct(a)))
            .collect::<Vec<_>>();
        Ok(Response::new(Box::pin(stream::iter(releases))))
    }

    #[tracing::instrument(skip(self))]
    async fn get_queue(
        &self,
        req: Request<GetQueueRequest>,
    ) -> GetQueueResult<Self::GetQueueStream> {
        let user = self.get_user().await?;
        let inner = req.into_inner();
        let client_id = parse_client_id(&inner.client_id)?;

        let (tx, rx) = mpsc::channel(10);
        let self = self.clone();
        tokio::spawn(async move {
            let mut select = String::new();
            let items = self.db.queue_for_user(&user, &client_id, &mut select);
            send_queue_item_stream(
                "get_queue",
                items,
                tx,
                user.user_id.to_string().as_str(),
                client_id.to_string().as_str(),
            )
            .await;
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    #[tracing::instrument(skip(self))]
    async fn add_to_queue(
        &self,
        req: Request<AddToQueueRequest>,
    ) -> AddToQueueResult<Self::AddToQueueStream> {
        let user = self.get_user().await?;
        let inner = req.into_inner();
        let client_id = parse_client_id(&inner.client_id)?;
        let track_ids = inner
            .track_ids
            .iter()
            .map(|id| Uuid::parse_str(id))
            .collect::<Result<Vec<Uuid>, uuid::Error>>()
            .map_err(|e| {
                event!(
                    Level::ERROR,
                    error = %e,
                    "error parsing a track_id as UUID",
                );
                TonicStatus::internal("Server error")
            })?;

        let (tx, rx) = mpsc::channel(10);
        let self = self.clone();
        tokio::spawn(async move {
            let mut select = String::new();
            let items = self
                .db
                .add_to_queue_for_user(&user, &client_id, &track_ids, &mut select)
                .await;
            send_queue_item_stream(
                "add_to_queue",
                items,
                tx,
                user.user_id.to_string().as_str(),
                client_id.to_string().as_str(),
            )
            .await;
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    #[tracing::instrument(skip(self))]
    async fn replace_queue(
        &self,
        req: Request<ReplaceQueueRequest>,
    ) -> ReplaceQueueResult<Self::ReplaceQueueStream> {
        let user = self.get_user().await?;
        let inner = req.into_inner();
        let client_id = parse_client_id(&inner.client_id)?;
        let track_ids = inner
            .track_ids
            .iter()
            .map(|i| Uuid::parse_str(i))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                event!(
                    Level::ERROR,
                    error = %e,
                    "error parsing a track_id as a UUID",
                );
                TonicStatus::internal("Server error")
            })?;

        let (tx, rx) = mpsc::channel(10);
        let self = self.clone();
        tokio::spawn(async move {
            let mut select = String::new();
            let items = self
                .db
                .replace_queue_for_user(&user, &client_id, &track_ids, &mut select)
                .await;
            send_queue_item_stream(
                "replace_queue_for_user",
                items,
                tx,
                user.user_id.to_string().as_str(),
                client_id.to_string().as_str(),
            )
            .await;
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    #[tracing::instrument(skip(self))]
    async fn remove_from_queue(
        &self,
        req: Request<RemoveFromQueueRequest>,
    ) -> RemoveFromQueueResult<Self::RemoveFromQueueStream> {
        let user = self.get_user().await?;
        let inner = req.into_inner();
        let client_id = parse_client_id(&inner.client_id)?;
        let positions = inner
            .positions
            .iter()
            .map(|p| Decimal::from_str(p))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                event!(
                    Level::ERROR,
                    error = %e,
                    "error parsing a queue position as decimal",
                );
                TonicStatus::internal("Server error")
            })?;

        let (tx, rx) = mpsc::channel(10);
        let self = self.clone();
        tokio::spawn(async move {
            let mut select = String::new();
            let items = self
                .db
                .remove_from_queue_for_user(&user, &client_id, &positions, &mut select)
                .await;
            send_queue_item_stream(
                "remove_from_queue",
                items,
                tx,
                user.user_id.to_string().as_str(),
                client_id.to_string().as_str(),
            )
            .await;
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    #[tracing::instrument(skip(self))]
    async fn move_queue_forward(
        &self,
        req: Request<MoveQueueForwardRequest>,
    ) -> MoveQueueForwardResult<Self::MoveQueueForwardStream> {
        let user = self.get_user().await?;
        let inner = req.into_inner();
        let client_id = parse_client_id(&inner.client_id)?;

        let (tx, rx) = mpsc::channel(10);
        let self = self.clone();
        tokio::spawn(async move {
            let mut select = String::new();
            let items = self
                .db
                .move_queue_forward_for_user(&user, &client_id, &mut select)
                .await;
            send_queue_item_stream(
                "move_queue_forward",
                items,
                tx,
                user.user_id.to_string().as_str(),
                client_id.to_string().as_str(),
            )
            .await;
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    #[tracing::instrument(skip(self))]
    async fn move_queue_backward(
        &self,
        req: Request<MoveQueueBackwardRequest>,
    ) -> MoveQueueBackwardResult<Self::MoveQueueBackwardStream> {
        let user = self.get_user().await?;
        let inner = req.into_inner();
        let client_id = parse_client_id(&inner.client_id)?;

        let (tx, rx) = mpsc::channel(10);
        let self = self.clone();
        tokio::spawn(async move {
            let mut select = String::new();
            let items = self
                .db
                .move_queue_backward_for_user(&user, &client_id, &mut select)
                .await;
            send_queue_item_stream(
                "move_queue_backward",
                items,
                tx,
                user.user_id.to_string().as_str(),
                client_id.to_string().as_str(),
            )
            .await;
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    #[tracing::instrument(skip(self))]
    async fn like_track(&self, req: Request<LikeTrackRequest>) -> LikeTrackResult {
        let user = self.get_user().await?;
        let req_track_id = req.into_inner().track_id;
        let track_id = Uuid::parse_str(&req_track_id).map_err(|e| {
            event!(
                Level::ERROR,
                track_id = %req_track_id,
                error = %e,
                "error parsing track_id as UUID",
            );
            TonicStatus::internal("Server error")
        })?;
        self.db
            .add_upvote_for_user(&user, &track_id)
            .await
            .map_err(|e| {
                event!(
                    Level::ERROR,
                    user_id = %user.user_id.to_string(),
                    track_id_id = %track_id.to_string(),
                    error = %e,
                    "error adding downvote for user and track",
                );
                TonicStatus::internal("Server error")
            })?;
        Ok(Response::new(LikeOrDislikeTrackResponse {}))
    }

    #[tracing::instrument(skip(self))]
    async fn dislike_track(&self, req: Request<DislikeTrackRequest>) -> DislikeTrackResult {
        let user = self.get_user().await?;
        let req_track_id = req.into_inner().track_id;
        let track_id = Uuid::parse_str(&req_track_id).map_err(|e| {
            event!(
                Level::ERROR,
                track_id = %req_track_id,
                error = %e,
                "error parsing track_id as UUID",
            );
            TonicStatus::internal("Server error")
        })?;
        self.db
            .add_downvote_for_user(&user, &track_id)
            .await
            .map_err(|e| {
                event!(
                    Level::ERROR,
                    user_id = %user.user_id.to_string(),
                    track_id_id = %track_id.to_string(),
                    error = %e,
                    "error adding upvote for user and track",
                );
                TonicStatus::internal("Server error")
            })?;
        Ok(Response::new(LikeOrDislikeTrackResponse {}))
    }
}

impl MyCrumb {
    async fn new<U: AsRef<str>>(db_uri: U) -> Result<Self> {
        let db = DB::new(db_uri.as_ref()).await?;
        Ok(Self { db })
    }

    async fn get_user(&self) -> Result<User, TonicStatus> {
        // XXX - need to get this from request somehow
        let email = "autarch@urth.org";
        match self.db.get_user(email).await {
            Ok(Some(user)) => Ok(user),
            Ok(None) => {
                return Err(TonicStatus::unauthenticated(
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
                return Err(TonicStatus::internal("Server error"));
            }
        }
    }
}

async fn send_queue_item_stream<'a>(
    what: &'static str,
    mut items: BoxStream<'a, Result<crumb_db::QueueItem, DBError>>,
    tx: Sender<Result<QueueItem, TonicStatus>>,
    user_id: &'a str,
    client_id: &'a str,
) {
    loop {
        let item = match items.try_next().await {
            Ok(Some(item)) => Ok(to_rpc_queue_item_struct(item)),
            Ok(None) => break,
            Err(e) => {
                event!(
                    Level::ERROR,
                    user_id = user_id,
                    client_id = client_id,
                    error = %e,
                    "error in {}", what,
                );
                Err(TonicStatus::internal("Server error"))
            }
        };
        let is_err = item.is_err();
        handle_send_error("get_queue", tx.send(item).await);
        if is_err {
            break;
        }
    }
}

fn handle_send_error<T>(what: &str, res: Result<(), SendError<T>>) {
    if let Err(e) = res {
        event!(
            Level::ERROR,
            error = %e,
            "error sending {} message to channel", what,
        );
    }
}

fn to_rpc_artist_list_item_struct(a: crumb_db::ArtistListItem) -> ArtistListItem {
    ArtistListItem {
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
        release_cover_uri: a.release_cover_uri,
    }
}

fn to_rpc_artist_item_struct(a: crumb_db::ArtistItem) -> ArtistItem {
    ArtistItem {
        core: Some(to_rpc_artist_list_item_struct(a.core)),
        releases: a
            .releases
            .into_iter()
            .map(|r| to_rpc_release_list_item_struct(r))
            .collect::<Vec<_>>(),
    }
}

fn to_rpc_release_list_item_struct(r: crumb_db::ReleaseListItem) -> ReleaseListItem {
    ReleaseListItem {
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
        release_cover_uri: r.release_cover_uri,
    }
}

fn to_rpc_release_item_struct(r: crumb_db::ReleaseItem) -> ReleaseItem {
    ReleaseItem {
        core: Some(to_rpc_release_list_item_struct(r.core)),
        artist: Some(to_rpc_artist_list_item_struct(r.artist)),
        tracks: r
            .tracks
            .into_iter()
            .map(|t| to_rpc_release_track_struct(t))
            .collect::<Vec<_>>(),
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
            "http://localhost:7137/audio/{}/{}/{}",
            &hash[0..1],
            &hash[0..2],
            hash,
        ),
        release_id: t.release_id.to_string(),
        position: t.position as u32,
    }
}

fn to_rpc_queue_item_struct(q: crumb_db::QueueItem) -> QueueItem {
    QueueItem {
        release_track: Some(to_rpc_release_track_struct(q.release_track)),
        release_id: q.release_id.to_string(),
        release_display_title: q.release_display_title.into_string(),
        release_cover_uri: q.release_cover_uri,
        artist_id: q.artist_id.to_string(),
        artist_display_name: q.artist_display_name.into_string(),
        queue_position: format!("{}", q.queue_position),
        is_current: q.is_current,
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
        .accept_http1(true)
        .add_service(tonic_web::enable(
            CrumbServer::new(greeter).send_gzip().accept_gzip(),
        ))
        .serve(addr)
        .await?;

    Ok(())
}

fn parse_client_id(id: &str) -> Result<Uuid, TonicStatus> {
    Uuid::parse_str(id).map_err(|e| {
        event!(
            Level::ERROR,
            client_id = %id,
            error = %e,
            "error parsing client_id as UUID",
        );
        TonicStatus::internal("Server error")
    })
}

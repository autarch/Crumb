use acoustid::AcoustIDClient;
use anyhow::{anyhow, Context, Result};
use clap::{App, Arg, ArgGroup, ArgMatches};
use crumb_db::*;
use fern::colors::{Color, ColoredLevelConfig};
use fern::Dispatch;
use id3::Tag;
use itertools::Itertools;
use jwalk::WalkDir;
use log::{debug, error, info};
use mime::Mime;
use mime_detective::MimeDetective;
use serde::Deserialize;
use sha256::digest_file;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process,
    sync::Arc,
};
use tokio::sync::Mutex;
use uuid::Uuid;


#[tokio::main]
async fn main() {
    let matches = app().get_matches();
    let res = init_logger(&matches);
    if let Err(e) = res {
        eprintln!("Error creating logger: {}", e);
        std::process::exit(126);
    }
    let u = Importer::new(&matches);
    let status = match u {
        Ok(u) => u.run().await,
        Err(e) => {
            debug!("{:#?}", e);
            error!("{}", e);
            127
        }
    };
    std::process::exit(status);
}

fn app<'a>() -> App<'a> {
    App::new("ubi")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Dave Rolsky <autarch@urth.org>")
        .about("Import audio files into Crumb")
        .arg(
            Arg::new("path")
                .long("path")
                .short('p')
                .takes_value(true)
                .required(true)
                .help("The path to the file or directory to import."),
        )
        .arg(
            Arg::new("acoustid-key")
                .long("acoustid-key")
                .short('a')
                .takes_value(true)
                .required(true)
                .help("The AcoustID API key to use."),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output"),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Enable debugging output"),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Suppresses most output"),
        )
        .group(ArgGroup::new("log-level").args(&["verbose", "debug", "quiet"]))
}

pub fn init_logger(matches: &ArgMatches) -> Result<(), log::SetLoggerError> {
    let line_colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::BrightBlack)
        .debug(Color::BrightBlack);

    let level = if matches.is_present("debug") {
        log::LevelFilter::Debug
    } else if matches.is_present("verbose") {
        log::LevelFilter::Info
    } else if matches.is_present("quiet") {
        log::LevelFilter::Error
    } else {
        log::LevelFilter::Warn
    };

    let level_colors = line_colors.info(Color::Green).debug(Color::Black);

    Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{color_line}[{target}][{level}{color_line}] {message}\x1B[0m",
                color_line = format_args!(
                    "\x1B[{}m",
                    line_colors.get_color(&record.level()).to_fg_str()
                ),
                target = record.target(),
                level = level_colors.color(record.level()),
                message = message,
            ));
        })
        .level(level)
        .chain(std::io::stderr())
        .apply()
}

struct Importer {
    path: String,
    key: String,
    detective: MimeDetective,
}

#[derive(Debug)]
struct AudioFileInfo {
    file: PathBuf,
    fingerprint: String,
    track_info: crumb_db::TrackInfo,
    mb_album_id: Option<String>,
    mb_artist_id: Option<String>,
    mb_album_artist_id: Option<String>,
    mb_release_group_id: Option<String>,
    mb_track_id: Option<String>,
}

impl Importer {
    pub fn new(matches: &ArgMatches) -> Result<Importer> {
        Ok(Importer {
            path: matches.value_of("path").unwrap().to_string(),
            key: matches.value_of("acoustid-key").unwrap().to_string(),
            detective: MimeDetective::new()?,
        })
    }

    async fn run(&self) -> i32 {
        match self.import().await {
            Ok(count) => {
                info!("Imported {} tracks", count);
                0
            }
            Err(e) => {
                debug!("{:#?}", e);
                error!("{}", e);
                1
            }
        }
    }

    async fn import(&self) -> Result<usize> {
        let db = DB::new("postgres://autarch:autarch@localhost/crumb").await?;
        let user = db.get_or_insert_user("autarch@urth.org").await?;

        // Collect all tracks for an album by using MP3 data.
        //   Get all matching release groups & release for recording ID based on fingerprint.
        //   If there's only one release group, use that.
        //   If there's >1 group, first match the release group name to the MP3 album name.
        //     If one matches, use that.
        //     If multiples match, prompt user?
        //     If none match, match based on track count.
        //       If one matches, use that.
        //       If multiples match, prompt user?
        //       If none match, use MP3 metadata and match to MB again later?

        let now = std::time::Instant::now();
        let mut albums: Arc<Mutex<HashMap<String, Vec<AudioFileInfo>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        for e in WalkDir::new(&self.path) {
            let e = e?;
            let path = e.path();
            let mime = self.mime_type(&path)?;
            if !is_audio_file(&mime) {
                continue;
            }

            //let sha256 = digest_file(path)?;
            let par_albums = Arc::clone(&albums);
            let info = audio_file_info(&path)?;
            let mut hm = par_albums.lock().await;
            if let Some(album) = hm.get_mut(&info.track_info.album) {
                album.push(info);
            } else {
                hm.insert(info.track_info.album.clone(), vec![info]);
            }
        }
        println!("ELAPSED = {}", now.elapsed().as_millis());

        let mut import_count = 0;
        for album in Arc::try_unwrap(albums)
            .expect("Could not unwrap Arc around albums")
            .into_inner()
            .into_values()
        {
            let matches = db
                .best_matches_for_tracks(
                    &album
                        .iter()
                        .map(|a| &a.track_info)
                        .collect::<Vec<&TrackInfo>>(),
                )
                .await?;
            println!("{:#?}", matches);

            let status = album_match_status(&album, &matches);
            import_count += match status {
                MatchStatus::Perfect => {
                    self.import_album(
                        &db,
                        &user,
                        &album,
                        &matches.values().map(|vt| &vt[0]).collect::<Vec<_>>(),
                    )
                    .await?
                }
                MatchStatus::MultipleReleasesForRecording
                | MatchStatus::MultipleReleasesForReleaseGroup => {
                    self.import_album(&db, &user, &album, &pick_best_release(&album, &matches))
                        .await?
                }
                _ => {
                    debug!("Match status = {:?}", status);
                    0
                }
            };

            // let ids = acoustid
            //     .lookup(&fp.fingerprint, fp.duration.round() as u64)
            //     .await?;

            // let uuid = Uuid::parse_str(&ids[0])?;
            // db.insert_user_track_for_recording_id(&uuid, &user).await?;
        }

        //let acoustid = AcoustIDClient::new(&self.key);

        Ok(import_count)
    }

    async fn import_album(
        &self,
        db: &DB,
        user: &User,
        album: &[AudioFileInfo],
        best_matches: &[&TrackMatch],
    ) -> Result<usize> {
        db.insert_user_tracks(user, best_matches).await?;
        Ok(best_matches.len())
    }

    fn mime_type(&self, path: &Path) -> Result<Mime> {
        self.detective
            .detect_filepath(path)
            .context("Failed to get MIME type for path")
    }
}

fn is_audio_file(mime: &Mime) -> bool {
    match (mime.type_().as_str(), mime.subtype().as_str()) {
        ("audio", "aac") | ("audio", "flac") | ("audio", "mpeg") | ("audio", "ogg") => true,
        _ => false,
    }
}

fn audio_file_info(path: &Path) -> Result<AudioFileInfo> {
    let fp = fingerprint(path)?;
    let tag = Tag::read_from_path(path)?;
    // Where are the null bytes coming from in these strings? Is this a bug in
    // the id3 crate or legit bad data in my MP3s?
    Ok(AudioFileInfo {
        file: path.to_owned(),
        fingerprint: fp.fingerprint,
        track_info: crumb_db::TrackInfo {
            position: tag
                .track()
                .ok_or_else(|| anyhow!("No track number in metadata for `{}`", path.display()))?
                as i32,
            title: tag
                .title()
                .ok_or_else(|| anyhow!("No title in metadata for `{}`", path.display()))?
                .trim_matches('\0')
                .to_string(),
            album: tag
                .album()
                .ok_or_else(|| anyhow!("No album in metadata for `{}`", path.display()))?
                .trim_matches('\0')
                .to_string(),
            artist: tag
                .artist()
                .ok_or_else(|| anyhow!("No artist in metadata for `{}`", path.display()))?
                .trim_matches('\0')
                .to_string(),
        },
        mb_album_id: get_optional_tag_content(&tag, "MusicBrainz Album Id"),
        mb_artist_id: get_optional_tag_content(&tag, "MusicBrainz Artist Id"),
        mb_album_artist_id: get_optional_tag_content(&tag, "MusicBrainz Album Artist Id"),
        mb_release_group_id: get_optional_tag_content(&tag, "MusicBrainz Release Group Id"),
        mb_track_id: get_optional_tag_content(&tag, "MusicBrainz Release Track Id"),
    })
}

fn get_optional_tag_content(tag: &id3::Tag, description: &str) -> Option<String> {
    for t in tag.extended_texts() {
        if t.description == description {
            return Some(t.value.clone());
        }
    }
    None
}

#[derive(Deserialize)]
struct FP {
    duration: f64,
    fingerprint: String,
}

fn fingerprint(path: &Path) -> Result<FP> {
    let mut c = process::Command::new("fpcalc");
    c.arg("-json");
    c.arg(path.as_os_str());

    let output = c.output()?;
    match output.status.code() {
        Some(0) => (),
        _ => {
            return Err(anyhow::anyhow!(
                "Got unexpected output from fpcalc`: {}\n{}",
                String::from_utf8(output.stdout)?,
                String::from_utf8(output.stderr)?,
            ));
        }
    }

    let fp: FP = serde_json::from_slice(&output.stdout)?;
    Ok(fp)
}

#[derive(Debug)]
enum MatchStatus {
    Perfect,
    MultipleReleasesForRecording,
    MultipleReleasesForReleaseGroup,
    PositionMismatch,
    TrackCountMismatch,
    MultipleValidMatches,
}

fn album_match_status(
    album: &[AudioFileInfo],
    matches: &HashMap<i32, Vec<TrackMatch>>,
) -> MatchStatus {
    let matched_positions = matches.keys().map(|p| *p).sorted().collect::<Vec<i32>>();
    let mut album_positions = album
        .iter()
        .map(|t| t.track_info.position)
        .sorted()
        .collect::<Vec<i32>>();

    if album_positions.len() != matched_positions.len() {
        debug!("Number of tracks in album does not match number of tracks matched");
        return MatchStatus::TrackCountMismatch;
    }

    if album_positions == matched_positions {
        if matches.values().all(|m| m.len() == 1) {
            debug!(concat!(
                "All album track positions match and",
                " there is only one track match per position",
            ));
            return MatchStatus::Perfect;
        }

        let each_set_has_the_same_recording_id = matches
            .values()
            .map(|vt| {
                vt.iter()
                    .map(|t| t.recording_id)
                    .unique()
                    .collect::<Vec<i32>>()
                    .len()
                    == 1
            })
            .all(|u| u);
        let release_ids = matches
            .values()
            .map(|vt| vt.iter().map(|t| t.release_id))
            .flatten()
            .unique()
            .collect::<Vec<i32>>();
        if each_set_has_the_same_recording_id && release_ids.len() > 1 {
            debug!(concat!(
                "All album track positions match and there is more than",
                " one track match for at least one position,",
                " but each position's matches have the same recording",
            ));
            return MatchStatus::MultipleReleasesForRecording;
        }

        if matches
            .values()
            .map(|m| m.iter().map(|m| m.release_group_id))
            .flatten()
            .unique()
            .collect::<Vec<_>>()
            .len()
            == 1
        {
            debug!(concat!(
                "All album track positions match, every track has",
                " the same number of matches, and every match is for",
                " the same release_group_id.",
            ));
            return MatchStatus::MultipleReleasesForReleaseGroup;
        }

        if matches.values().any(|m| m.len() > 1) {
            debug!(concat!(
                "All album track positions match but there is",
                " more than one track match for at least one position",
            ));
            return MatchStatus::MultipleValidMatches;
        }
    }

    // I think this will only happen if an import's metadata has some sort of
    // weird track numbering oddity. For example, it's a 2-CD sset with 20
    // total tracks and the imported data has sequential track numbers (1-20),
    // but musicbrainz numbers each CD separately (1-9, 1-11), or vice versa.
    debug!("Album has same number of tracks as matches but the positions don't match");
    MatchStatus::PositionMismatch
}

fn pick_best_release<'a>(
    album: &[AudioFileInfo],
    matches: &'a HashMap<i32, Vec<TrackMatch>>,
) -> Vec<&'a TrackMatch> {
    // This turns our matches from a single vec where each value can have 1+
    // TrackMatch into a hasmap of vecs, where each vec contains all of the
    // matches for a single release.
    let mut releases: HashMap<i32, Vec<&TrackMatch>> = HashMap::new();
    for v in matches.values() {
        for tm in v {
            if let Some(sm) = releases.get_mut(&tm.release_id) {
                sm.push(tm);
            } else {
                let release: Vec<&TrackMatch> = vec![tm];
                releases.insert(tm.release_id, release);
            }
        }
    }

    // All sets of tracks have the same title and recording id. This is more
    // or less identical variations of the same release, at least as far as
    // we're concerned.
    if matches.values().all(|vt| {
        vt.iter()
            .map(|t| &t.track_title)
            .unique()
            .collect::<Vec<_>>()
            .len()
            == 1
            && vt
                .iter()
                .map(|t| t.recording_id)
                .unique()
                .collect::<Vec<_>>()
                .len()
                == 1
    }) {
        debug!(concat!(
            "All sets of tracks have the same title",
            " and recording id. Looking for a release without",
            " a comment.",
        ));
        return pick_release_based_on_comments(releases);
    }

    let same_length_releases = releases
        .values()
        .cloned()
        .filter(|r| r.len() == album.len())
        .map(|r| (r[0].release_id, r))
        .collect::<HashMap<_, Vec<_>>>();
    if !same_length_releases.is_empty() {
        debug!(concat!(
            "All releases have the same length. Looking for a",
            " release without a comment.",
        ));
        return pick_release_based_on_comments(same_length_releases);
    }

    // XXX - There's probably more to be done to pick the best one ...
    releases.into_values().next().unwrap().clone()
}

fn pick_release_based_on_comments<'a>(
    mut releases: HashMap<i32, Vec<&'a TrackMatch>>,
) -> Vec<&'a TrackMatch> {
    let first_tracks = releases.values().map(|r| r[0]).collect::<Vec<_>>();
    let mut release_id = *releases.keys().next().unwrap();
    // If all the releases have a comment or none have one then we just
    // use the first index. Otherwise we pick the first index without a
    // comment.
    if !(first_tracks.iter().all(|t| !t.release_comment.is_empty())
        || first_tracks.iter().all(|t| t.release_comment.is_empty()))
    {
        for t in first_tracks {
            if !t.release_comment.is_empty() {
                continue;
            }
            release_id = t.release_id;
            break;
        }
    }
    releases.remove(&release_id).unwrap()
}

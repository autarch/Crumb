use acoustid::AcoustIDClient;
use anyhow::{Context, Result};
use clap::{App, Arg, ArgGroup, ArgMatches};
use crumb_db::*;
use fern::colors::{Color, ColoredLevelConfig};
use fern::Dispatch;
use log::{debug, error, info};
use mime::Mime;
use mime_detective::MimeDetective;
use serde::Deserialize;
use sha256::digest_file;
use std::{fs::File, path::Path, process};
use walkdir::WalkDir;

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
                .about("The path to the file or directory to import."),
        )
        .arg(
            Arg::new("acoustid-key")
                .long("acoustid-key")
                .short('a')
                .takes_value(true)
                .required(true)
                .about("The AcoustID API key to use."),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .about("Enable verbose output"),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .about("Enable debugging output"),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .about("Suppresses most output"),
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

    async fn import(&self) -> Result<u32> {
        let acoustid = AcoustIDClient::new(&self.key);

        for e in WalkDir::new(&self.path) {
            let e = e?;
            let path = e.path();
            let mime = self.mime_type(path)?;
            if !is_audio_file(&mime) {
                continue;
            }
            //let sha256 = digest_file(path)?;

            let fp = fingerprint(path)?;
            println!("{} = {}", path.display(), fp.fingerprint);

            let ids = acoustid
                .lookup(&fp.fingerprint, fp.duration.round() as u64)
                .await?;
            println!("  {:?}", ids);
        }
        Ok(0)
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

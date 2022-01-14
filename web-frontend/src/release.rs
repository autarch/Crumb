use crate::{
    client::{get_release_response, Client, ReleaseTrack},
    components::{AlbumCover, PageTitle, UserFacingError},
};
use dioxus::{prelude::*, router::use_route};

pub(crate) fn Release(cx: Scope) -> Element {
    let release_id = use_route(&cx)
        .segment::<String>("release_id")
        .expect("id parameter was not found in path somehow")
        .expect("id parameter could not be parsed as a String");
    let release = use_future(&cx, || async move {
        Client::new().get_release(&release_id).await
    });
    cx.render(rsx! {
        crate::Crumb {
            match release.value() {
                Some(Ok(response)) => {
                    match &response.response_either {
                        Some(get_release_response::ResponseEither::Release(release)) => {
                            let core = release.core.as_ref().unwrap();
                            rsx! {
                                PageTitle {
                                    "{core.display_title}"
                                },
                                div {
                                    class: "flex flex-row flex-wrap justify-center",
                                    release.tracks.iter().map(|t| rsx!{
                                        [
                                            rsx! {
                                                OneTrack {
                                                    key: "{t.track_id}",
                                                    track: t,
                                                }
                                            },
                                            rsx!{ br {} },
                                        ],
                                    }),
                                },
                            }
                        },
                        Some(get_release_response::ResponseEither::Error(e)) => rsx! {
                            UserFacingError {
                                error: e
                            }
                        },
                        None => {
                            log::error!("Empty response for GetRelease request!");
                            rsx! {
                                "Error loading release"
                            }
                        }
                    }
                },
                Some(Err(e)) => {
                    log::error!("Error loading release: {}", e);
                    rsx! {
                        "Error loading release"
                    }
                },
                None => {
                    rsx! {
                        "Loading release",
                    }
                },
            }
        }
    })
}

#[inline_props]
fn OneTrack<'a>(cx: Scope, track: &'a ReleaseTrack) -> Element {
    let track_url = "/foo";
    cx.render(rsx! {
        "{track.position}. " "{track.display_title}"
    })
}

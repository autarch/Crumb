use crate::{
    client::{ArtistItem, ReleaseItem},
    image_src,
};
use seed::{prelude::*, *};

#[derive(Clone, Debug)]
pub enum Msg {
    Dummy,
}

pub fn release_image(
    artist: &ArtistItem,
    release: &ReleaseItem,
    classes: Option<&[&'static str]>,
) -> Node<Msg> {
    let cover_uri = match release.album_cover_uri {
        Some(c) => c,
        None => return Node::Empty,
    };
    let mut i = img![attrs! {
        At::Src => image_src(&cover_uri),
        At::Title => format!("{} by {}", release.display_title, artist.display_name),
    }];
    if let Some(classes) = classes {
        for &c in classes {
            i.add_class(c);
        }
    }
    i
}

use crate::{
    image_src,
    models::{Album, Artist},
};
use seed::{prelude::*, *};

#[derive(Clone, Debug)]
pub enum Msg {
    Dummy,
}

pub fn album_image(artist: &Artist, album: &Album, classes: Option<&[&'static str]>) -> Node<Msg> {
    let cover = album.cover_image_url();
    let mut i = img![attrs! {
        At::Src => image_src(cover.as_str()),
        At::Title => format!("{} by {}", album.title, artist.name),
    }];
    if let Some(classes) = classes {
        for c in classes {
            i.add_class(*c);
        }
    }
    i
}

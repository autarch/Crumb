use seed::{prelude::*, *};
use typed_builder::TypedBuilder;

#[derive(Clone, Debug)]
pub enum Msg {
    Dummy,
}

#[derive(TypedBuilder)]
#[builder(doc)]
pub struct Icon {
    path: Attrs,
    #[builder(default = 20)]
    size: u16,
    #[builder(default = "currentColor")]
    fill: &'static str,
    #[builder(default = vec![])]
    classes: Vec<&'static str>,
    title: &'static str,
}

pub fn x_circle() -> IconBuilder<((seed::Attrs,), (), (), (), ())> {
    Icon::builder().path(attrs! {
        At::FillRule => "evenodd",
        At::D => "M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z",
        At::ClipRule => "evenodd",
    })
}

pub fn dots_vertical() -> IconBuilder<((seed::Attrs,), (), (), (), ())> {
    Icon::builder().path(attrs! {
        At::D => "M10 6a2 2 0 110-4 2 2 0 010 4zM10 12a2 2 0 110-4 2 2 0 010 4zM10 18a2 2 0 110-4 2 2 0 010 4z",
    })
}

pub fn rewind() -> IconBuilder<((seed::Attrs,), (), (), (), ())> {
    Icon::builder().path(attrs! {
        At::D => "M8.445 14.832A1 1 0 0010 14v-2.798l5.445 3.63A1 1 0 0017 14V6a1 1 0 00-1.555-.832L10 8.798V6a1 1 0 00-1.555-.832l-6 4a1 1 0 000 1.664l6 4z",
    })
}

pub fn play() -> IconBuilder<((seed::Attrs,), (), (), (), ())> {
    Icon::builder().path(attrs! {
        At::FillRule => "evenodd",
        At::D => "M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z",
        At::ClipRule => "evenodd",
    })
}

pub fn pause() -> IconBuilder<((seed::Attrs,), (), (), (), ())> {
    Icon::builder().path(attrs! {
        At::FillRule => "evenodd",
        At::D => "M18 10a8 8 0 11-16 0 8 8 0 0116 0zM7 8a1 1 0 012 0v4a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v4a1 1 0 102 0V8a1 1 0 00-1-1z",
        At::ClipRule => "evenodd",
    })
}

pub fn fast_forward() -> IconBuilder<((seed::Attrs,), (), (), (), ())> {
    Icon::builder().path(attrs! {
        At::D => "M4.555 5.168A1 1 0 003 6v8a1 1 0 001.555.832L10 11.202V14a1 1 0 001.555.832l6-4a1 1 0 000-1.664l-6-4A1 1 0 0010 6v2.798l-5.445-3.63z",
    })
}

pub fn volume_up() -> IconBuilder<((seed::Attrs,), (), (), (), ())> {
    Icon::builder().path(attrs! {
        At::FillRule => "evenodd",
        At::D => "M9.383 3.076A1 1 0 0110 4v12a1 1 0 01-1.707.707L4.586 13H2a1 1 0 01-1-1V8a1 1 0 011-1h2.586l3.707-3.707a1 1 0 011.09-.217zM14.657 2.929a1 1 0 011.414 0A9.972 9.972 0 0119 10a9.972 9.972 0 01-2.929 7.071 1 1 0 01-1.414-1.414A7.971 7.971 0 0017 10c0-2.21-.894-4.208-2.343-5.657a1 1 0 010-1.414zm-2.829 2.828a1 1 0 011.415 0A5.983 5.983 0 0115 10a5.984 5.984 0 01-1.757 4.243 1 1 0 01-1.415-1.415A3.984 3.984 0 0013 10a3.983 3.983 0 00-1.172-2.828 1 1 0 010-1.415z",
        At::ClipRule => "evenodd",
    })
}

// pub fn volume_mute() -> IconBuilder<((seed::Attrs,), (), (), (), ())> {
//     Icon::builder().path(attrs! {
//         At::FillRule => "evenodd",
//         At::D => "M9.383 3.076A1 1 0 0110 4v12a1 1 0 01-1.707.707L4.586 13H2a1 1 0 01-1-1V8a1 1 0 011-1h2.586l3.707-3.707a1 1 0 011.09-.217zM12.293 7.293a1 1 0 011.414 0L15 8.586l1.293-1.293a1 1 0 111.414 1.414L16.414 10l1.293 1.293a1 1 0 01-1.414 1.414L15 11.414l-1.293 1.293a1 1 0 01-1.414-1.414L13.586 10l-1.293-1.293a1 1 0 010-1.414z",
//         At::ClipRule => "evenodd",
//     })
// }

pub fn thumbs_up() -> IconBuilder<((seed::Attrs,), (), (), (), ())> {
    Icon::builder().path(attrs! {
        At::D => "M2 10.5a1.5 1.5 0 113 0v6a1.5 1.5 0 01-3 0v-6zM6 10.333v5.43a2 2 0 001.106 1.79l.05.025A4 4 0 008.943 18h5.416a2 2 0 001.962-1.608l1.2-6A2 2 0 0015.56 8H12V4a2 2 0 00-2-2 1 1 0 00-1 1v.667a4 4 0 01-.8 2.4L6.8 7.933a4 4 0 00-.8 2.4z",
    })
}

pub fn thumbs_down() -> IconBuilder<((seed::Attrs,), (), (), (), ())> {
    Icon::builder().path(attrs! {
        At::D => "M18 9.5a1.5 1.5 0 11-3 0v-6a1.5 1.5 0 013 0v6zM14 9.667v-5.43a2 2 0 00-1.105-1.79l-.05-.025A4 4 0 0011.055 2H5.64a2 2 0 00-1.962 1.608l-1.2 6A2 2 0 004.44 12H8v4a2 2 0 002 2 1 1 0 001-1v-.667a4 4 0 01.8-2.4l1.4-1.866a4 4 0 00.8-2.4z",
    })
}

pub fn cog() -> IconBuilder<((seed::Attrs,), (), (), (), ())> {
    Icon::builder().path(attrs! {
        At::FillRule => "evenodd",
        At::D => "M11.49 3.17c-.38-1.56-2.6-1.56-2.98 0a1.532 1.532 0 01-2.286.948c-1.372-.836-2.942.734-2.106 2.106.54.886.061 2.042-.947 2.287-1.561.379-1.561 2.6 0 2.978a1.532 1.532 0 01.947 2.287c-.836 1.372.734 2.942 2.106 2.106a1.532 1.532 0 012.287.947c.379 1.561 2.6 1.561 2.978 0a1.533 1.533 0 012.287-.947c1.372.836 2.942-.734 2.106-2.106a1.533 1.533 0 01.947-2.287c1.561-.379 1.561-2.6 0-2.978a1.532 1.532 0 01-.947-2.287c.836-1.372-.734-2.942-2.106-2.106a1.532 1.532 0 01-2.287-.947zM10 13a3 3 0 100-6 3 3 0 000 6z",
        At::ClipRule => "evenodd",
    })
}

pub fn menu() -> IconBuilder<((seed::Attrs,), (), (), (), ())> {
    Icon::builder().path(attrs! {
        At::FillRule => "evenodd",
        At::D => "M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z",
        At::ClipRule => "evenodd",
    })
}

impl Icon {
    pub fn into_svg(self) -> Node<Msg> {
        let mut svg = svg![
            attrs! {
                At::Height => self.size.to_string(),
                At::Width => self.size.to_string(),
                At::ViewBox => "0 0 20 20",
                At::Fill => self.fill,
            },
            path![self.path],
        ];

        for c in self.classes {
            svg.add_class(c);
        }

        svg.add_child(title![self.title]);

        svg
    }
}

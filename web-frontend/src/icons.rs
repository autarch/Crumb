use dioxus::prelude::*;

// pub fn x_circle() -> IconBuilder<((SVGPath,), (), (), (), ())> {
//     Icon::builder().path(
//         SVGPath::builder().fill_rule(Some("evenodd")).
//             d("M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z").
//             clip_rule(Some("evenodd")).build(),
//     )
// }

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Shape {
    Cog,
    DotsVertical,
    FastForward,
    Hamburger,
    Pause,
    Play,
    Rewind,
    ThumbsDown,
    ThumbsUp,
    VolumeMute,
}

impl Shape {
    fn path(&self) -> LazyNodes {
        match self {
            Shape::Cog => rsx! {
                path {
                    fill_rule: "evenodd",
                    d: "M11.49 3.17c-.38-1.56-2.6-1.56-2.98 0a1.532 1.532 0 01-2.286.948c-1.372-.836-2.942.734-2.106 2.106.54.886.061 2.042-.947 2.287-1.561.379-1.561 2.6 0 2.978a1.532 1.532 0 01.947 2.287c-.836 1.372.734 2.942 2.106 2.106a1.532 1.532 0 012.287.947c.379 1.561 2.6 1.561 2.978 0a1.533 1.533 0 012.287-.947c1.372.836 2.942-.734 2.106-2.106a1.533 1.533 0 01.947-2.287c1.561-.379 1.561-2.6 0-2.978a1.532 1.532 0 01-.947-2.287c.836-1.372-.734-2.942-2.106-2.106a1.532 1.532 0 01-2.287-.947zM10 13a3 3 0 100-6 3 3 0 000 6z",
                    clip_rule: "evenodd",
                }
            },
            Shape::DotsVertical => rsx! {
                path {
                    d: "M10 6a2 2 0 110-4 2 2 0 010 4zM10 12a2 2 0 110-4 2 2 0 010 4zM10 18a2 2 0 110-4 2 2 0 010 4z",
                },
            },
            Shape::FastForward => rsx! {
                path {
                    d: "M4.555 5.168A1 1 0 003 6v8a1 1 0 001.555.832L10 11.202V14a1 1 0 001.555.832l6-4a1 1 0 000-1.664l-6-4A1 1 0 0010 6v2.798l-5.445-3.63z",
                },
            },
            Shape::Hamburger => rsx! {
                path {
                    fill_rule: "evenodd",
                    d: "M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z",
                    clip_rule: "evenodd",
                }
            },
            Shape::Pause => rsx! {
                path {
                    fill_rule: "evenodd",
                    d: "M18 10a8 8 0 11-16 0 8 8 0 0116 0zM7 8a1 1 0 012 0v4a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v4a1 1 0 102 0V8a1 1 0 00-1-1z",
                    clip_rule: "evenodd",
                }
            },
            Shape::Play => rsx! {
                path {
                    fill_rule: "evenodd",
                    d: "M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z",
                    clip_rule: "evenodd",
                }
            },
            Shape::Rewind => rsx! {
                path {
                    d: "M8.445 14.832A1 1 0 0010 14v-2.798l5.445 3.63A1 1 0 0017 14V6a1 1 0 00-1.555-.832L10 8.798V6a1 1 0 00-1.555-.832l-6 4a1 1 0 000 1.664l6 4z",
                }
            },
            Shape::ThumbsDown => rsx! {
                path {
                    d: "M18 9.5a1.5 1.5 0 11-3 0v-6a1.5 1.5 0 013 0v6zM14 9.667v-5.43a2 2 0 00-1.105-1.79l-.05-.025A4 4 0 0011.055 2H5.64a2 2 0 00-1.962 1.608l-1.2 6A2 2 0 004.44 12H8v4a2 2 0 002 2 1 1 0 001-1v-.667a4 4 0 01.8-2.4l1.4-1.866a4 4 0 00.8-2.4z",
                }
            },
            Shape::ThumbsUp => {
                rsx! {
                    path {
                        d: "M2 10.5a1.5 1.5 0 113 0v6a1.5 1.5 0 01-3 0v-6zM6 10.333v5.43a2 2 0 001.106 1.79l.05.025A4 4 0 008.943 18h5.416a2 2 0 001.962-1.608l1.2-6A2 2 0 0015.56 8H12V4a2 2 0 00-2-2 1 1 0 00-1 1v.667a4 4 0 01-.8 2.4L6.8 7.933a4 4 0 00-.8 2.4z",
                    }
                }
            }
            Shape::VolumeMute => {
                rsx! {
                    path {
                        fill_rule: "evenodd",
                        d: "M9.383 3.076A1 1 0 0110 4v12a1 1 0 01-1.707.707L4.586 13H2a1 1 0 01-1-1V8a1 1 0 011-1h2.586l3.707-3.707a1 1 0 011.09-.217zM12.293 7.293a1 1 0 011.414 0L15 8.586l1.293-1.293a1 1 0 111.414 1.414L16.414 10l1.293 1.293a1 1 0 01-1.414 1.414L15 11.414l-1.293 1.293a1 1 0 01-1.414-1.414L13.586 10l-1.293-1.293a1 1 0 010-1.414z",
                        clip_rule: "evenodd",
                    },
                }
            }
        }
    }
}

#[derive(PartialEq, Props)]
pub(crate) struct IconButtonProps {
    #[props(default, strip_option)]
    class: Option<&'static str>,
    #[props(default, strip_option)]
    title: Option<&'static str>,
    #[props(default = 20)]
    size: u8,
    #[props(default = "currentColor")]
    fill: &'static str,
    #[props(default = false)]
    disabled: bool,
    shape: Shape,
}

pub(crate) fn IconButton(cx: Scope<IconButtonProps>) -> Element {
    cx.render(rsx! {
        button {
            class: format_args!("{}", cx.props.class.unwrap_or("")),
            title: format_args!("{}", cx.props.title.unwrap_or("")),
            disabled: format_args!("{}", if cx.props.disabled { "true" } else { "false" }),
            Icon {
                disabled: cx.props.disabled,
                size: cx.props.size,
                shape: cx.props.shape,
            },
        },
    })
}

#[derive(PartialEq, Props)]
pub(crate) struct IconProps {
    #[props(default, strip_option)]
    class: Option<&'static str>,
    #[props(default = 20)]
    size: u8,
    #[props(default = "currentColor")]
    fill: &'static str,
    shape: Shape,
    #[props(default = false)]
    disabled: bool,
}

pub(crate) fn Icon(cx: Scope<IconProps>) -> Element {
    let fill = if cx.props.disabled {
        // coolGray 400
        "#9CA3AF"
    } else {
        "currentColor"
    };
    cx.render(rsx! {
        svg {
            class: format_args!("{}", cx.props.class.unwrap_or("")),
            height: format_args!("{}", cx.props.size),
            width: format_args!("{}", cx.props.size),
            view_box: "0 0 20 20",
            fill: format_args!("{}", fill),
            cx.props.shape.path(),
        }
    })
}

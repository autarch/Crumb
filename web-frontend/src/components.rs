use crate::prelude::*;
use dioxus::router::Link;

#[inline_props]
pub(crate) fn PageTitle<'a>(cx: Scope, children: Element<'a>) -> Element {
    cx.render(rsx! {
        h1 {
            class: DC![C.typ.text_2xl, C.spc.mb_4],
            children,
        },
    })
}

#[inline_props]
pub(crate) fn SubTitle<'a>(cx: Scope, children: Element<'a>) -> Element {
    cx.render(rsx! {
        h2 {
            class: DC![C.typ.text_xl, C.spc.mb_1],
            children,
        },
    })
}

pub(crate) enum Color {
    Indigo,
    White,
}

#[derive(Props)]
pub(crate) struct UnderlineLinkProps<'a> {
    to: &'a str,
    #[props(default, strip_option)]
    color: Option<Color>,
    children: Element<'a>,
}

pub(crate) fn UnderlineLink<'a>(cx: Scope<'a, UnderlineLinkProps<'a>>) -> Element<'a> {
    let class = C![
        M![M.hover, C.bor.border_b],
        // We need to use an enum here so that each possible class is written
        // out statically, as opposed to generating it dynamically from
        // input. That way the tailwindcss extractor can see all the possible
        // classes.
        match cx.props.color {
            Some(Color::Indigo) => M![M.hover, C.bor.border_indigo_400],
            Some(Color::White) => M![M.hover, C.bor.border_white],
            None => M![M.hover, C.bor.border_black],
        },
    ];
    cx.render(rsx! {
        Link {
            class: "{class}",
            to: cx.props.to,
            &cx.props.children,
        },
    })
}

#[derive(Props)]
pub struct AlbumCoverProps<'a> {
    #[props(default, strip_option)]
    class: Option<String>,
    #[props(default)]
    uri: Option<&'a str>,
    #[props(default = 200)]
    size: u16,
    #[props(default = true)]
    round: bool,
    #[props(default = true)]
    border: bool,
}

pub(crate) fn AlbumCover<'a>(cx: Scope<'a, AlbumCoverProps<'a>>) -> Element<'a> {
    let src = match cx.props.uri {
        // All the thumb sized images are JPEGs, I think.
        Some(u) => u
            .replace(".jpg", "_thumb250.jpg")
            .replace(".png", "_thumb250.jpg")
            .replace(".gif", "_thumb250.jpg"),
        None => "https://dummyimage.com/200x200/fff/aaa".to_string(),
    };
    let mut class = cx.props.class.as_deref().unwrap_or("").to_string();
    if cx.props.round {
        class.push_str(&C![C.bor.rounded_full]);
        if cx.props.border {
            class.push(' ');
            class.push_str(&C![
                C.bor.ring_4,
                C.bor.ring_indigo_500,
                C.bor.ring_opacity_50
            ]);
        }
    }
    cx.render(rsx! {
        img {
            // This is already set on the containing a{} element. Is needing
            // to set this here as well a dioxus bug? Maybe?
            prevent_default: "onclick",
            class: "{class}",
            height: "{cx.props.size}",
            width: "{cx.props.size}",
            src: "{src}",
        }
    })
}

#[inline_props]
pub(crate) fn UserFacingError<'a>(cx: Scope, error: &'a crate::client::Status) -> Element {
    cx.render(rsx! {
        PageTitle {
            "Error"
        },
        div {
            class: DC![C.lay.flex, C.fg.flex_row, C.fg.flex_wrap C.fg.justify_center],
            "{error.message}",
        }
    })
}

#[derive(Props)]
pub(crate) struct TableAttributes<'a> {
    #[props(default, strip_option)]
    class: Option<&'a str>,
    children: Element<'a>,
}

pub(crate) fn Table<'a>(cx: Scope<'a, TableAttributes<'a>>) -> Element<'a> {
    let class = cx.props.class.unwrap_or_default();
    cx.render(rsx! {
        table {
            class: "{class}",
            &cx.props.children,
        },
    })
}

#[derive(Props)]
pub(crate) struct TrAttributes<'a> {
    #[props(default, strip_option)]
    class: Option<&'a str>,
    children: Element<'a>,
}

pub(crate) fn Tr<'a>(cx: Scope<'a, TrAttributes<'a>>) -> Element<'a> {
    let mut class = cx.props.class.unwrap_or_default().to_string();
    class.push(' ');
    class.push_str(&C![C.bor.border_y, C.bor.border_indigo_200]);
    cx.render(rsx! {
        tr {
            class: "{class}",
            &cx.props.children,
        },
    })
}

#[derive(Props)]
pub(crate) struct TdAttributes<'a> {
    #[props(default, strip_option)]
    class: Option<&'a str>,
    #[props(default, strip_option)]
    colspan: Option<u16>,
    children: Element<'a>,
}

pub(crate) fn Td<'a>(cx: Scope<'a, TdAttributes<'a>>) -> Element<'a> {
    let class = cx.props.class.unwrap_or_default();
    let colspan = cx.props.colspan.unwrap_or(1);
    cx.render(rsx! {
        td {
            class: "{class}",
            colspan: "{colspan}",
            &cx.props.children,
        },
    })
}

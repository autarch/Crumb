use crate::prelude::*;

pub(crate) fn Loading(cx: Scope) -> Element {
    cx.render(rsx! {
        section {
            "Loading",
        }
    })
}

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

#[derive(Props)]
pub struct AlbumCoverProps<'a> {
    #[props(default, strip_option)]
    class: Option<String>,
    #[props(default)]
    uri: Option<&'a str>,
    #[props(default = 200)]
    size: u16,
    #[props(default = true)]
    ring: bool,
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
    class.push_str(" rounded-full");
    if cx.props.ring {
        class.push_str(" ring-4 ring-indigo-500 ring-opacity-50");
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

// #[derive(Props)]
// struct TableAttributes<'a> {
// //    #[props(default, strip_option)]
// //    attributes: Attributes<'a>,
//     children: Element<'a>,
// }

// pub(crate) fn Table<'a>(cx: Scope<'a, TableAttributes<'a>>)  -> Element<'a> {
//     cx.render(rsx! {
//         table {
// //            ..cx.props.attributes,
//             &cx.props.children,
//         },
//     })
// }

// #[derive(Props)]
// struct TrAttributes<'a> {
//     attributes: Attributes<'a>,
//     children: Element<'a>,
// }

// pub(crate) fn Tr<'a>(cx: Scope<'a, TrAttributes<'a>>)  -> Element<'a> {
//     cx.render(rsx! {
//         tr {
//             class: "py-2 border-y border-indigo-200",
// //            ..cx.props.attributes,
//             &cx.props.children,
//         },
//     })
// }

// #[derive(Props)]
// struct TdAttributes<'a> {
//     attributes: Attributes<'a>,
//     children: Element<'a>,
// }

// pub(crate) fn Td<'a>(cx: Scope<'a, TdAttributes<'a>>)  -> Element<'a> {
//     cx.render(rsx! {
//         tr {
//             class: "pr-4",
// //            ..cx.props.attributes,
//             &cx.props.children,
//         },
//     })
// }

use dioxus::prelude::*;

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
            class: "text-2xl",
            children,
        },
    })
}

#[inline_props]
pub(crate) fn SubTitle<'a>(cx: Scope, children: Element<'a>) -> Element {
    cx.render(rsx! {
        h2 {
            class: "text-xl",
            children,
        },
    })
}

#[derive(Props)]
pub struct AlbumCoverProps<'a> {
    #[props(default, strip_option)]
    class: Option<&'a str>,
    #[props(default)]
    uri: Option<&'a str>,
    #[props(default = 200)]
    size: u16,
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
    let mut class = cx.props.class.unwrap_or("").to_string();
    class.push_str(" rounded-full ring-4 ring-indigo-500 ring-opacity-50");
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
            class: "flex flex-row flex-wrap justify-center",
            "{error.message}",
        }
    })
}

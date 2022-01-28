use crate::{
    css,
    icons::{IconButton, Shape},
};
use dioxus::{prelude::*, router::*};

pub(crate) fn Menu(cx: Scope) -> Element {
    let classes = &[
        "fixed",
        "top-0",
        "flex",
        "flex-wrap",
        "place-items-center",
        "w-screen",
        "h-14",
        "border-b-2",
        "border-gray-500",
        "bg-indigo-600",
        "text-white",
    ]
    .join(" ");
    let classes = css::Classes::builder()
        .classes(classes)
        .with_standard_padding(true)
        .build();

    cx.render(rsx! {
        section {
            class: format_args!("{}", classes),
            HomeLink {},
            HamburgerButton {},
            MenuItems {},
        },
    })
}

pub(crate) fn HomeLink(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "flex items-center flex-shrink-0 mr-8",
            span {
                class: "font-bold text-xl tracking-tight",
                Link {
                    to: "/"
                    title: "Home",
                    "Crumb",
                },
            },
        }
    })
}

pub(crate) fn HamburgerButton(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "block lg:hidden",
            IconButton {
                class: "flex items-center py-2 mr-8",
                title: "Navigation",
                shape: Shape::Hamburger,
            },
        },
    })
}

pub(crate) fn MenuItems(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "w-full lg:w-auto block lg:flex lg:flex-grow lg:items-center",
            div {
                class: "lg:flex-grow",
                MenuItem {
                    text: "Artists",
                    href: "/artists",
                    title: "All artists",
                },
                MenuItem {
                    text: "Releases",
                    href: "/releases",
                    title: "All releases",
                },
                MenuItem {
                    text: "Tracks",
                    href: "/tracks",
                    title: "All tracks",
                },
                MenuItem {
                    text: "Queue",
                    href: "/queue",
                    title: "Current queue",
                },
            },
            div {
                class: "mr-0 lg:mr-8",
                IconButton {
                    class: "inline-block leading-none",
                    title: "Your settings",
                    size: 30,
                    shape: Shape::Cog,
                },
            },
        },
    })
}

#[inline_props]
pub(crate) fn MenuItem(
    cx: Scope,
    text: &'static str,
    href: &'static str,
    title: &'static str,
) -> Element {
    let location = use_route(&cx).current_location();
    let is_active = location.path() == *href;
    let mut classes: Vec<_> = vec![
        // These first two styles are for the hamburger view, where the items
        // are stacked vertically. Then we override for a large screen to get
        // a side-by-side layout.
        "block",
        "mt-4",
        "lg:inline-block",
        "lg:mt-0",
        "mr-8",
        "p-2",
        "rounded",
    ];
    if is_active {
        classes.push("text-slate-50 bg-blue-500");
    } else {
        classes.push("text-slate-100");
    }
    let class = classes.join(" ");
    cx.render(rsx! {
        Link {
            class: "{class}",
            to: "{href}",
            title: "{title}",
            "{text}",
        }
    })
}

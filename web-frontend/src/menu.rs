use crate::{css::Classes, prelude::*};
use dioxus::router::*;
use dioxus_heroicons::{solid::Shape, IconButton};

pub(crate) fn Menu(cx: Scope) -> Element {
    let classes = C![
        C.lay.fixed,
        C.lay.top_0,
        C.lay.flex,
        C.fg.flex_wrap,
        C.fg.place_items_center,
        C.siz.w_screen,
        C.siz.h_14,
        C.bor.border_b_2,
        C.bg.bg_indigo_600,
        C.typ.text_white,
    ];
    let class = Classes::builder()
        .classes(classes)
        .with_standard_padding(true)
        .build();

    cx.render(rsx! {
        section {
            class: "{class}",
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
    let class = C![C.lay.flex, C.fg.items_center, C.spc.py_2, C.spc.mr_8];
    cx.render(rsx! {
        div {
            class: "block lg:hidden",
            IconButton {
                class: "{class}",
                title: "Navigation",
                icon: Shape::Menu
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
                    icon: Shape::Cog,
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

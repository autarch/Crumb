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
            class: DC![C.lay.flex, C.fg.items_center, C.fg.flex_shrink_0, C.spc.mr_8],
            span {
                class: DC![C.typ.font_bold, C.typ.text_xl, C.typ.tracking_tight],
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
            class: DC![C.lay.block, M![M.lg, C.lay.hidden]],
            IconButton {
                class: "{class}",
                title: "Navigation",
                icon: Shape::Menu
            },
        },
    })
}

pub(crate) fn MenuItems(cx: Scope) -> Element {
    let cog_class = C![C.lay.inline_block, C.typ.leading_none];
    cx.render(rsx! {
        div {
            class: DC![
                C.siz.w_full,
                M![M.lg, C.siz.w_auto],
                C.lay.block,
                M![M.lg, C.lay.flex],
                M![M.lg, C.fg.flex_grow],
                M![M.lg, C.fg.items_center],
            ],
            div {
                class: DC![M![M.lg, C.fg.flex_grow]],
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
                class: DC![C.spc.mr_8, M![M.lg, C.spc.mr_8]],
                IconButton {
                    class: "{cog_class}",
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
    let class = C![
        C.lay.block,
        C.spc.mt_4,
        M![M.lg, C.lay.inline_block],
        M![M.lg, C.spc.mt_0],
        M![M.lg, C.spc.mr_8,]
        C.spc.p_2,
        C.bor.rounded,
        C.typ.text_slate_100,
    ];
    let active_class = C![C.typ.text_slate_50, C.bg.bg_blue_500];
    cx.render(rsx! {
        Link {
            class: "{class}",
            active_class: "{active_class}",
            to: "{href}",
            title: "{title}",
            "{text}",
        }
    })
}

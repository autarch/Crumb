use crate::{generated::css_classes::C, icons, Page, Urls};
use seed::{prelude::*, *};

#[derive(Clone, Debug)]
pub enum Msg {
    Dummy,
    IconsMsg(icons::Msg),
}

#[allow(clippy::too_many_lines)]
pub fn view(page: &Page, base_url: &Url) -> Node<Msg> {
    nav![
        C![
            C.fixed,
            C.top_0,
            C.flex,
            C.items_center,
            C.justify_between,
            C.flex_wrap,
            C.w_screen,
            // There needs to be padding on the page body that matches this.
            C.h_14,
            C.border_b_2,
            C.border_gray_500,
            C.bg_indigo_600,
            C.text_white,
            C.p_2,
        ],
        home_link(),
        optional_hamburger_icon(),
        menu_items(page, base_url),
    ]
}

fn home_link() -> Node<Msg> {
    div![
        C![
            C.flex,
            C.items_center,
            C.flex_shrink_0,
            C.mr_8,
            C.ml_0,
            C.lg__ml_2,
        ],
        span![
            C![C.font_bold, C.text_xl, C.tracking_tight],
            a![
                attrs! {
                    At::Href => "/",
                    At::Title => "Home",
                },
                "Crumb"
            ],
        ]
    ]
}

fn optional_hamburger_icon() -> Node<Msg> {
    div![
        C![C.block, C.lg__hidden],
        button![
            C![C.flex, C.items_center, C.py_2, C.mr_8],
            icons::menu()
                .classes(vec![C.fill_current])
                .size(20)
                .title("Navigation")
                .build()
                .into_svg()
                .map_msg(Msg::IconsMsg),
        ],
    ]
}

fn menu_items(page: &Page, base_url: &Url) -> Node<Msg> {
    div![
        C![
            C.w_full,
            C.block,
            C.flex_grow,
            C.lg__flex,
            C.lg__items_center,
            C.lg__w_auto,
        ],
        div![
            C![C.lg__flex_grow, C.text_base],
            menu_item(
                matches!(page, Page::Artists(_)),
                "Artists",
                Urls::new(base_url).artists(),
                "All artists",
            ),
            menu_item(
                matches!(page, Page::Albums),
                "Albums",
                Urls::new(base_url).albums(),
                "All albums",
            ),
            menu_item(
                matches!(page, Page::Tracks),
                "Tracks",
                Urls::new(base_url).tracks(),
                "All tracks",
            ),
            menu_item(
                matches!(page, Page::Queue(_)),
                "Queue",
                Urls::new(base_url).queue(),
                "Current queue",
            ),
        ],
        div![
            C![C.mr_0, C.lg__mr_8],
            button![
                C![
                    C.inline_block,
                    C.leading_none,
                    C.mt_4,
                    C.lg__mt_0,
                    C.text_lg,
                ],
                icons::cog()
                    .size(30)
                    .title("Your preferences")
                    .build()
                    .into_svg()
                    .map_msg(Msg::IconsMsg),
            ]
        ],
    ]
}

fn menu_item(is_active: bool, text: &str, href: Url, title: &str) -> Node<Msg> {
    a![
        C![
            // These first two styles are for the hamburger view, where the
            // items are stacked vertically. Then we override for a large
            // screen to get a side-by-side layout.
            C.block,
            C.mt_4,
            C.lg__inline_block,
            C.lg__mt_0,
            C.mr_8,
            C.bg_blue_500,
            IF![ is_active => C.bg_blue_600 ],
            C.p_2,
            C.rounded,
        ],
        attrs! {
            At::Href => href,
            At::Title => title,
        },
        text,
    ]
}

use crate::{prelude::*, util::get_element, ContextMenus};
use dioxus::{events::MouseEvent, router::Link};
use web_sys::HtmlElement;

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
    #[props(!optional)]
    uri: Option<&'a str>,
    #[props(default = 200)]
    size: u16,
    #[props(default = true)]
    round: bool,
    #[props(default = true)]
    border: bool,
    #[props(default = false)]
    lazy: bool,
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
    let loading = if cx.props.lazy { "lazy" } else { "eager" };
    cx.render(rsx! {
        img {
            // This is already set on the containing a{} element. Is needing
            // to set this here as well a dioxus bug? Maybe?
            prevent_default: "onclick",
            class: "{class}",
            height: "{cx.props.size}",
            width: "{cx.props.size}",
            src: "{src}",
            "loading": "{loading}",
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

#[derive(Props)]
pub(crate) struct ContextMenuAttributes<'a> {
    id: &'a str,
    #[props(default, strip_option)]
    class: Option<&'a str>,
    children: Element<'a>,
}

pub(crate) fn ContextMenu<'a>(cx: Scope<'a, ContextMenuAttributes<'a>>) -> Element<'a> {
    let context_menus = use_context::<ContextMenus>(&cx).unwrap();
    let context_menu_id = cx.props.id;

    (*context_menus.write_silent()).register(context_menu_id);
    let context_menu_is_enabled = context_menus.read().is_enabled(context_menu_id);

    let mut context_menu_classes = vec![
        C.lay.absolute,
        C.lay.z_50,
        C.siz.w_1_of_12,
        C.spc.py_1,
        C.spc.px_3,
        C.bg.bg_indigo_600,
        C.typ.text_left,
        C.typ.text_sm,
        C.typ.text_white,
        if context_menu_is_enabled {
            C.lay.visible
        } else {
            C.lay.hidden
        },
    ];
    if let Some(c) = cx.props.class.as_ref() {
        context_menu_classes.push(c);
    }

    let (control_elt, menu_elt) = match cx.props.children {
        Some(VNode::Fragment(f)) => {
            if f.children.len() != 2 {
                panic!("Need exactly two nodes!");
            }
            match (&f.children[0], &f.children[1]) {
                (VNode::Element(c), VNode::Element(r)) => (VNode::Element(c), VNode::Element(r)),
                _ => panic!("The children nodes need to be elements: {:#?}", f.children),
            }
        }
        _ => panic!("urp"),
    };

    let control_id = format!("{}-context-menu-control", context_menu_id);
    let control_onclick = context_menu_onclick(context_menus, context_menu_id, control_id.clone());

    cx.render(rsx! {
        div {
            id: "{context_menu_id}",
            class: DC![context_menu_classes],
            [menu_elt],
        },
        div {
            id: "{control_id}",
            onclick: control_onclick,
            control_elt,
        },
    })
}

fn context_menu_onclick<'a>(
    context_menus: UseSharedState<'a, ContextMenus>,
    context_menu_id: &'a str,
    control_id: String,
) -> impl Fn(MouseEvent) + 'a {
    move |e: MouseEvent| {
        // If we don't cancel this then the page's onclick handler sees the
        // click as well and disables all context menus.
        e.cancel_bubble();
        let is_enabled = context_menus.read().is_enabled(context_menu_id);

        let menu = get_element::<HtmlElement>(context_menu_id).unwrap();
        let menu_style = menu.style();

        if is_enabled {
            (*context_menus.write()).disable(&context_menu_id);
            return;
        }

        let control = get_element::<HtmlElement>(&control_id).unwrap();
        let control_rect = control.get_bounding_client_rect();

        menu.set_class_name(&menu.class_name().replace("hidden", "visible"));
        // We need to make it visible so we can calculate its dimensions, but
        // we don't want it to be on screen yet.
        menu_style.set_property("left", "-10000px").unwrap();

        let menu_rect = menu.get_bounding_client_rect();
        // This is distance in pixels from the edge of the control element to
        // the middle of dots icon.
        let width_to_dots = control_rect.width() / 2.0;
        let transform = format!(
            "translateX({}px) translateY({}px)",
            // If the menu is wider than the distance to the dots, this moves
            // it left so the menu's right edge appears above the dots. If
            // it's thinner than the distance to the dots it moves it right.
            width_to_dots - menu_rect.width(),
            -1.0 * (menu_rect.height() + 3.0),
        );
        menu_style.set_property("transform", &transform).unwrap();
        menu_style.remove_property("left").unwrap();

        (*context_menus.write()).enable(context_menu_id);
    }
}

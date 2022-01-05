use crate::{client::ReleaseTrack, components, generated::css_classes::C, icons, Queue};
use seed::{prelude::*, *};
use std::cell::Ref;

#[derive(Debug)]
pub enum Msg {
    SetNowPlaying(ReleaseTrack),
    TogglePlayPause,
    NextTrack,
    PreviousTrack,
    VolumeChange(Volume),
    ToggleMute,
    ThumbsUp(String),
    ThumbsDown(String),

    ComponentsMsg(components::Msg),
    IconsMsg(icons::Msg),
}

#[derive(Clone, Debug)]
pub struct Volume(u8);

pub fn view(model: Option<Ref<Queue>>) -> Node<Msg> {
    match model {
        None => Node::Empty,
        Some(queue) => view_now_playing(queue),
    }
}

fn view_now_playing(queue: Ref<Queue>) -> Node<Msg> {
    div![
        C![
            C.fixed,
            C.inset_x_0,
            C.bottom_0,
            C.z_10,
            // We need to add exactly this much padding to the main body when
            // the now playing div is being displayed in order to make sure
            // that this now playing div doesn't cover main body content.
            C.h_24,
            C.w_screen,
            C.bg_indigo_500,
        ],
        div![C![C.bg_black, C.h_1, C.w_full]],
        div![
            C![C.pl_8, C.pr_8, C.pt_4, C.pb_4],
            div![
                C![C.grid, C.grid_cols_3, C.items_center, C.text_white],
                view_current_track(&queue),
                view_prev_play_pause_next_buttons(&queue),
                view_additional_buttons(&queue),
            ],
        ],
    ]
}

fn view_current_track(queue: &Ref<Queue>) -> Node<Msg> {
    let current = match &queue.current {
        Some(c) => c,
        None => return Node::Empty,
    };
    let artist = current.artist.core.as_ref().unwrap();
    let release = current.release.core.as_ref().unwrap();
    let contents = match queue.is_empty() {
        true => vec![div![C![C.col_span_5], "Queue is empty"]],
        false => match &queue.current {
            None => vec![div![C![C.col_span_5], "Nothing is currently playing."]],
            Some(current) => {
                let current_track = queue.current_track().unwrap();
                vec![
                    components::release_image(artist, release, Some(&[C.h_auto, C.w_16, C.mr_6]))
                        .map_msg(Msg::ComponentsMsg),
                    div![
                        C![C.col_span_4],
                        &current_track.display_title,
                        br![],
                        a![
                            attrs! { At::Href => artist.url() },
                            &artist.display_name,
                        ],
                        " - ",
                        a![
                            attrs! { At::Href => release.url() },
                            &release.display_title,
                        ]
                    ],
                ]
            }
        },
    };

    div![C![C.grid, C.grid_cols_5, C.items_center], contents]
}

fn view_prev_play_pause_next_buttons(queue: &Ref<Queue>) -> Node<Msg> {
    div![
        C![C.flex, C.flex_1, C.items_center, C.justify_center],
        view_previous_button(queue),
        view_play_or_pause_button(queue),
        view_next_button(queue),
    ]
}

fn view_previous_button(queue: &Ref<Queue>) -> Node<Msg> {
    let disabled = !queue.can_move_to_previous();
    let mut icon = icons::rewind()
        .size(35)
        .title("Previous track")
        .build()
        .into_svg()
        .map_msg(Msg::IconsMsg);

    if disabled {
        disable_icon(&mut icon);
    }

    button![
        IF![ disabled => attrs!{ At::Disabled => 1 } ],
        icon,
        ev(Ev::Click, |_| Msg::PreviousTrack),
    ]
}

fn view_play_or_pause_button(queue: &Ref<Queue>) -> Node<Msg> {
    if queue.is_playing {
        return button![
            icons::pause()
                .size(50)
                .title("Pause")
                .build()
                .into_svg()
                .map_msg(Msg::IconsMsg),
            ev(Ev::Click, |_| Msg::TogglePlayPause),
        ];
    }

    let disabled = !queue.can_play();
    let mut play = icons::play()
        .size(50)
        .title("Play")
        .build()
        .into_svg()
        .map_msg(Msg::IconsMsg);

    if disabled {
        disable_icon(&mut play);
    }

    button![
        IF![ disabled => attrs!{ At::Disabled => 1 } ],
        play,
        ev(Ev::Click, |_| Msg::TogglePlayPause),
    ]
}

fn view_next_button(queue: &Ref<Queue>) -> Node<Msg> {
    let disabled = !queue.can_move_to_next();
    let mut icon = icons::fast_forward()
        .size(35)
        .title("Next track")
        .build()
        .into_svg()
        .map_msg(Msg::IconsMsg);
    if disabled {
        disable_icon(&mut icon);
    }

    button![
        IF![ disabled => attrs!{ At::Disabled => 1 } ],
        icon,
        ev(Ev::Click, |_| Msg::NextTrack),
    ]
}

fn disable_icon(icon: &mut Node<Msg>) {
    icon.add_attr("disabled", "1");
    // coolGray 400
    icon.add_attr("fill", "#9CA3AF");
}

fn view_additional_buttons(queue: &Ref<Queue>) -> Node<Msg> {
    div![
        C![C.grid, C.grid_cols_6, C.items_center, C.justify_center],
        div![
            C![C.flex, C.flex_1, C.items_center, C.justify_center],
            button![icons::volume_up()
                .size(30)
                .title("Mute")
                .build()
                .into_svg()
                .map_msg(Msg::IconsMsg)],
        ],
        div![
            C![
                C.col_span_2,
                C.flex,
                C.flex_1,
                C.items_center,
                C.justify_center
            ],
            input![attrs! {
                At::Id => "volume",
                At::Type => "range",
                At::Min => 0,
                At::Max => 100,
                At::Step => 1,
                At::Title => "Volume",
            }],
        ],
        div![
            C![C.flex, C.flex_1, C.items_center, C.justify_center],
            button![icons::dots_vertical()
                .size(30)
                .title("More actions")
                .build()
                .into_svg()
                .map_msg(Msg::IconsMsg)],
        ],
        view_thumb_buttons(queue),
    ]
}

fn view_thumb_buttons(queue: &Ref<Queue>) -> Vec<Node<Msg>> {
    let disabled = queue.current_track().is_none();
    let mut thumbs_up = icons::thumbs_up()
        .size(30)
        .title("I like it")
        .build()
        .into_svg()
        .map_msg(Msg::IconsMsg);
    let mut thumbs_down = icons::thumbs_down()
        .size(30)
        .title("I don't like it")
        .build()
        .into_svg()
        .map_msg(Msg::IconsMsg);
    if disabled {
        disable_icon(&mut thumbs_up);
        disable_icon(&mut thumbs_down);
    }

    vec![
        div![
            C![C.flex, C.flex_1, C.items_center, C.justify_center],
            button![IF![ disabled => attrs!{ At::Disabled => 1 } ], thumbs_up],
        ],
        div![
            C![C.flex, C.flex_1, C.items_center, C.justify_center],
            button![IF![ disabled => attrs!{ At::Disabled => 1 } ], thumbs_down],
        ],
    ]
}

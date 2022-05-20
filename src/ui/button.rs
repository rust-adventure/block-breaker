use bevy::prelude::{Res, World};
use kayak_ui::{
    bevy::ImageManager,
    core::{
        rsx,
        styles::{Edge, Style, StyleProp, Units},
        widget, Bound, Color, CursorIcon, EventType,
        MutableBound, OnEvent, WidgetProps,
    },
    widgets::NinePatch,
};

use crate::assets::ImageAssets;

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct SnakeButtonProps {
    #[prop_field(Styles)]
    pub styles: Option<Style>,
    #[prop_field(OnEvent)]
    pub on_event: Option<OnEvent>,
    #[prop_field(Children)]
    pub children: Option<kayak_ui::core::Children>,
}

#[widget]
pub fn SnakeButton(props: SnakeButtonProps) {
    let button_styles = Style {
        background_color: StyleProp::Value(Color::BLACK),
        height: StyleProp::Value(Units::Pixels(50.0)),
        width: StyleProp::Value(Units::Pixels(200.0)),
        padding_top: StyleProp::Value(Units::Stretch(1.0)),
        padding_bottom: StyleProp::Value(Units::Stretch(
            1.0,
        )),
        padding_left: StyleProp::Value(Units::Stretch(1.0)),
        padding_right: StyleProp::Value(Units::Stretch(
            1.0,
        )),
        cursor: CursorIcon::Hand.into(),
        ..props.styles.clone().unwrap_or_default()
    };

    let (button, button_pressed) = context
        .query_world::<Res<ImageAssets>, _, _>(|assets| {
            (
                assets.button.clone(),
                assets.button_pressed.clone(),
            )
        });

    let (blue_button_handle, blue_button_hover_handle) =
        context
            .get_global_mut::<World>()
            .map(|mut world| {
                let mut image_manager = world
                    .get_resource_mut::<ImageManager>()
                    .unwrap();
                (
                    image_manager.get(&button),
                    image_manager.get(&button_pressed),
                )
            })
            .unwrap();

    let current_button_handle = context
        .create_state::<u16>(blue_button_handle)
        .unwrap();

    let cloned_current_button_handle =
        current_button_handle.clone();
    let parent_on_event = props.on_event.clone();
    let on_event = OnEvent::new(move |ctx, event| {
        match event.event_type {
            EventType::MouseDown(..) => {
                cloned_current_button_handle
                    .set(blue_button_hover_handle);
            }
            EventType::MouseUp(..) => {
                cloned_current_button_handle
                    .set(blue_button_handle);
            }
            EventType::Click(..) => {
                match &parent_on_event {
                    Some(v) => v.try_call(ctx, event),
                    None => todo!(),
                };
            }
            _ => (),
        }
    });

    let children = props.get_children();
    rsx! {
        <NinePatch
            border={Edge::all(24.0)}
            handle={current_button_handle.get()}
            styles={Some(button_styles)}
            on_event={Some(on_event)}
        >
            {children}
        </NinePatch>
    }
}

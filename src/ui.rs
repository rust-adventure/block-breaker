use std::time::Duration;

use bevy::{
    app::AppExit,
    prelude::{
        AssetServer, Commands, EventWriter, Plugin, Res,
        ResMut, World,
    },
};
use iyes_loopless::state::{CurrentState, NextState};
use kayak_ui::{
    bevy::{
        BevyContext, BevyKayakUIPlugin, FontMapping,
        ImageManager, UICameraBundle,
    },
    core::{
        bind, render, rsx,
        styles::{
            Corner, Edge, LayoutType, Style, StyleProp,
            Units,
        },
        use_state, widget, Binding, Bound, Color,
        EventType, Handler, Index, MutableBound, OnEvent,
    },
    widgets::{App, Element, If, NinePatch, Text},
};

use crate::{
    assets::ImageAssets,
    scoring::{HighScore, Score, Timer},
    // settings::GameSettings,
    GameState,
    STARTING_GAME_STATE,
};

mod button;
// mod checkbox;
// mod settings;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(BevyKayakUIPlugin)
            .insert_resource(bind(STARTING_GAME_STATE))
            .add_startup_system(game_ui)
            .add_system(bind_gamestate)
            // .add_system(bind_game_settings)
            .add_system(bind_score)
            .add_system(bind_high_score)
            .add_system(bind_timer);
    }
}

pub fn bind_gamestate(
    state: Res<CurrentState<GameState>>,
    binding: Res<Binding<GameState>>,
) {
    if state.is_changed() {
        binding.set(state.0);
    }
}

// pub fn bind_game_settings(
//     state: Res<GameSettings>,
//     binding: Res<Binding<GameSettings>>,
// ) {
//     if state.is_changed() {
//         binding.set(state.clone());
//     }
// }

pub fn bind_score(
    state: Res<Score>,
    binding: Res<Binding<Score>>,
) {
    if state.is_changed() {
        binding.set(state.clone());
    }
}

pub fn bind_high_score(
    state: Res<HighScore>,
    binding: Res<Binding<HighScore>>,
) {
    if state.is_changed() {
        binding.set(state.clone());
    }
}

pub fn bind_timer(
    state: Res<Timer>,
    binding: Res<Binding<Duration>>,
) {
    match *state {
        Timer {
            start: _,
            runtime: Some(duration),
        } => {
            binding.set(duration);
        }
        Timer {
            start: Some(instant),
            runtime: None,
        } => {
            binding.set(instant.elapsed());
        }
        _ => {
            binding.set(Duration::from_secs(0));
        }
    };
}

// THIS ONLY RUNS ONCE. VERY IMPORTANT FACT.
fn game_ui(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
    // settings: Res<GameSettings>,
    score: Res<Score>,
    high_score: Res<HighScore>,
) {
    commands.spawn_bundle(UICameraBundle::new());
    font_mapping.set_default(
        asset_server.load("roboto.kayak_font"),
    );
    // commands.insert_resource(bind(settings.
    // clone()));
    commands.insert_resource(bind(score.clone()));
    commands.insert_resource(bind(high_score.clone()));
    commands.insert_resource(bind(Duration::from_secs(0)));

    let context = BevyContext::new(|context| {
        render! {
            <App>
                <GameMenu/>
            </App>
        }
    });

    commands.insert_resource(context);
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Menu {
    Main,
    Settings,
}

#[widget]
fn GameMenu() {
    let container_styles = Style {
        border_radius: StyleProp::Value(Corner::all(15.0)),
        background_color: StyleProp::Value(Color::WHITE),
        bottom: StyleProp::Value(Units::Stretch(1.0)),
        height: StyleProp::Value(Units::Pixels(500.0)),
        layout_type: StyleProp::Value(LayoutType::Column),
        left: StyleProp::Value(Units::Stretch(1.0)),
        padding: StyleProp::Value(Edge::all(
            Units::Stretch(1.0),
        )),
        right: StyleProp::Value(Units::Stretch(1.0)),
        row_between: StyleProp::Value(Units::Pixels(20.0)),
        top: StyleProp::Value(Units::Stretch(1.0)),
        width: StyleProp::Value(Units::Pixels(360.0)),
        ..Default::default()
    };
    let gameboard_spacer_styles = Style {
        bottom: StyleProp::Value(Units::Stretch(1.0)),
        layout_type: StyleProp::Value(LayoutType::Column),
        top: StyleProp::Value(Units::Stretch(1.0)),
        width: StyleProp::Value(Units::Pixels(600.0)),
        ..Default::default()
    };

    let row_styles = Style {
        layout_type: StyleProp::Value(LayoutType::Row),
        padding_top: StyleProp::Value(Units::Stretch(1.0)),
        padding_bottom: StyleProp::Value(Units::Stretch(
            1.0,
        )),
        ..Default::default()
    };
    let left_styles = Style {
        padding_left: StyleProp::Value(Units::Stretch(1.0)),
        height: StyleProp::Value(Units::Pixels(600.0)),
        border: StyleProp::Value(Edge::all(25.0)),
        ..Default::default()
    };
    let right_styles = Style {
        height: StyleProp::Value(Units::Pixels(600.0)),
        border: StyleProp::Value(Edge::all(25.0)),
        ..Default::default()
    };

    let (menu_state, set_menu_state, ..) =
        use_state!(Menu::Main);

    // let set_menu = set_menu_state.clone();
    // let set_menu_to_main = Handler::new(move |_| {
    //     set_menu(Menu::Main);
    // });

    let show_menus = {
        let gamestate = context
            .query_world::<Res<Binding<GameState>>, _, _>(
                |state| state.clone(),
            );

        context.bind(&gamestate);
        gamestate.get() == GameState::Menu
    };

    let score = {
        let score = context
            .query_world::<Res<Binding<Score>>, _, _>(
                |state| state.clone(),
            );

        context.bind(&score);
        score.get().score
    };

    let high_score = {
        let score = context
            .query_world::<Res<Binding<HighScore>>, _, _>(
                |state| state.clone(),
            );

        context.bind(&score);
        score.get()
    };

    let current_run_time = {
        let score = context
            .query_world::<Res<Binding<Duration>>, _, _>(
                |state| state.clone(),
            );

        context.bind(&score);
        score.get()
    };

    let panel = context
        .query_world::<Res<ImageAssets>, _, _>(|assets| {
            assets.panel.clone()
        });

    let container = context
        .get_global_mut::<World>()
        .map(|mut world| {
            world
                .get_resource_mut::<ImageManager>()
                .unwrap()
                .get(&panel)
        })
        .unwrap();

    let on_click_new_game =
        OnEvent::new(|ctx, event| match event.event_type {
            EventType::Click(..) => {
                let mut world =
                    ctx.get_global_mut::<World>().unwrap();
                world.insert_resource(NextState(
                    GameState::Playing,
                ));
            }
            _ => {}
        });

    let set_menu = set_menu_state.clone();
    let on_click_settings =
        OnEvent::new(move |_, event| {
            match event.event_type {
                EventType::Click(..) => {
                    set_menu(Menu::Settings);
                }
                _ => {}
            }
        });

    let on_click_exit =
        OnEvent::new(|ctx, event| match event.event_type {
            EventType::Click(..) => {
                ctx
                .query_world::<EventWriter<AppExit>, _, _>(
                    |mut exit| {
                        exit.send(AppExit);
                    },
                );
            }
            _ => {}
        });

    let show_main_menu = menu_state == Menu::Main;
    let show_settings_menu = menu_state == Menu::Settings;

    rsx! {
    <Element styles={Some(row_styles)}>
      <Element styles={Some(left_styles)}>
    //     <Text
    //       size={50.0}
    //       content={"Current Run".to_string()}
    //     />
    //     <Text
    //       size={50.0}
    //       content={score.to_string()}
    //     />
    //     <Text
    //       size={25.0}
    //       content={format!("{} seconds",current_run_time.as_secs().to_string())}
    //     />
      </Element>
      <Element styles={Some(gameboard_spacer_styles)}>
        <If condition={show_menus}>
          <If condition={show_main_menu}>
            <NinePatch
              styles={Some(container_styles)}
              border={Edge::all(10.0)}
              handle={container}
            >
              <button::SnakeButton
                on_event={Some(on_click_new_game)}
              >
                <Text
                    size={20.0}
                    content={"New Game".to_string()}
                />
              </button::SnakeButton>
              <button::SnakeButton
                on_event={Some(on_click_settings)}
              >
                <Text
                    size={20.0}
                    content={"Settings".to_string()}
                />
              </button::SnakeButton>
              <button::SnakeButton
                on_event={Some(on_click_exit)}
              >
                <Text
                    size={20.0}
                    content={"Exit".to_string()}
                />
              </button::SnakeButton>
            </NinePatch>
          </If>
        //   <If condition={show_settings_menu}>
        //     <settings::SettingsMenu
        //       back={set_menu_to_main}
        //     />
        //   </If>
        </If>
      </Element>
      <Element styles={Some(right_styles)}>
    //     <Text
    //       size={50.0}
    //       content={"High Score".to_string()}
    //     />
    //     <Text
    //       size={50.0}
    //       content={high_score.score.to_string()}
    //     />
    //     <Text
    //       size={25.0}
    //       content={format!("{} seconds",high_score.time.as_secs().to_string())}
    //     />
      </Element>
    </Element>
        }
}

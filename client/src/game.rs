use std::num::NonZeroU8;

use bevy::{asset::LoadState, prelude::*, window::{PrimaryWindow, WindowMode, WindowResolution}};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .add_systems(Startup, startup)
            .add_systems(
                Update,
                (
                    check_assets.run_if(in_state(GameState::Startup)),
                    draw_main_menu_bg.run_if(in_state(GameState::MainMenu))
                )
            )
            .add_systems(OnEnter(GameState::MainMenu), create_window);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum LoadingState {
    CreatingRoom,
    FindingRoom,
    JoiningRoom,
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Copy, Default)]
pub enum GameState {
    #[default]
    Startup,
    MainMenu,
    Loading(LoadingState),
    Lobby,
    Game,
}

#[derive(Resource)]
pub struct MapTextures {
    pub blurred: Handle<Image>,
    pub clear: Handle<Image>,
}

#[derive(Resource)]
pub struct ButtonTextures {
    pub create_room: Handle<Image>,
    pub join_room: Handle<Image>,
    pub quit: Handle<Image>,
}

#[derive(Resource)]
struct UnloadedImages(Vec<Handle<Image>>);

// pretty much just loads assets, could rename
fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let blurred: Handle<Image> = asset_server.load("background.png");
    let clear: Handle<Image> = asset_server.load("map.png");
    commands.insert_resource(MapTextures { blurred: blurred.clone(), clear: clear.clone() });

    let create_room: Handle<Image> = asset_server.load("create_room_button.png");
    let join_room: Handle<Image> = asset_server.load("join_room_button.png");
    let quit: Handle<Image> = asset_server.load("quit_button.png");
    commands.insert_resource(ButtonTextures {
        create_room: create_room.clone(),
        join_room: join_room.clone(),
        quit: quit.clone(),
    });

    commands.insert_resource(UnloadedImages(vec![blurred, clear, create_room, join_room, quit]));
}

fn create_window(mut commands: Commands) {
    commands.spawn((
        PrimaryWindow,
        Window {
            title: "Bear Island".into(),
            resolution: WindowResolution::new(1920, 1080),
            mode: WindowMode::Fullscreen(MonitorSelection::Primary, VideoModeSelection::Current),
            ..default()
        }
    ));
}

fn draw_main_menu_bg(
    mut commands: Commands,
    backgrounds: Res<MapTextures>,
    window: Option<Single<&Window, With<PrimaryWindow>>>,
    mut has_run: Local<bool>,
) {
    if *has_run { return; };
    let Some(window) = window else { return ;};

    commands.spawn(Camera2d);

    println!("{}*{}", window.width(), window.height());

    commands.spawn(
        Sprite {
            image: backgrounds.blurred.clone(),
            custom_size: Some(Vec2::new(window.width(), window.height())),
            ..default()
        }
    );

    *has_run = true;
}

// why the freak are assets loaded asynchronously
fn check_assets(
    mut images: ResMut<UnloadedImages>,
    asset_server: Res<AssetServer>,
    mut message_writer: MessageWriter<AppExit>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut updated_vec: Vec<Handle<Image>> = Vec::new();

    for handle in &images.0 {
        match asset_server.get_load_state(handle) {
            Some(LoadState::Loading) => {
                updated_vec.push(handle.clone());
            },
            Some(LoadState::Failed(e)) => {
                eprintln!("Error loading asset: {e}");
                message_writer.write(AppExit::Error(NonZeroU8::new(1).unwrap()));
            },
            _ => continue,
        }
    }

    if updated_vec.is_empty() { next_state.set(GameState::MainMenu) } // No more assets left to load finally

    images.0 = updated_vec;
}
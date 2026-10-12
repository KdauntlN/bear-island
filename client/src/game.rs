use std::num::NonZeroU8;

use bevy::{asset::LoadState, prelude::*, window::{PrimaryWindow, WindowMode, WindowResolution}};

use crate::ui::messages::ServerEvent;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .add_sub_state::<LoadingState>()
            .add_systems(Startup, startup)
            .add_systems(Update, (
                check_assets.run_if(in_state(GameState::Startup)),
                draw_main_menu_bg.run_if(in_state(GameState::MainMenu)),
                (draw_loading_screen, check_server_events).run_if(in_state(GameState::Loading)),
            ))
            .add_systems(OnEnter(GameState::MainMenu), create_window);
    }
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Copy, Default)]
pub enum GameState {
    #[default]
    Startup,
    MainMenu,
    Loading,
    Lobby,
    Game,
}

#[derive(SubStates, Debug, Hash, Eq, PartialEq, Clone, Copy, Default)]
#[source(GameState = GameState::Loading)]
pub enum LoadingState {
    #[default]
    CreatingRoom,
    FindingRoom,
    JoiningRoom,
}

#[derive(Resource)]
pub struct MapTextures {
    pub blurred: Handle<Image>,
    pub clear: Handle<Image>,
}

#[derive(Resource)]
pub struct UiTextures {
    pub create_room: Handle<Image>,
    pub join_room: Handle<Image>,
    pub quit: Handle<Image>,
    pub logo_text: Handle<Image>,
}

#[derive(Resource)]
struct UnloadedImages(Vec<Handle<Image>>);

#[derive(Component)]
struct LoadingText;

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
    let logo_text: Handle<Image> = asset_server.load("logo_text.png");

    commands.insert_resource(UiTextures {
        create_room: create_room.clone(),
        join_room: join_room.clone(),
        quit: quit.clone(),
        logo_text: logo_text.clone(),
    });

    commands.insert_resource(UnloadedImages(vec![blurred, clear, create_room, join_room, quit, logo_text]));
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

    commands.spawn(Camera2d);
}

fn draw_main_menu_bg(
    mut commands: Commands,
    backgrounds: Res<MapTextures>,
    window: Option<Single<&Window, With<PrimaryWindow>>>,
    mut has_run: Local<bool>,
) {
    if *has_run { return; };
    let Some(window) = window else { return ;};

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

fn draw_loading_screen(
    mut commands: Commands,
    state: Res<State<LoadingState>>,
    mut text_q: Query<&mut Text, With<LoadingText>>,
    mut last_state: Local<Option<LoadingState>>,
) {
    if *last_state == None {
        *last_state = Some(state.get().clone());
    }

    let message = match state.get() {
        LoadingState::CreatingRoom => "Creating room...",
        LoadingState::FindingRoom => "Finding room...",
        LoadingState::JoiningRoom => "Joining room..."
    };

    if text_q.is_empty() {
        commands.spawn((
            DespawnOnExit(GameState::Loading),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        )).with_child((
            LoadingText,
            Text::new(message),
            TextFont {
                font_size: FontSize::Px(40.0),
                ..default()
            },
            TextColor(Color::WHITE),
        ));
    } else {
        if &last_state.unwrap() != state.get() {
            if let Ok(mut text) = text_q.single_mut() {
                text.0 = message.to_string();
            }
            *last_state = Some(state.get().clone());
        }
    }
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

fn check_server_events(mut reader: MessageReader<ServerEvent>, mut next_state: ResMut<NextState<LoadingState>>) {
    for message in reader.read() {
        match message {
            ServerEvent::RoomCreated { address } => next_state.set(LoadingState::JoiningRoom),
            _ => todo!(),
        }
    }
}
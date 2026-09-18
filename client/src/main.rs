use bevy::window::WindowMode;
use common::{ServerMessage, ClientMessage};

use std::net::{TcpStream};
use std::io::{self, Read, Write};

use bevy::{prelude::*, tasks};
use bevy::tasks::{IoTaskPool, Task};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bear Island".into(),
                    mode: WindowMode::BorderlessFullscreen(
                        MonitorSelection::Primary
                    ),
                    ..default()
                }),
                ..default()
            })
        )
        .init_state::<GameState>()
        .add_systems(OnEnter(GameState::MainMenu), setup)
        .add_systems(Update, (button_system, check_room_creation).run_if(in_state(GameState::MainMenu)))
        .run();
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Copy, Default)]
enum GameState {
    #[default]
    MainMenu,
    JoiningRoom,
    Lobby,
    Game,
}

#[derive(Component)]
struct CreateRoomButton;

#[derive(Component)]
struct MainMenu;

#[derive(Resource)]
struct NetworkTask(Task<io::Result<String>>);

#[derive(Resource)]
struct RoomInfo {
    code: String,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            image: asset_server.load("background.png"),
            custom_size: Some(Vec2::new(1920.0 * 2.0, 1080.0 * 2.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -10.0),
    ));

    commands.spawn((
        MainMenu,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        },
    ))
    .with_children(|parent| {
        parent.spawn((
            Button,
            CreateRoomButton,
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Px(300.0),
                height: Val::Px(50.0),
                border_radius: BorderRadius::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::BLACK),
        ))
        .with_child((
            Text::new("Create Room"),
        ));

        parent.spawn((
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Px(300.0),
                height: Val::Px(50.0),
                border_radius: BorderRadius::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::BLACK),
        ))
        .with_child((
            Text::new("Join Room"),
        ));
    });
}

fn button_system(
    mut commands: Commands,
    query: Query<&Interaction, (Changed<Interaction>, With<CreateRoomButton>)>,
) {
    for interaction in &query {
        if *interaction == Interaction::Pressed {
            let task = IoTaskPool::get().spawn(create_room());

            commands.insert_resource(NetworkTask(task));
        }
    }
}

fn check_room_creation(mut commands: Commands, mut task: Option<ResMut<NetworkTask>>) {
    if let Some(mut task) = task {
        if let Some(result) = tasks::block_on(tasks::poll_once(&mut task.0)) {
            match result {
                Ok(addr) => {
                    println!("Room was created at address {addr}");
                    commands.remove_resource::<NetworkTask>();
                },
                Err(e) => {
                    println!("There was an error: {e}");
                    commands.remove_resource::<NetworkTask>();
                },
            }
        }
    }
}

async fn create_room() -> io::Result<String> {
    println!("Sending create room message");
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    stream.write(&wincode::serialize(&ClientMessage::CreateRoom).unwrap())?;

    let mut response = [0 as u8; 128];
    stream.read(&mut response)?;

    let response = wincode::deserialize::<ServerMessage>(&response).unwrap();

    if let ServerMessage::RoomCreated { address } = response {
        stream.shutdown(std::net::Shutdown::Both)?;
        return Ok(address);
    } else {
        panic!("Invalid response");
    }
}
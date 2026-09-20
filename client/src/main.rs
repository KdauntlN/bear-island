use bevy::window::WindowMode;
use common::{ServerMessage, ClientMessage};

use std::net::{Shutdown, TcpStream};
use std::io::{self, Read, Write, BufReader, BufWriter, ErrorKind};
use std::time::Duration;

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
        .add_systems(OnEnter(GameState::Loading), loading_screen)
        .add_systems(Update, (create_room_button, quit_button).run_if(in_state(GameState::MainMenu)))
        .add_systems(Update, check_room_creation.run_if(
            in_state(GameState::MainMenu)
                .or_else(in_state(GameState::Loading))   
        ))
        .run();
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Copy, Default)]
enum GameState {
    #[default]
    MainMenu,
    Loading,
    JoiningRoom,
    Lobby,
    Game,
}

#[derive(Component)]
struct CreateRoomButton;

#[derive(Component)]
struct JoinRoomButton;

#[derive(Component)]
struct QuitButton;

#[derive(Component)]
struct MainMenu;

#[derive(Resource)]
struct NetworkTask(Task<io::Result<String>>);

#[derive(Resource)]
struct RoomInfo {
    code: String,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        DespawnOnExit(GameState::MainMenu),
        Camera2d
    ));

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
        DespawnOnExit(GameState::MainMenu),
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
            Button,
            JoinRoomButton,
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

        parent.spawn((
            Button,
            QuitButton,
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
            Text::new("Quit"),
        ));
    });
}

fn loading_screen(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(GameState::Loading),
        Camera2d
    ));

    commands.spawn((
        DespawnOnExit(GameState::Loading),
        Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
    )).with_child((
        Text::new("Loading..."),
    ));
}

fn create_room_button(
    mut commands: Commands,
    query: Query<&Interaction, (Changed<Interaction>, With<CreateRoomButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Loading);
            let task = IoTaskPool::get().spawn(create_room());

            commands.insert_resource(NetworkTask(task));
        }
    }
}

fn quit_button(
    query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
    mut app_exit_events: MessageWriter<AppExit>,
) {
    for interaction in &query {
        if *interaction == Interaction::Pressed {
            app_exit_events.write(AppExit::Success);
        }
    }
}

fn check_room_creation(mut commands: Commands, task: Option<ResMut<NetworkTask>>, mut next_state: ResMut<NextState<GameState>>) {
    if let Some(mut task) = task {
        if let Some(result) = tasks::block_on(tasks::poll_once(&mut task.0)) {
            match result {
                Ok(code) => {
                    println!("Room was created with code: {code}");
                    next_state.set(GameState::MainMenu);
                },
                Err(e) => {
                    println!("There was an error: {e}");
                    next_state.set(GameState::MainMenu);
                },
            }
            commands.remove_resource::<NetworkTask>();
        }
    }
}

async fn create_room() -> io::Result<String> {
    println!("Sending create room message");
    let stream = TcpStream::connect("152.69.167.180:27015")?;
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;

    let mut buf_writer = BufWriter::new(stream);

    let message = wincode::serialize(&ClientMessage::CreateRoom).unwrap();
    let size = message.len().to_be_bytes();

    buf_writer.write(&size)?;
    buf_writer.write(&wincode::serialize(&ClientMessage::CreateRoom).unwrap())?;
    buf_writer.flush()?;

    let stream = buf_writer.into_inner()?;
    let mut buf_reader = BufReader::new(stream);

    let mut size_bytes = [0 as u8; 8];
    buf_reader.read_exact(&mut size_bytes)?;

    let size = usize::from_be_bytes(size_bytes);
    let mut response_bytes = vec![0; size];

    buf_reader.read_exact(&mut response_bytes)?;

    let response = match wincode::deserialize::<ServerMessage>(&response_bytes) {
        Ok(message) => message,
        Err(_) => {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                "Response from server was invalid"
            ));
        }
    };

    let stream = buf_reader.into_inner();
    stream.shutdown(Shutdown::Both)?;

    match response {
        ServerMessage::RoomCreated { code } => {
            Ok(code)
        },
        ServerMessage::RoomCreationError { error } => {
            return Err(
                io::Error::new(
                    ErrorKind::Other,
                    error)
            );
        },
        _ => {
            return Err(
                io::Error::new(
                    ErrorKind::InvalidData,
                    "Invalid response from server",
                )
            );
        }
    }
}

async fn join_room(code: String) -> io::Result<RoomInfo> {
    todo!();
}
use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_simple_aabb::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component, Default)]
struct Velocity {
	x: f32,
	y: f32,
}

fn main() {
    App::new()
		.add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())

		.add_plugin(PhysicsPlugin)

        .add_startup_system(setup)
        .add_startup_system(spawn_players)
        .add_startup_system(spawn_tiles)

		.add_system(keyboard_input)
        .add_system(ray_look_at_mouse)
		.add_system(gravity.after(keyboard_input))
		.add_system(move_players.after(keyboard_input))

		.run();
}

fn setup(mut commands: Commands) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_players(mut commands: Commands) {
	let player_size = Vec2::new(40.0, 60.0);

	commands.spawn()
		.insert_bundle(SpriteBundle {
			sprite: Sprite {
                custom_size: Some(player_size),
                color: Color::Rgba { red: 0.69, green: 0.79, blue: 0.61, alpha: 1.0 },
                ..default()
            },
            ..default()
		})
        .insert(KinematicBody {
            shape: Rectangle::new().with_size(player_size),
            ..default()
        })
        .insert(Player)
		.insert(Velocity::default());
}

fn spawn_tiles(mut commands: Commands, windows: Res<Windows>) {
    const TILEMAP: [[u8; 16]; 9] = [
        [1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
        [1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1],
        [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1],
        [1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];

    let window = windows.get_primary().unwrap();
    
    let num_of_v_tiles = TILEMAP.len();
    let num_of_h_tiles = TILEMAP[0].len();
    let tile_size = Vec2::new(
        (window.width() / TILEMAP[0].len() as f32).ceil(),
        (window.height() / TILEMAP.len() as f32).ceil(),
    );
    
    let mut current_pos = Vec3::new(-window.width(), window.height(), 0.0) / 2.0;
    current_pos.x += tile_size.x / 2.0;
    current_pos.y -= tile_size.y / 2.0;

    for v in 0..num_of_v_tiles {
        for h in 0..num_of_h_tiles {
            if TILEMAP[v][h] == 1 { 
                commands.spawn()
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(tile_size),
                        color: if (v + h) % 2 == 0 { Color::DARK_GRAY } else { Color::BLACK },
                        ..default()
                    },
                    transform: Transform::from_translation(current_pos),
                    ..default()
                })
                .insert(StaticBody {
                    shape: Rectangle::new().with_size(tile_size),
                });
            }


            current_pos.x += tile_size.x;
        }

        current_pos.x = (-window.width() + tile_size.x) / 2.0;
        current_pos.y -= tile_size.y;
    }
}

fn keyboard_input(
	keyboard: Res<Input<KeyCode>>,
    mut players: Query<&mut Velocity, With<Player>>,
) {
    const SPEED: f32 = 7.0;
    const JUMP_STRENGTTH: f32 = 10.0;

	for mut vel in players.iter_mut() {
		vel.x = 0.0;
		if keyboard.pressed(KeyCode::D) { vel.x += SPEED; }
		if keyboard.pressed(KeyCode::A) { vel.x -= SPEED; }

		if keyboard.just_pressed(KeyCode::Space) {
			vel.y = JUMP_STRENGTTH;
		}
    }
}

fn ray_look_at_mouse(
    mut rays: Query<(&mut Ray, &Transform), With<Player>>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();

    match window.cursor_position() {
        Some(mut pos) => {
            pos.x -= window.width() / 2.0;
            pos.y -= window.height() / 2.0;
            for (mut r, trans) in rays.iter_mut() {
                r.direction.x = pos.x - trans.translation.x;
                r.direction.y = pos.y - trans.translation.y;
            }
        },
        None => return,
    }
}

fn gravity(mut q: Query<&mut Velocity>) {
	for mut vel in q.iter_mut() {
		vel.y += -0.5;
		vel.y = vel.y.max(-16.0)
	}
}

fn move_players(
    mut players: Query<(&mut KinematicBody, &Velocity), With<Player>>,
) {
	for (mut body, vel) in players.iter_mut() {
        body.motion.x = vel.x;
        body.motion.y = vel.y;
    }
}
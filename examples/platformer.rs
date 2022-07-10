use bevy::prelude::*;
use bevy_simple_aabb::prelude::*;

#[derive(Component, Default)]
struct Player {
    is_on_ground: bool,
}

#[derive(Component)]
struct GroundRay;

#[derive(Component, Default)]
struct Velocity {
	x: f32,
	y: f32,
}

fn main() {
    App::new()
		.add_plugins(DefaultPlugins)

		.add_plugin(PhysicsPlugin)

        .add_startup_system(setup)
        .add_startup_system(spawn_players)
        .add_startup_system(spawn_tiles)

        .add_system(check_player_rays.before(keyboard_input))
        .add_system(debug_player_contacts)
		.add_system(keyboard_input)
		.add_system(gravity.after(keyboard_input))
		.add_system(move_players.after(keyboard_input))

		.run();
}

fn setup(mut commands: Commands) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_players(mut commands: Commands) {
	let player_size = Vec2::new(48.0, 64.0);

	commands.spawn()
		.insert_bundle(SpriteBundle {
			sprite: Sprite {
                custom_size: Some(player_size),
                color: Color::Rgba { red: 0.69, green: 0.79, blue: 0.61, alpha: 1.0 },
                ..default()
            },
            ..default()
		})
        .insert(KinematicBody::new(Rectangle::new().with_size(player_size)))
        .insert(Player::default())
		.insert(Velocity::default())
        .with_children(|parent| {
            parent.spawn_bundle(RaycastBundle {
                raycast: Raycast::new()
                    .with_direction(Vec2::Y * -4.0)
                    .with_offset(Vec2::new((player_size.x - 1.0) / 2.0, -player_size.y / 2.0)),
                ..default()
            })
            .insert(GroundRay);
        })
        .with_children(|parent| {
            parent.spawn_bundle(RaycastBundle {
                raycast: Raycast::new()
                    .with_direction(Vec2::Y * -4.0)
                    .with_offset(Vec2::new(-(player_size.x - 1.0) / 2.0, -player_size.y / 2.0)),
                ..default()
            })
            .insert(GroundRay);
        });
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
                .insert(StaticBody::new(Rectangle::new().with_size(tile_size)));
            }


            current_pos.x += tile_size.x;
        }

        current_pos.x = (-window.width() + tile_size.x) / 2.0;
        current_pos.y -= tile_size.y;
    }
}

fn check_player_rays(
    mut players: Query<(&mut Player, &Children)>,
    ground_rays: Query<&Raycast, With<GroundRay>>
) {
    for (mut player, children) in players.iter_mut() {
        player.is_on_ground = false;
        
        for &child in children.iter() {
            if let Ok(ray) = ground_rays.get(child) {
                player.is_on_ground = player.is_on_ground || ray.is_colliding();
            };
        }
    }
}

fn debug_player_contacts(
    players: Query<&KinematicBody, With<Player>>,
) {
    for player in players.iter() {
        for c in player.contacts() {
            println!("Player is in contact with entity {:?} with normal {}", c.entity(), c.normal());
        }
        println!();
    }
}

fn keyboard_input(
	keyboard: Res<Input<KeyCode>>,
    mut players: Query<(&Player, &mut Velocity)>,
) {
    const SPEED: f32 = 6.0;
    const JUMP_STRENGTTH: f32 = 10.0;

	for (player, mut vel) in players.iter_mut() {
		vel.x = 0.0;
		if keyboard.pressed(KeyCode::D) { vel.x += SPEED; }
		if keyboard.pressed(KeyCode::A) { vel.x -= SPEED; }

		if keyboard.just_pressed(KeyCode::Space) && player.is_on_ground {
            vel.y = JUMP_STRENGTTH;
		}
    }
}

fn gravity(mut q: Query<(&Player, &mut Velocity)>) {
	for (player, mut vel) in q.iter_mut() {
        if player.is_on_ground { continue; }
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
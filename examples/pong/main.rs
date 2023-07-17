use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
mod utils;

mod audio;

fn main() {
    let audio_model = audio::Model::new();
    if let Ok(am) = audio_model {
        // am.output_stream.pause();
        let game = Game { audio_model: am };
        App::new()
            .add_plugins(DefaultPlugins)
            .insert_non_send_resource(game)
            .insert_resource(Scoreboard {
                player_you: 0,
                player_ai: 0,
            })
            .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
            .insert_resource(TargetPosition { factor: 0.0 })
            .insert_resource(SpectrumData { averages: vec![] })
            .add_startup_system(setup)
            .add_system(local_resource_controller)
            .add_system(paddle_movement_system)
            .add_system(spectrum_bar_system)
            .add_system(ball_collision_system)
            .add_system(ball_movement_system)
            .add_system(scoreboard_system)
            // .add_system(bevy::input::system::exit_on_esc_system.system())
            .run();
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // cameras
    commands.spawn(Camera2dBundle::default());
    // spectrum bars
    for i in 0..8 {
        commands
            .spawn(SpriteBundle {
                transform: Transform::from_translation(Vec3::new(
                    -450.0,
                    -210.0 + 60.0 * i as f32,
                    0.0,
                )),
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.5, 0.21),
                    custom_size: Some(Vec2::new(30.0, 30.0)),
                    ..default()
                },
                ..default()
            })
            .insert(SpectrumBar { id: i as u8 });
    }
    // paddle one
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-400.0, 0.0, 0.0)),
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.21),
                custom_size: Some(Vec2::new(30.0, 120.0)),
                ..default()
            },
            ..Default::default()
        })
        .insert(Player::You)
        .insert(Paddle { speed: 500.0 })
        .insert(Collider::Paddle);
    // paddle two
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(400.0, 0.0, 0.0)),
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.21),
                custom_size: Some(Vec2::new(30.0, 120.0)),
                ..default()
            },
            ..Default::default()
        })
        .insert(Player::Ai)
        .insert(Paddle { speed: 500.0 })
        .insert(Collider::Paddle);
    // ball
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0.0, -50.0, 1.0)),
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.21),
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            ..Default::default()
        })
        .insert(Ball {
            velocity: 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
        });
    // scoreboard
    commands.spawn(
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "test",
            TextStyle {
                font: asset_server.load("fonts/emulogic.ttf"),
                font_size: 40.0,
                color: Color::rgb(0.5, 0.5, 1.0),
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Auto,
            ..default()
        }),
    );

    // wals
    let wall_thickness = 10.0;
    let bounds = Vec2::new(900.0, 600.0);

    commands
        // left
        .spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-bounds.x / 2.0, 0.0, 0.0)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
                ..default()
            },
            ..Default::default()
        })
        .insert(Collider::Solid)
        .insert(Collider::ScorableYou)
        .insert(Player::You);
    // right
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(bounds.x / 2.0, 0.0, 0.0)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
                ..default()
            },
            ..Default::default()
        })
        .insert(Collider::Solid)
        .insert(Collider::ScorableAi)
        .insert(Player::Ai);
    // bottom
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0.0, -bounds.y / 2.0, 0.0)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
                ..default()
            },
            ..Default::default()
        })
        .insert(Collider::Solid);
    // top
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0.0, bounds.y / 2.0, 0.0)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
                ..default()
            },
            ..Default::default()
        })
        .insert(Collider::Solid);
}

fn local_resource_controller(world: &mut World) {
    let mut samples = vec![];
    for _ in 0..1024 {
        let sample = match world
            .get_non_send_resource_mut::<Game>()
            .unwrap()
            .audio_model
            .consumer
            .pop()
        {
            Some(s) => s,
            None => 0.0,
        };

        samples.push(sample);
    }

    let fft_input: Vec<_> = samples
        .iter()
        .map(|frame| std::convert::From::from(*frame as f64))
        .collect();
    let fft_output = kopek::fft::fft(&fft_input);
    let frequency_domain = utils::get_frequency_domain_graph(&fft_output, 1.0);
    let average_bins = utils::get_narrow_bar_spectrum_low(&frequency_domain);

    let mut max = 0.0;
    let mut index = 0;
    for (i, bin) in average_bins.iter().enumerate() {
        if *bin > max {
            max = *bin;
            index = i;
        }
    }

    world.get_resource_mut::<TargetPosition>().unwrap().factor = if max > 50.0 {
        -1.0 + index as f32 * 0.5
    } else {
        0.0
    };
    world.get_resource_mut::<SpectrumData>().unwrap().averages = average_bins;
}

fn spectrum_bar_system(
    spectrum_data: Res<SpectrumData>,
    mut query: Query<(&SpectrumBar, &mut Transform)>,
) {
    if spectrum_data.averages.len() == 8 {
        let mut i = 0;
        for (_, mut transform) in query.iter_mut() {
            transform.scale.x = spectrum_data.averages[i] / 100.0;
            i += 1;
        }
    }
}

fn paddle_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    target_position: Res<TargetPosition>,
    mut query: ParamSet<(
        Query<(&Paddle, &Player, &mut Transform)>,
        Query<(&Ball, &Transform)>,
    )>,
    // mut paddle_query: Query<(&Paddle, &Player, &mut Transform)>,
    // ball_query: Query<(&Ball, &Transform)>,
) {
    let mut ball_y = 0.0;
    for (_, transform) in query.p1().iter() {
        ball_y = transform.translation.y;
    }
    for (paddle, player, mut transform) in query.p0().iter_mut() {
        if let Player::You = *player {
            let mut direction = 0.0;
            let factor = target_position.factor;
            if keyboard_input.pressed(KeyCode::A) {
                direction += 0.25;
            }

            if keyboard_input.pressed(KeyCode::Z) {
                direction -= 0.25;
            }

            direction += factor * 0.75;
            // println!("target position: {}", target_position.index);

            let translation = &mut transform.translation;
            // move the paddle horizontally
            translation.y += time.delta_seconds() * direction * paddle.speed;

            // *translation.y_mut() = -200.0 + target_position.index as f32 * 50.0;
            // bound the paddle within the walls
            translation.y = translation.y.min(220.0).max(-220.0);
        } else if let Player::Ai = *player {
            let mut direction = 0.0;
            if keyboard_input.pressed(KeyCode::J) {
                direction += 1.0;
            }

            if keyboard_input.pressed(KeyCode::N) {
                direction -= 1.0;
            }

            direction = (ball_y - transform.translation.y) / 200.0;
            let translation = &mut transform.translation;
            translation.y += time.delta_seconds() * direction * paddle.speed;
            translation.y = translation.y.min(220.0).max(-220.0);
        }
    }
}

fn ball_movement_system(time: Res<Time>, mut ball_query: Query<(&Ball, &mut Transform)>) {
    // clamp the timestep to stop the ball from escaping when the game starts
    let delta_seconds = f32::min(0.2, time.delta_seconds());

    for (ball, mut transform) in ball_query.iter_mut() {
        transform.translation += ball.velocity * delta_seconds;
    }
}

fn scoreboard_system(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[0].value = format!(
        "YOU:{}                       {}:AI",
        scoreboard.player_you, scoreboard.player_ai
    );
}

fn ball_collision_system(
    mut _commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
    collider_query: Query<(Entity, &Collider, &Transform, &Sprite)>,
) {
    for (mut ball, ball_transform, sprite) in ball_query.iter_mut() {
        let ball_size = sprite.custom_size.unwrap();
        let velocity = &mut ball.velocity;

        // check collision with walls
        for (_collider_entity, collider, transform, sprite) in collider_query.iter() {
            let collision = collide(
                ball_transform.translation,
                ball_size,
                transform.translation,
                sprite.custom_size.unwrap(),
            );
            if let Some(collision) = collision {
                // scorable colliders should be despawned and increment the scoreboard on collision
                if let Collider::ScorableYou = *collider {
                    scoreboard.player_ai += 1;
                } else if let Collider::ScorableAi = *collider {
                    scoreboard.player_you += 1;
                }

                // reflect the ball when it collides
                let mut reflect_x = false;
                let mut reflect_y = false;

                // only reflect if the ball's velocity is going in the opposite direction of the collision
                match collision {
                    Collision::Left => reflect_x = velocity.x > 0.0,
                    Collision::Right => reflect_x = velocity.x < 0.0,
                    Collision::Top => reflect_y = velocity.y < 0.0,
                    Collision::Bottom => reflect_y = velocity.y > 0.0,
                    Collision::Inside => (),
                }

                // reflect velocity on the x-axis if we hit something on the x-axis
                if reflect_x {
                    velocity.x = -velocity.x;
                }

                // reflect velocity on the y-axis if we hit something on the x-axis
                if reflect_y {
                    velocity.y = -velocity.y;
                }

                // break if this collidera is on a solid, otherwise continue check whether a solid is also in collision
                if let Collider::Solid = *collider {
                    break;
                }
            }
        }
    }
}

#[derive(Component)]
struct SpectrumBar {
    id: u8,
}

#[derive(Resource)]
struct SpectrumData {
    averages: Vec<f32>,
}

#[derive(Component, Debug)]
enum Player {
    You,
    Ai,
}

struct Game {
    audio_model: audio::Model,
}

#[derive(Resource)]
struct TargetPosition {
    factor: f32,
}

#[derive(Component)]
struct Paddle {
    speed: f32,
}

#[derive(Component)]
struct Ball {
    velocity: Vec3,
}

#[derive(Resource)]
struct Scoreboard {
    player_you: u8,
    player_ai: u8,
}

#[derive(Component)]
enum Collider {
    Solid,
    ScorableYou,
    ScorableAi,
    Paddle,
}

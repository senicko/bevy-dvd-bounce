use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(DvdPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

pub struct DvdPlugin;

impl Plugin for DvdPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(dvd_setup)
            .add_system(movement)
            .add_system(bounce);
    }
}

#[derive(Component)]
struct MoveSpeed(f32);

#[derive(Component)]
struct SpeedMultiplier {
    x: f32,
    y: f32,
}

#[derive(Bundle)]
struct DvdBundle {
    move_speed: MoveSpeed,
    speed_multiplier: SpeedMultiplier,
    #[bundle]
    logo: SpriteBundle,
}

fn dvd_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(DvdBundle {
        move_speed: MoveSpeed(400.),
        speed_multiplier: SpeedMultiplier { x: 1., y: 1. },
        logo: SpriteBundle {
            texture: asset_server.load("logo.png"),
            transform: Transform::from_scale(Vec3::new(0.5, 0.5, 1.)),
            sprite: Sprite {
                color: Color::RED,
                ..default()
            },
            ..default()
        },
    });
}

fn movement(time: Res<Time>, mut query: Query<(&MoveSpeed, &SpeedMultiplier, &mut Transform)>) {
    for (move_speed, speed_multiplier, mut transform) in query.iter_mut() {
        transform.translation.x += move_speed.0 * speed_multiplier.x * time.delta_seconds();
        transform.translation.y += move_speed.0 * speed_multiplier.y * time.delta_seconds();
    }
}

fn bounce(
    mut query: Query<(
        &Handle<Image>,
        &mut SpeedMultiplier,
        &Transform,
        &mut Sprite,
    )>,
    windows: Res<Windows>,
    assets: Res<Assets<Image>>,
) {
    let (logo, mut speed_multiplier, transform, mut sprite) = query.single_mut();

    if let Some(logo) = assets.get(logo) {
        let sprite_width = logo.texture_descriptor.size.width as f32 * transform.scale.x;
        let sprite_height = logo.texture_descriptor.size.height as f32 * transform.scale.y;

        let window = windows.primary();
        let window_width = window.requested_width();
        let window_height = window.requested_height();

        if transform.translation.x + sprite_width / 2. > window_width / 2. {
            speed_multiplier.x = -1.;
            sprite.color = Color::BLUE;
        } else if transform.translation.x - sprite_width / 2. < -window_width / 2. {
            speed_multiplier.x = 1.;
            sprite.color = Color::GREEN;
        }

        if transform.translation.y + sprite_height / 2. > window_height / 2. {
            speed_multiplier.y = -1.;
            sprite.color = Color::YELLOW;
        } else if transform.translation.y - sprite_height / 2. < -window_height / 2. {
            speed_multiplier.y = 1.;
            sprite.color = Color::RED;
        }
    }
}

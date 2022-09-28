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
        app.add_startup_system(dvd_setup).add_system(bounce);
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
        move_speed: MoveSpeed(350.),
        speed_multiplier: SpeedMultiplier { x: 1., y: 1. },
        logo: SpriteBundle {
            texture: asset_server.load("logo.png"),
            transform: Transform::from_scale(Vec3::new(0.2, 0.2, 1.)),
            ..default()
        },
    });
}

fn bounce(
    time: Res<Time>,
    mut query: Query<(
        &MoveSpeed,
        &Handle<Image>,
        &mut SpeedMultiplier,
        &mut Transform,
    )>,
    windows: Res<Windows>,
    assets: Res<Assets<Image>>,
) {
    for (move_speed, logo, mut speed_multiplier, mut transform) in query.iter_mut() {
        if let Some(logo) = assets.get(logo) {
            for window in windows.iter() {
                let width = window.requested_width();
                let height = window.requested_height();

                let right_edge = transform.translation.x
                    + (logo.texture_descriptor.size.width as f32 / 2.) * transform.scale.x;

                let left_edge = transform.translation.x
                    - (logo.texture_descriptor.size.width as f32 / 2.) * transform.scale.x;

                let top_edge = transform.translation.y
                    + (logo.texture_descriptor.size.height as f32 / 2.) * transform.scale.y;

                let bottom_edge = transform.translation.y
                    - (logo.texture_descriptor.size.height as f32 / 2.) * transform.scale.y;

                if right_edge > width / 2. {
                    speed_multiplier.x = -1.;
                } else if left_edge < -width / 2. {
                    speed_multiplier.x = 1.;
                }

                if top_edge > height / 2. {
                    speed_multiplier.y = -1.;
                } else if bottom_edge < -height / 2. {
                    speed_multiplier.y = 1.;
                }
            }

            transform.translation.x += move_speed.0 * speed_multiplier.x * time.delta_seconds();
            transform.translation.y += move_speed.0 * speed_multiplier.y * time.delta_seconds();
        }
    }
}

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use figure_8::FigureEightCamera;

mod figure_8;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, spawn_ball)
        .add_plugins(FigureEightCamera)
        .run();
}

#[derive(Component, Default)]
#[require(Camera2d)]
struct MainCamera;

fn move_camera() {}

fn spawn_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((
            MainCamera,
            Visibility::Hidden, // Since this is hidden you will see the circle toggle this line to be commented out and you should see the children
            Projection::Orthographic(OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical {
                    viewport_height: 100.0,
                },
                scale: 1.0,
                ..OrthographicProjection::default_2d()
            }),
        ))
        .with_children(|child| {
            child.spawn((
                Mesh2d(meshes.add(Rectangle::new(10., 10.))),
                MeshMaterial2d(
                    materials.add(ColorMaterial::from_color(Color::srgb_u8(0, 170, 170))),
                ),
                Transform::from_translation(Vec3::new(0., 0., -1.)),
                Visibility::Visible, // Toggle this line as commented in and it should always
                                     // display a square as the child of the camera
            ));
        });
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(10.))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb_u8(255, 255, 255)))),
    ));
}

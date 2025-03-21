use bevy::prelude::*;

use crate::MainCamera;

const FIGURE_EIGHT_RADIUS: f32 = 50.0; // Adjust for size

pub struct FigureEightCamera;

impl Plugin for FigureEightCamera {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_figure_eight_animation);
    }
}
fn main() {}

fn camera_figure_eight_animation(
    time: Res<Time>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    let t = time.elapsed_secs();

    // Equation is a parametric equation from an LLM
    // to make a figure eight loop with the camera
    let x = FIGURE_EIGHT_RADIUS * (1.0 - t.sin().powf(2.)) * (t).cos();
    let y = FIGURE_EIGHT_RADIUS * t.sin() * t.cos();

    for mut transform in camera_query.iter_mut() {
        transform.translation = Vec3::new(x, y, 0.0);
    }
}

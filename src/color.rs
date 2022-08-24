use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use bevy_turborand::*;

use crate::GameState;

pub struct ColorCirclePlugin;

impl Plugin for ColorCirclePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RngPlugin::default());
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(add_circles));

        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(drag_circles));
    }
}

#[derive(Component)]
struct ColorCircle;

fn add_circles(
    mut commands: Commands,
    mut global_rng: ResMut<GlobalRng>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let radius = 40.;
    let num = 20;
    let circle = meshes.add(shape::Circle::new(radius).into());

    let draw_area = (600.0, 400.0);

    let places = generate_circle_places(num, radius, draw_area, &mut global_rng);

    for (i, place) in places.iter().enumerate() {
        let color = random_color(&mut global_rng);
        commands
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: circle.clone().into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::from_translation(
                    (*place - Vec2::from(draw_area) / 2.).extend(0.),
                ),
                ..default()
            })
            .insert(ColorCircle)
            .insert(Name::from(format!("Color circle {i}")));
    }
}

fn random_color(rng: &mut GlobalRng) -> Color {
    let hue = rng.usize(0..3600) as f32 / 10.;
    let saturation = rng.usize(100..900) as f32 / 1000.;
    let lightness = rng.usize(500..800) as f32 / 1000.;
    let alpha = 1.0;

    Color::hsla(hue, saturation, lightness, alpha)
}

fn drag_circles() {}

fn generate_circle_places(
    number: usize,
    radius: f32,
    (width, height): (f32, f32),
    rng: &mut GlobalRng,
) -> Vec<Vec2> {
    let mut res: Vec<Vec2> = Vec::with_capacity(number);

    let mut new_point = || {
        Vec2::new(
            rng.usize(0..width as usize) as f32,
            rng.usize(0..height as usize) as f32,
        )
    };
    let mut fails = 0;

    while res.len() < number {
        let point = new_point();
        if res.iter().all(|x| x.distance(point) > 2. * radius) {
            res.push(point);
            fails = 0;
        } else {
            fails += 1;
            if fails >= 1000 {
                error!("could not place circles");
                break;
            }
        }
    }

    res
}

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::PickableBundle;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use bevy_turborand::*;

use crate::{draggable::Draggable, GameState};

pub struct ColorCirclePlugin;

impl Plugin for ColorCirclePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RngPlugin::default());
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(add_circles));
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(circles_intersect));
    }
}

#[derive(Component)]
struct ColorCircle {
    color: Color,
}

#[derive(Component, Clone, Copy)]
struct CircleGroup {
    combined_color: Color,
    individual_colors: [Color; 2],
}

impl CircleGroup {
    fn from_colors(one: Color, two: Color) -> Self {
        fn dark(color: Color) -> Color {
            match color.as_hsla() {
                Color::Hsla {
                    hue,
                    saturation,
                    lightness,
                    alpha,
                } => Color::Hsla {
                    hue,
                    saturation,
                    lightness: lightness / 2.,
                    alpha,
                },
                _ => unreachable!(),
            }
        }

        let combined_color = dark(one) + dark(two);

        CircleGroup {
            combined_color,
            individual_colors: [one, two],
        }
    }
}

const RADIUS: f32 = 40.;
const NUM: usize = 20;

fn add_circles(
    mut commands: Commands,
    mut global_rng: ResMut<GlobalRng>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let circle = meshes.add(shape::Circle::new(RADIUS).into());

    let draw_area = (600.0, 400.0);

    let places = generate_circle_places(NUM, RADIUS, draw_area, &mut global_rng);

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
            .insert_bundle(PickableBundle { ..default() })
            .insert(Draggable)
            .insert(ColorCircle { color })
            .insert(Name::from(format!("Color circle {i}")));
    }
}

// Inspired by https://stackoverflow.com/a/43195379/1254484
fn random_color(rng: &mut GlobalRng) -> Color {
    let hue = rng.usize(0..3600) as f32 / 10.;
    let saturation = rng.usize(200..900) as f32 / 1000.;
    let lightness = rng.usize(600..900) as f32 / 1000.;
    let alpha = 0.7;

    Color::hsla(hue, saturation, lightness, alpha)
}

fn circles_intersect(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut circles: Query<(&Transform, Entity, &Name, &ColorCircle), (Without<Parent>,)>,
) {
    for [one, two] in circles.iter_combinations_mut() {
        let one_pos = one.0.translation.truncate();
        let two_pos = two.0.translation.truncate();

        if one_pos.distance(two_pos) < 2. * RADIUS {
            info!("{} and {} overlap", one.2, two.2);

            let group = CircleGroup::from_colors(one.3.color, two.3.color);
            commands
                .spawn_bundle(SpatialBundle::default())
                .insert(group)
                .insert_bundle(PickableBundle { ..default() })
                .insert(Draggable)
                .insert(Name::from(format!(
                    "Color group with circles {:?} and {:?}",
                    one.1, two.1
                )))
                .add_child(one.1)
                .add_child(two.1);

            let color = materials.add(ColorMaterial::from(group.combined_color));
            commands
                .entity(one.1)
                .insert(color.clone())
                .remove::<Draggable>();
            commands.entity(two.1).insert(color).remove::<Draggable>();
        }
    }
}

// Inspired by https://stackoverflow.com/a/36177801/1254484
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

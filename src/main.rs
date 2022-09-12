use bevy::diagnostic::Diagnostics;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::{prelude::*, window};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_rapier2d::prelude::*;
use libm::*;
use rand::Rng;
use std::f32::consts::PI;
use std::time::Duration;
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            present_mode: window::PresentMode::AutoNoVsync,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(CirclePlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        .run();
}
struct CirclePlugin;
impl Plugin for CirclePlugin {
    fn build(&self, app: &mut App) {
        let mut spawn_timer = Timer::from_seconds(0.03, true);
        spawn_timer.tick(Duration::from_secs_f32(2.));
        let mut fps_timer = Timer::from_seconds(0.05, true);
        fps_timer.tick(Duration::from_secs_f32(2.));
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(setup)
            .insert_resource(SpawnTimer(spawn_timer))
            .insert_resource(FPSCounterTimer(fps_timer))
            .insert_resource(WorldMousePosition(Vec2 { x: 0., y: 0. }))
            .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
            .insert_resource(CameraSpeed(400.))
            .insert_resource(TargetCameraPosition(Vec2::new(0., 0.)))
            .insert_resource(CameraSmoothness(0.3))
            .insert_resource(ParticleCounter(0))
            .insert_resource(SimulationSettings {
                gravitational_constant: 1000.,
                attraction_color_requirement: 1.,
                repulsion_multiplier: 2.,
                attraction_multiplier: 30.,
            })
            .add_system(add_circle)
            .add_system(definitely_my_cursor_system_which_isnt_stolen)
            .add_system(camera_movement)
            .add_system(text_update)
            .add_system(added_gravity);
    }
}
#[derive(Component)]
struct Name(String);
#[derive(Component)]
struct MainCamera;
#[derive(Component)]
struct Ball {
    density: f32,
    mass: f32,
    radius: f32,
    id: u32,
    r: f32,
    g: f32,
    b: f32,
}
struct SpawnTimer(Timer);
struct FPSCounterTimer(Timer);
struct WorldMousePosition(Vec2);
struct CameraSpeed(f32);
struct CameraSmoothness(f32);
struct TargetCameraPosition(Vec2);
struct ParticleCounter(u32);
struct SimulationSettings {
    gravitational_constant: f32,
    attraction_multiplier: f32,
    repulsion_multiplier: f32,
    attraction_color_requirement: f32,
}
#[derive(Component)]
struct TextId(String);
fn add_circle(
    commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    input: Res<Input<MouseButton>>,
    mouse_position: Res<WorldMousePosition>,
    mut particle_counter: ResMut<ParticleCounter>,
) {
    if (timer.0.tick(time.delta()).finished() && input.pressed(MouseButton::Right))
        || input.just_pressed(MouseButton::Left)
    {
        spawn_circle(
            mouse_position.0,
            commands,
            meshes,
            materials,
            particle_counter.0,
        );
        particle_counter.0 += 1;
    }
}
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    //mut rapier_configuration: ResMut<RapierConfiguration>,
) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(MainCamera);
    commands
        .spawn_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "FPS: ",
                    TextStyle {
                        font: asset_server.load("RobotoMono-Medium.ttf"),
                        font_size: 20.,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("RobotoMono-Medium.ttf"),
                    font_size: 20.,
                    color: Color::WHITE,
                }),
            ])
            .with_text_alignment(TextAlignment::TOP_LEFT)
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(10.),
                    top: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .insert(TextId(String::from("fps")));
    commands
        .spawn_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "Particles: ",
                    TextStyle {
                        font: asset_server.load("RobotoMono-Medium.ttf"),
                        font_size: 20.,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("RobotoMono-Medium.ttf"),
                    font_size: 20.,
                    color: Color::WHITE,
                }),
            ])
            .with_text_alignment(TextAlignment::TOP_LEFT)
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(10.),
                    top: Val::Px(30.),
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .insert(TextId(String::from("particles")));
}
fn spawn_circle(
    position: Vec2,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    particle_count: u32,
) {
    let r = rand::thread_rng().gen::<f32>() + 0.3;
    let g = rand::thread_rng().gen::<f32>() + 0.3;
    let b = rand::thread_rng().gen::<f32>() + 0.3;
    let radius = rand::thread_rng().gen::<f32>() * 10. + 1.0;
    let mass = PI * radius * radius;
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(bevy::prelude::shape::Circle::new(radius).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::rgb(r, g, b))),
            transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.)),
            ..Default::default()
        })
        .insert(Ball {
            density: 1.,
            mass,
            radius: radius,
            id: particle_count,
            r,
            g,
            b,
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(radius))
        .insert(GravityScale(0.))
        .insert(Damping {
            linear_damping: 1.,
            angular_damping: 1.,
        })
        .insert(AdditionalMassProperties::Mass(mass))
        .insert(ExternalImpulse {
            impulse: Vec2::new(0., 0.),
            torque_impulse: 0.,
        });
}
fn definitely_my_cursor_system_which_isnt_stolen(
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut mouse_position: ResMut<WorldMousePosition>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = wnds.primary();

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();
        mouse_position.0 = world_pos;
    }
}
fn camera_movement(
    mut query: Query<&mut Transform, With<MainCamera>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    speed: Res<CameraSpeed>,
    smoothness: Res<CameraSmoothness>,
    mut target_position: ResMut<TargetCameraPosition>,
) {
    if input.pressed(KeyCode::W) {
        target_position.0.y += speed.0 * time.delta().as_secs_f32();
    }
    if input.pressed(KeyCode::A) {
        target_position.0.x -= speed.0 * time.delta().as_secs_f32();
    }
    if input.pressed(KeyCode::S) {
        target_position.0.y -= speed.0 * time.delta().as_secs_f32();
    }
    if input.pressed(KeyCode::D) {
        target_position.0.x += speed.0 * time.delta().as_secs_f32();
    }
    for mut transform in query.iter_mut() {
        transform.translation.x -= (transform.translation.x - target_position.0.x) / smoothness.0
            * time.delta().as_secs_f32();
        transform.translation.y -= (transform.translation.y - target_position.0.y) / smoothness.0
            * time.delta().as_secs_f32();
    }
}
fn text_update(
    diagnostics: Res<Diagnostics>,
    mut query: Query<(&mut Text, &TextId)>,
    mut timer: ResMut<FPSCounterTimer>,
    time: Res<Time>,
    particles: Res<ParticleCounter>,
) {
    if timer.0.tick(time.delta()).finished() {
        for mut text in query.iter_mut() {
            if text.1 .0 == String::from("fps") {
                if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                    if let Some(average) = fps.average() {
                        text.0.sections[1].value = format!("{average:.2}");
                    }
                }
            } else if text.1 .0 == String::from("particles") {
                text.0.sections[1].value = format!("{}", particles.0);
            }
        }
    }
}
fn added_gravity(
    query1: Query<(&Ball, &Transform)>,
    mut query2: Query<(&Ball, &Transform, &mut ExternalImpulse)>,
    simulation_settings: Res<SimulationSettings>,
    time: Res<Time>,
) {
    for ball1 in query1.iter() {
        for mut ball2 in query2.iter_mut() {
            if ball1.1 != ball2.1 {
                let mut angle: f32 = atan2f(
                    ball2.1.translation.x - ball1.1.translation.x,
                    ball2.1.translation.y - ball1.1.translation.y,
                ) * 180.
                    / PI;
                if angle < 0. {
                    angle += 360.;
                }
                angle = angle.to_radians();
                let mut attraction_force: Vec2 = Vec2::new(0., 0.);
                //gravity
                let distance: f32 = sqrtf(
                    powf(ball2.1.translation.x - ball1.1.translation.x, 2.)
                        + powf(ball2.1.translation.y - ball1.1.translation.y, 2.),
                );
                attraction_force.x =
                    -simulation_settings.gravitational_constant * ball1.0.mass * ball2.0.mass
                        / powf(distance, 2.)
                        * sinf(angle);
                attraction_force.y =
                    -simulation_settings.gravitational_constant * ball1.0.mass * ball2.0.mass
                        / powf(distance, 2.)
                        * cosf(angle);
                let repulsion_force: Vec2 = attraction_force / -powf(distance, 2.);
                let mut attraction_color_multiplier: f32 =
                    simulation_settings.attraction_color_requirement;
                attraction_color_multiplier -= fabsf(ball1.0.r - ball2.0.r)
                    + fabsf(ball1.0.g - ball2.0.g)
                    + fabsf(ball1.0.b - ball2.0.b);
                ball2.2.impulse = attraction_force
                    * simulation_settings.attraction_multiplier
                    * time.delta_seconds();
                //* attraction_color_multiplier
                //*
                //+ repulsion_force
                //    * time.delta_seconds()
                //    * simulation_settings.repulsion_multiplier;
            }
        }
    }
}

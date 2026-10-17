use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const LIMIT: usize = 250;

#[derive(Component)]
struct Body;
#[derive(Component)]
struct OrbitCamera;
#[derive(Component)]
struct Status;
#[derive(Component, Clone, Copy)]
enum Action { Cube, Ball, Pause, Reset }
#[derive(Resource, Default)]
struct Simulation { paused: bool, serial: u32 }
#[derive(Resource)]
struct Shapes {
    cube: Handle<Mesh>,
    ball: Handle<Mesh>,
    colors: Vec<Handle<StandardMaterial>>,
}
#[derive(Resource)]
struct Orbit { angle: f32, height: f32, radius: f32 }

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.035, 0.05, 0.09)))
        .init_resource::<Simulation>()
        .insert_resource(Orbit { angle: 0.7, height: 13.0, radius: 24.0 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust Physik-Labor | 3D".into(),
                resolution: (1280, 800).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (controls, camera, cleanup, status).chain())
        .run();
}

fn spawn_body(commands: &mut Commands, shapes: &Shapes, ball: bool, pos: Vec3, index: u32) {
    commands.spawn((
        Body,
        Mesh3d(if ball { shapes.ball.clone() } else { shapes.cube.clone() }),
        MeshMaterial3d(shapes.colors[index as usize % shapes.colors.len()].clone()),
        Transform::from_translation(pos).with_rotation(Quat::from_euler(
            EulerRot::XYZ, 0.12 * index as f32, 0.17 * index as f32, 0.05,
        )),
        RigidBody::Dynamic,
        if ball { Collider::ball(0.55) } else { Collider::cuboid(0.5, 0.5, 0.5) },
        Restitution::coefficient(if ball { 0.65 } else { 0.2 }),
        Friction::coefficient(0.7),
        Damping { linear_damping: 0.08, angular_damping: 0.15 },
        Ccd::enabled(),
    ));
}

fn initial_scene(commands: &mut Commands, shapes: &Shapes) {
    for y in 0..6 {
        for x in 0..5 {
            let n = (y * 5 + x) as u32;
            spawn_body(commands, shapes, false,
                Vec3::new(x as f32 * 1.15 - 2.3, 0.7 + y as f32 * 1.15, 0.0), n);
        }
    }
    for n in 0..6 {
        spawn_body(commands, shapes, true,
            Vec3::new(n as f32 - 2.5, 10.0 + n as f32 * 1.3, 0.4), n);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let shapes = Shapes {
        cube: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        ball: meshes.add(Sphere::new(0.55)),
        colors: [(0.1, 0.75, 0.9), (1.0, 0.48, 0.15), (0.65, 0.4, 0.95), (0.25, 0.85, 0.55)]
            .into_iter().map(|(r,g,b)| materials.add(StandardMaterial {
                base_color: Color::srgb(r,g,b), perceptual_roughness: 0.38, ..default()
            })).collect(),
    };
    let ground = materials.add(Color::srgb(0.15, 0.2, 0.28));
    // Solid floor and four low retaining walls; mesh and collider sizes match.
    for (position, size) in [
        (Vec3::new(0.0,-0.3,0.0), Vec3::new(22.0,0.6,22.0)),
        (Vec3::new(-11.0,0.6,0.0), Vec3::new(0.4,1.8,22.0)),
        (Vec3::new(11.0,0.6,0.0), Vec3::new(0.4,1.8,22.0)),
        (Vec3::new(0.0,0.6,-11.0), Vec3::new(22.0,1.8,0.4)),
        (Vec3::new(0.0,0.6,11.0), Vec3::new(22.0,1.8,0.4)),
    ] {
        commands.spawn((Mesh3d(meshes.add(Cuboid::new(size.x,size.y,size.z))),
            MeshMaterial3d(ground.clone()), Transform::from_translation(position),
            RigidBody::Fixed, Collider::cuboid(size.x/2.0,size.y/2.0,size.z/2.0),
            Friction::coefficient(0.8)));
    }
    commands.spawn((DirectionalLight { illuminance: 10000.0, shadows_enabled: true, ..default() },
        Transform::from_xyz(8.0,16.0,10.0).looking_at(Vec3::ZERO, Vec3::Y)));
    commands.spawn((PointLight { intensity: 1_500_000.0, range: 45.0, ..default() },
        Transform::from_xyz(-8.0,10.0,-5.0)));
    commands.spawn((Camera3d::default(), OrbitCamera,
        Transform::from_xyz(18.0,13.0,18.0).looking_at(Vec3::new(0.0,2.0,0.0), Vec3::Y)));
    initial_scene(&mut commands, &shapes);
    commands.insert_resource(shapes);

    commands.spawn((Node {
        position_type: PositionType::Absolute, left: px(20), top: px(20),
        padding: UiRect::all(px(18)), flex_direction: FlexDirection::Column,
        row_gap: px(12), ..default()
    }, BackgroundColor(Color::srgba(0.03,0.05,0.09,0.94))))
    .with_children(|panel| {
        panel.spawn((Text::new("PHYSIK / LAB"), TextFont { font_size: 28.0, ..default() },
            TextColor(Color::srgb(0.25,0.85,1.0))));
        panel.spawn((Text::new(""), TextFont { font_size: 18.0, ..default() }, Status));
        for (label, action) in [
            ("Wuerfel hinzufuegen [1]", Action::Cube),
            ("Kugel hinzufuegen [2]", Action::Ball),
            ("Pause / Weiter [Leertaste]", Action::Pause),
            ("Neu starten [R]", Action::Reset),
        ] {
            panel.spawn((Button, action, Node {
                width: px(275), height: px(48), justify_content: JustifyContent::Center,
                align_items: AlignItems::Center, ..default()
            }, BackgroundColor(Color::srgb(0.12,0.23,0.34))))
            .with_children(|button| {
                button.spawn((Text::new(label), TextFont { font_size: 18.0, ..default() }));
            });
        }
        panel.spawn((Text::new("Kamera: Pfeiltasten\nZoom: + / -\nMaximal 250 Objekte"),
            TextFont { font_size: 16.0, ..default() }));
    });
}

fn controls(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut buttons: Query<(&Interaction, &Action, &mut BackgroundColor), Changed<Interaction>>,
    bodies: Query<Entity, With<Body>>,
    shapes: Res<Shapes>,
    mut sim: ResMut<Simulation>,
    mut configs: Query<&mut RapierConfiguration>,
) {
    let mut actions = Vec::new();
    for (interaction, action, mut color) in &mut buttons {
        *color = BackgroundColor(match interaction {
            Interaction::Pressed => Color::srgb(0.05,0.65,0.8),
            Interaction::Hovered => Color::srgb(0.18,0.36,0.5),
            Interaction::None => Color::srgb(0.12,0.23,0.34),
        });
        if *interaction == Interaction::Pressed { actions.push(*action); }
    }
    for (key, action) in [(KeyCode::Digit1,Action::Cube), (KeyCode::Digit2,Action::Ball),
        (KeyCode::Space,Action::Pause), (KeyCode::KeyR,Action::Reset)] {
        if keys.just_pressed(key) { actions.push(action); }
    }
    // Reset takes precedence over other inputs in the same frame.
    if actions.iter().any(|a| matches!(a, Action::Reset)) {
        for entity in &bodies { commands.entity(entity).despawn(); }
        initial_scene(&mut commands, &shapes);
        sim.paused = false;
        sim.serial = 0;
    } else {
        let mut count = bodies.iter().count();
        for action in actions {
            match action {
                Action::Pause => sim.paused = !sim.paused,
                Action::Cube | Action::Ball if count < LIMIT => {
                    let n = sim.serial;
                    // Distinct slots for consecutive spawns avoid initial overlap.
                    let position = Vec3::new((n % 7) as f32 * 1.5 - 4.5,
                        10.0 + (n / 49) as f32 * 1.5, ((n / 7) % 7) as f32 * 1.5 - 4.5);
                    spawn_body(&mut commands, &shapes, matches!(action,Action::Ball), position, n);
                    sim.serial = sim.serial.wrapping_add(1);
                    count += 1;
                }
                _ => {}
            }
        }
    }
    for mut config in &mut configs { config.physics_pipeline_active = !sim.paused; }
}

fn camera(keys: Res<ButtonInput<KeyCode>>, time: Res<Time>, mut orbit: ResMut<Orbit>,
    mut cameras: Query<&mut Transform, With<OrbitCamera>>) {
    let dt = time.delta_secs();
    if keys.pressed(KeyCode::ArrowLeft) { orbit.angle -= dt; }
    if keys.pressed(KeyCode::ArrowRight) { orbit.angle += dt; }
    if keys.pressed(KeyCode::ArrowUp) { orbit.height += dt * 10.0; }
    if keys.pressed(KeyCode::ArrowDown) { orbit.height -= dt * 10.0; }
    if keys.pressed(KeyCode::Equal) || keys.pressed(KeyCode::NumpadAdd) { orbit.radius -= dt * 12.0; }
    if keys.pressed(KeyCode::Minus) || keys.pressed(KeyCode::NumpadSubtract) { orbit.radius += dt * 12.0; }
    orbit.radius = orbit.radius.clamp(8.0, 55.0);
    orbit.height = orbit.height.clamp(3.0, 40.0);
    for mut transform in &mut cameras {
        *transform = Transform::from_xyz(orbit.angle.sin()*orbit.radius, orbit.height,
            orbit.angle.cos()*orbit.radius).looking_at(Vec3::new(0.0,2.0,0.0),Vec3::Y);
    }
}

fn cleanup(mut commands: Commands, bodies: Query<(Entity, &Transform), With<Body>>) {
    for (entity, transform) in &bodies {
        if transform.translation.y < -30.0 { commands.entity(entity).despawn(); }
    }
}

fn status(sim: Res<Simulation>, bodies: Query<Entity,With<Body>>, mut labels: Query<&mut Text,With<Status>>) {
    for mut text in &mut labels {
        **text = format!("{} | {} Objekte", if sim.paused { "PAUSIERT" } else { "SIMULATION LAEUFT" }, bodies.iter().count());
    }
}

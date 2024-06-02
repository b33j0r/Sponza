//! Loads and renders a glTF file as a scene.

use bevy::{
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
};
use std::f32::consts::*;

fn main() {
    App::new()
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins(DefaultPlugins)
        // .add_systems(Startup, setup)
        .add_systems(Startup, spawn_gltf)
        .add_systems(Update, keyboard_movement)
        // .add_systems(Update, animate_light_direction)
        .run();
}

fn spawn_gltf(
    mut commands: Commands,
    ass: Res<AssetServer>,
) {
    // note that we have to include the `Scene0` label
    let my_gltf = ass.load("sponza/Sponza.glb#Scene0");

    // to position our 3d model, simply use the Transform
    // in the SceneBundle
    commands.spawn(SceneBundle {
        scene: my_gltf,
        // transform: Transform::from_xyz(2.0, 0.0, -5.0),
        ..Default::default()
    });
}

fn keyboard_movement(
    mut query: Query<&mut Transform, With<Camera3d>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let speed = 10.0;
    for mut transform in query.iter_mut() {
        let forward = -transform.local_z().clone();
        let right = transform.local_x().clone();
        if keys.pressed(KeyCode::KeyW) {
            transform.translation += forward * speed * time.delta_seconds();
        }
        if keys.pressed(KeyCode::KeyS) {
            transform.translation += -forward * speed * time.delta_seconds();
        }
        if keys.pressed(KeyCode::KeyA) {
            transform.translation += -right * speed * time.delta_seconds();
        }
        if keys.pressed(KeyCode::KeyD) {
            transform.translation += right * speed * time.delta_seconds();
        }
    }
}



// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.spawn((
//         Camera3dBundle {
//             transform: Transform::from_xyz(0.7, 0.7, 1.0)
//                 .looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
//             ..default()
//         },
//         // EnvironmentMapLight {
//         //     diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
//         //     specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
//         //     intensity: 250.0,
//         // },
//     ));
//
//     commands.spawn(DirectionalLightBundle {
//         directional_light: DirectionalLight {
//             shadows_enabled: true,
//             ..default()
//         },
//         // This is a relatively small scene, so use tighter shadow
//         // cascade bounds than the default for better quality.
//         // We also adjusted the shadow map to be larger since we're
//         // only using a single cascade.
//         cascade_shadow_config: CascadeShadowConfigBuilder {
//             num_cascades: 1,
//             maximum_distance: 1.6,
//             ..default()
//         }
//         .into(),
//         ..default()
//     });
//     commands.spawn(SceneBundle {
//         scene: asset_server
//             .load(GltfAssetLabel::Scene(0).from_asset("sponza/Sponza.glb")),
//         ..default()
//     });
// }
//
// fn animate_light_direction(
//     time: Res<Time>,
//     mut query: Query<&mut Transform, With<DirectionalLight>>,
// ) {
//     for mut transform in &mut query {
//         transform.rotation = Quat::from_euler(
//             EulerRot::ZYX,
//             0.0,
//             time.elapsed_seconds() * PI / 5.0,
//             -FRAC_PI_4,
//         );
//     }
// }

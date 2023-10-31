use bevy::prelude::*;
use bevy_wry_webview::{UiWebViewBundle, WebViewLocation, WebViewMarker, WebViewPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WebViewPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, moving_webview)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(UiWebViewBundle {
        node_bundle: NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Px(400.0),
                height: Val::Px(400.0),
                ..Default::default()
            },
            ..Default::default()
        },
        location: WebViewLocation("https://tauri.app/".to_owned()),
        ..Default::default()
    });

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn moving_webview(time: Res<Time>, mut query: Query<&mut Style, With<WebViewMarker>>) {
    let mut style = query.single_mut();

    let top = Val::Px(((time.elapsed_seconds().sin() / 2.0) + 0.5) * 300.0);
    let left = Val::Px(((time.elapsed_seconds().cos() / 2.0) + 0.5) * 300.0);

    *style = Style {
        top,
        left,
        ..style.clone()
    }
}

use bevy::prelude::*;
use bevy_wry_webview::{
    UiWebViewBundle, WebViewDespawning, WebViewLocation, WebViewMarker, WebViewPlugin,
};

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
        location: WebViewLocation::Html(
            r#"
<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
    </head>
    <body>
        <div id="ele">
            <div>Positioned element on transparent background</div>
        </div>
    </body>
    <style>
html, body {
    width: 100vw;
    height: 100vh;
    background-color: rgba(255, 192, 203, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
}

#ele {
    background-color: blue;
    color: white;
    width: 50%;
    height: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
}
    </style>
</html>
                      "#
            .to_owned(),
        ),
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
    let _ = query.get_single_mut().map(|mut style| {
        let top = Val::Px(((time.elapsed_seconds().sin() / 2.0) + 0.5) * 500.0);
        let left = Val::Px(((time.elapsed_seconds().cos() / 2.0) + 0.5) * 500.0);

        *style = Style {
            top,
            left,
            ..style.clone()
        };
    });
}

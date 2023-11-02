use reactivity::WebViewReactivityPlugin;
use wry::{
    raw_window_handle::{ActiveHandle, WindowHandle},
    WebView, WebViewBuilder,
};

use bevy::{
    prelude::*,
    window::{RawHandleWrapper, WindowResized},
};

pub mod ipc;
mod reactivity;

pub struct WebViewPlugin;

#[derive(Component, Debug)]
pub enum WebViewLocation {
    Url(String),
    Html(String),
}

#[derive(Component)]
pub struct WebViewMarker;

#[derive(DerefMut, Deref)]
pub struct WebViewRegistry {
    webviews: Vec<WebView>,
}

#[derive(Component, Deref, DerefMut)]
pub struct WebViewHandle(Option<usize>);

#[derive(Bundle)]
pub struct UiWebViewBundle {
    pub node_bundle: NodeBundle,
    pub location: WebViewLocation,
    pub handle: WebViewHandle,
    pub marker: WebViewMarker,
    // TODO Add IPC handler
}

impl Default for UiWebViewBundle {
    fn default() -> Self {
        UiWebViewBundle {
            node_bundle: default(),
            location: WebViewLocation::Html("".to_owned()),
            handle: WebViewHandle(None),
            marker: WebViewMarker,
        }
    }
}

/**
 * A simple trait to emulate a custom command for despawning `UiWebViewBundle`s
 */
pub trait WebViewDespawning {
    /**
     * Despawns `UiWebViewBundle`s and cleans up the associated `wry` `WebView`
     */
    fn despawn_webview(&mut self, entity: Entity);
}

impl WebViewDespawning for Commands<'_, '_> {
    fn despawn_webview(&mut self, entity: Entity) {
        self.add(move |world: &mut World| {
            let registry = world
                .get_non_send_resource::<WebViewRegistry>()
                .unwrap_or_else(|| {
                    panic!("WebView Registry not found; have you loaded `WebViewPlugin`")
                });
            let handle = world.entity(entity).get::<WebViewHandle>().unwrap();
            // TODO close it here -- Waiting on Tauri/Wry folks
            println!("Despawning here");
            world.despawn(entity);
        })
    }
}

impl Plugin for WebViewPlugin {
    fn build(&self, app: &mut App) {
        app.insert_non_send_resource(WebViewRegistry { webviews: vec![] })
            .add_plugins(WebViewReactivityPlugin)
            .add_systems(Update, Self::on_webview_spawn);
    }
}

impl WebViewPlugin {
    fn on_webview_spawn(
        mut registry: NonSendMut<WebViewRegistry>,
        window_handle: Query<&RawHandleWrapper>,
        mut query: Query<
            (
                &mut WebViewHandle,
                &WebViewLocation,
                &Node,
                &GlobalTransform,
                //&ComputedVisibility,
            ),
            With<WebViewMarker>,
        >,
    ) {
        if let Ok(window_handle) = window_handle.get_single().map(|x| x.window_handle) {
            for (mut handle, location, size, position) in
                query.iter_mut().filter(|(x, _, _, _)| x.is_none())
            // && v.is_visible())
            {
                let size = size.size();
                let final_position = (
                    (position.translation().x - size.x / 2.0) as i32,
                    (position.translation().y - size.y / 2.0) as i32,
                );

                let borrowed_handle =
                    unsafe { &WindowHandle::borrow_raw(window_handle, ActiveHandle::new()) };
                let webview = WebViewBuilder::new_as_child(&borrowed_handle)
                    .with_position(final_position)
                    .with_transparent(true)
                    .with_size((size.x as u32, size.y as u32));

                let webview = match location {
                    WebViewLocation::Url(url) => webview.with_url(url),
                    WebViewLocation::Html(html) => webview.with_html(html),
                }
                .unwrap()
                .build()
                .unwrap();

                *handle = WebViewHandle(Some(registry.len()));
                registry.push(webview);
            }
        }
    }
}

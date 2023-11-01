use wry::{
    raw_window_handle::{ActiveHandle, WindowHandle},
    WebView, WebViewBuilder,
};

use bevy::{
    prelude::*,
    window::{PrimaryWindow, RawHandleWrapper, WindowResized},
};

pub struct WebViewPlugin;

#[derive(Component, Debug)]
pub enum WebViewLocation {
    Url(String),
    Html(String),
}

#[derive(Component)]
pub struct WebViewMarker;

#[derive(DerefMut, Deref)]
struct WebViewRegistry {
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

impl Plugin for WebViewPlugin {
    fn build(&self, app: &mut App) {
        app.insert_non_send_resource(WebViewRegistry { webviews: vec![] })
            .add_systems(
                Update,
                (
                    Self::on_webview_spawn,
                    Self::on_webview_resize,
                    Self::on_webview_reposition,
                    Self::on_webview_redirect,
                    Self::on_window_resize,
                ),
            );
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
        let _ = window_handle
            .get_single()
            .map(|x| x.window_handle)
            .map(|window_handle| {
                for (mut handle, location, size, position) in
                    query.iter_mut().filter(|(x, _, _, _)| x.is_none())
                // && v.is_visible())
                {
                    let size = size.size();
                    let final_position = (
                        (position.translation().x - size.x / 2.0) as i32,
                        (position.translation().y - size.y / 2.0) as i32,
                        //((window_resolution.height() - position.translation().y) - size.y / 2.0)
                        //    as i32,
                    );

                    //println!("{:?}", final_position == (0, 320));
                    //(0, 320); // Uncommenting this line fixes the issue apparently. Why is
                    // completely beyond me.
                    //let final_position = (0, 320);
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
            });
    }

    fn _on_webview_despawn() {
        todo!("Despawn wry webview attached to bundle")
    }

    fn on_webview_resize(
        registry: NonSendMut<WebViewRegistry>,
        query: Query<(&WebViewHandle, &Node), (With<WebViewMarker>, Changed<Node>)>,
    ) {
        for (handle, size) in query.iter() {
            handle.map(|x| {
                registry
                    .get(x)
                    .map(|webview| webview.set_size((size.size().x as u32, size.size().y as u32)))
            });
        }
    }

    fn on_webview_reposition(
        registry: NonSendMut<WebViewRegistry>,
        query: Query<
            (&WebViewHandle, &GlobalTransform, &Node),
            (With<WebViewMarker>, Changed<GlobalTransform>),
        >,
    ) {
        for (handle, position, size) in query.iter() {
            let size = size.size();
            handle.map(|x| {
                registry.get(x).map(|webview| {
                    let final_position = (
                        (position.translation().x - size.x / 2.0) as i32,
                        (position.translation().y - size.y / 2.0) as i32,
                        //((window_resolution.height() - position.translation().y) - size.y / 2.0) as i32,
                    );
                    webview.set_position(final_position)
                })
            });
        }
    }

    fn on_webview_redirect(
        registry: NonSendMut<WebViewRegistry>,
        query: Query<
            (&WebViewHandle, &WebViewLocation),
            (With<WebViewMarker>, Changed<WebViewLocation>),
        >,
    ) {
        for (handle, location) in query.iter() {
            handle.map(|x| {
                registry.get(x).map(|webview| match location {
                    WebViewLocation::Url(url) => webview.load_url(url),
                    WebViewLocation::Html(_html) => {
                        // TODO Implement HTML loading past builder
                    }
                })
            });
        }
    }

    fn on_window_resize(
        e: EventReader<WindowResized>,
        registry: NonSendMut<WebViewRegistry>,
        query: Query<(&WebViewHandle, &Node, &GlobalTransform), With<WebViewHandle>>,
    ) {
        if !e.is_empty() {
            for (handle, size, position) in &query {
                let size = size.size();
                let final_position = (
                    (position.translation().x - size.x / 2.0) as i32,
                    (position.translation().y - size.y / 2.0) as i32,
                );
                handle
                    .map(|x| registry.get(x))
                    .flatten()
                    .map(|webview| webview.set_position(final_position));
            }
        }
    }
}

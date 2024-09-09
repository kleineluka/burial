// imports
use tauri::{Window, WindowBuilder};

// create a popup window (set width and height to 0 to use the current window's size)
pub fn create_popup(window: Window, app: &tauri::AppHandle, identifier: &str, width: f64, height: f64, title: &str, url: &str) {
    // determine window size based on given width and height
    let window_width = if width == 0.0 { window.inner_size().unwrap().width as f64 } else { width };
    let window_height = if height == 0.0 { window.inner_size().unwrap().height as f64 } else { height };
    // create path to the popup window (take url and make it a pathbuf)
    let window = WindowBuilder::new(
        app,
        identifier,
        tauri::WindowUrl::App(url.into())
    )
    .title(title) 
    .inner_size(window_width, window_height) 
    .build()
    .unwrap();
    // show the window
    window.show().unwrap();
}

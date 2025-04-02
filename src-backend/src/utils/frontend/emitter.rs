pub struct EventEmitter<'a> {
    window: Option<&'a tauri::Window>, // Optional reference to a Tauri Window
}

impl<'a> EventEmitter<'a> {
    // Constructor
    pub fn new(window: Option<&'a tauri::Window>) -> Self {
        Self { window }
    }

    // Emit an event if a window is available
    pub fn emit(&self, event: &str, message: &str) {
        if let Some(win) = self.window {
            win.emit(event, message).unwrap_or_else(|e| {
                eprintln!("Failed to emit event '{}': {}", event, e);
            });
        }
    }
}

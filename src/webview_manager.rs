use nightshade::prelude::{egui, window};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender, channel};
use web_host_protocol::{BackendMessage, FrontendMessage};
use wry::dpi::{LogicalPosition, LogicalSize};
use wry::{Rect, WebView, WebViewBuilder};

pub struct WebviewManager {
    webviews: HashMap<String, WebView>,
    webview_bounds: HashMap<String, (f64, f64, f64, f64)>,
    window: Option<Arc<window::Window>>,
    message_sender: Sender<(String, FrontendMessage)>,
    message_receiver: Receiver<(String, FrontendMessage)>,
}

impl Default for WebviewManager {
    fn default() -> Self {
        let (sender, receiver) = channel();
        Self {
            webviews: HashMap::new(),
            webview_bounds: HashMap::new(),
            window: None,
            message_sender: sender,
            message_receiver: receiver,
        }
    }
}

const INIT_SCRIPT: &str = r#"
window.__backendMessageHandler = null;

window.onBackendMessage = function(base64Data) {
    if (window.__backendMessageHandler) {
        window.__backendMessageHandler(base64Data);
    }
};

window.sendToBackend = function(base64Data) {
    window.ipc.postMessage(base64Data);
};
"#;

impl WebviewManager {
    pub fn set_window(&mut self, window: Arc<window::Window>) {
        self.window = Some(window);
    }

    pub fn create_webview(&mut self, id: String, url: &str, rect: egui::Rect, _scale_factor: f32) {
        if self.webviews.contains_key(&id) {
            return;
        }

        let Some(window) = &self.window else {
            return;
        };

        let bounds = Rect {
            position: LogicalPosition::new(rect.min.x as f64, rect.min.y as f64).into(),
            size: LogicalSize::new(rect.width() as f64, rect.height() as f64).into(),
        };

        let sender = self.message_sender.clone();
        let webview_id = id.clone();

        let webview_result = WebViewBuilder::new()
            .with_url(url)
            .with_bounds(bounds)
            .with_navigation_handler(|_| true)
            .with_initialization_script(INIT_SCRIPT)
            .with_ipc_handler(move |request| {
                let message = request.body();
                if let Some(frontend_message) = FrontendMessage::from_base64(message) {
                    let _ = sender.send((webview_id.clone(), frontend_message));
                }
            })
            .build_as_child(window.as_ref());

        if let Ok(webview) = webview_result {
            let _ = webview.set_visible(true);
            self.webview_bounds.insert(
                id.clone(),
                (
                    rect.min.x as f64,
                    rect.min.y as f64,
                    rect.width() as f64,
                    rect.height() as f64,
                ),
            );
            self.webviews.insert(id, webview);
        }
    }

    pub fn update_position(&mut self, id: &str, rect: egui::Rect, _scale_factor: f32) {
        let Some(webview) = self.webviews.get(id) else {
            return;
        };

        let new_bounds = (
            rect.min.x as f64,
            rect.min.y as f64,
            rect.width() as f64,
            rect.height() as f64,
        );

        if let Some(old_bounds) = self.webview_bounds.get(id)
            && *old_bounds == new_bounds
        {
            return;
        }

        let bounds = Rect {
            position: LogicalPosition::new(new_bounds.0, new_bounds.1).into(),
            size: LogicalSize::new(new_bounds.2, new_bounds.3).into(),
        };

        let _ = webview.set_bounds(bounds);
        self.webview_bounds.insert(id.to_string(), new_bounds);
    }

    pub fn ensure_all_visible(&self) {
        for webview in self.webviews.values() {
            let _ = webview.set_visible(true);
        }
    }

    pub fn has_webview(&self, id: &str) -> bool {
        self.webviews.contains_key(id)
    }

    pub fn retain_only(&mut self, active_ids: &std::collections::HashSet<String>) {
        self.webviews.retain(|id, _| active_ids.contains(id));
        self.webview_bounds.retain(|id, _| active_ids.contains(id));
    }

    pub fn send_message(&self, id: &str, message: &BackendMessage) {
        let Some(webview) = self.webviews.get(id) else {
            return;
        };

        let Some(base64_data) = message.to_base64() else {
            return;
        };

        let script = format!("window.onBackendMessage('{}');", base64_data);
        let _ = webview.evaluate_script(&script);
    }

    pub fn drain_messages(&self) -> Vec<(String, FrontendMessage)> {
        let mut messages = Vec::new();
        while let Ok(message) = self.message_receiver.try_recv() {
            messages.push(message);
        }
        messages
    }
}

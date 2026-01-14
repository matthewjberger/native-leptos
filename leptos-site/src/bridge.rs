use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_host_protocol::{BackendMessage, FrontendCommand, FrontendMessage};

thread_local! {
    static BACKEND_MESSAGE_HANDLER: RefCell<Option<Rc<dyn Fn(BackendMessage)>>> = const { RefCell::new(None) };
    static PENDING_MESSAGES: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

#[wasm_bindgen(js_name = "__backendMessageHandler")]
pub fn backend_message_handler(base64_data: &str) {
    if let Some(message) = BackendMessage::from_base64(base64_data) {
        BACKEND_MESSAGE_HANDLER.with(|handler| {
            if let Some(callback) = handler.borrow().as_ref() {
                callback(message);
            }
        });
    }
}

fn is_ipc_available() -> bool {
    let window = web_sys::window().expect("no window");
    js_sys::Reflect::has(&window, &JsValue::from_str("ipc")).unwrap_or(false)
}

fn post_message_direct(base64_data: &str) {
    let window = web_sys::window().expect("no window");
    if let Ok(ipc) = js_sys::Reflect::get(&window, &JsValue::from_str("ipc")) {
        if let Ok(post_fn) = js_sys::Reflect::get(&ipc, &JsValue::from_str("postMessage")) {
            if let Some(post_fn) = post_fn.dyn_ref::<js_sys::Function>() {
                let _ = post_fn.call1(&ipc, &JsValue::from_str(base64_data));
            }
        }
    }
}

fn flush_pending_messages() {
    PENDING_MESSAGES.with(|pending| {
        let messages: Vec<String> = pending.borrow_mut().drain(..).collect();
        for message in messages {
            post_message_direct(&message);
        }
    });
}

fn try_send_or_queue(base64_data: String) {
    if is_ipc_available() {
        flush_pending_messages();
        post_message_direct(&base64_data);
    } else {
        PENDING_MESSAGES.with(|pending| {
            pending.borrow_mut().push(base64_data);
        });
        schedule_retry();
    }
}

fn schedule_retry() {
    let closure = Closure::once(Box::new(|| {
        if is_ipc_available() {
            flush_pending_messages();
        } else {
            schedule_retry();
        }
    }) as Box<dyn FnOnce()>);

    let window = web_sys::window().expect("no window");
    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(),
        10,
    );
    closure.forget();
}

pub fn send_to_backend(message: FrontendMessage) {
    if let Some(base64_data) = message.to_base64() {
        try_send_or_queue(base64_data);
    }
}

pub fn send_command(command: FrontendCommand) {
    send_to_backend(FrontendMessage::Command(command));
}

pub fn set_backend_message_handler<F>(handler: F)
where
    F: Fn(BackendMessage) + 'static,
{
    BACKEND_MESSAGE_HANDLER.with(|h| {
        *h.borrow_mut() = Some(Rc::new(handler));
    });

    register_handler();
}

fn register_handler() {
    let window = web_sys::window().expect("no window");
    let closure = Closure::wrap(Box::new(|base64_data: String| {
        backend_message_handler(&base64_data);
    }) as Box<dyn Fn(String)>);

    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("__backendMessageHandler"),
        closure.as_ref(),
    )
    .expect("failed to set handler");

    closure.forget();
}

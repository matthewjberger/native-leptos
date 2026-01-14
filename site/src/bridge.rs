use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_host_protocol::{BackendEvent, FrontendCommand};

fn get_ipc() -> Option<js_sys::Function> {
    let window = web_sys::window()?;
    let ipc = js_sys::Reflect::get(&window, &JsValue::from_str("ipc")).ok()?;
    if ipc.is_undefined() {
        return None;
    }
    let post_message = js_sys::Reflect::get(&ipc, &JsValue::from_str("postMessage")).ok()?;
    post_message.dyn_ref::<js_sys::Function>().cloned()
}

pub fn send_command(cmd: FrontendCommand) {
    if let Some(data) = cmd.to_base64() {
        if let Some(post_message) = get_ipc() {
            let window = web_sys::window().unwrap();
            let ipc = js_sys::Reflect::get(&window, &JsValue::from_str("ipc")).unwrap();
            let _ = post_message.call1(&ipc, &JsValue::from_str(&data));
        }
    }
}

pub fn set_backend_handler(handler: impl Fn(BackendEvent) + 'static) {
    let window = web_sys::window().unwrap();
    let connected = Rc::new(Cell::new(false));
    let connected_for_handler = connected.clone();

    let closure = Closure::wrap(Box::new(move |data: String| {
        if let Some(event) = BackendEvent::from_base64(&data) {
            if matches!(event, BackendEvent::Connected) {
                connected_for_handler.set(true);
            }
            handler(event);
        }
    }) as Box<dyn Fn(String)>);

    let _ = js_sys::Reflect::set(&window, &JsValue::from_str("__h"), closure.as_ref());
    closure.forget();

    let ipc_ready = Rc::new(Cell::new(false));
    let ipc_ready_for_poll = ipc_ready.clone();

    let poll = Closure::wrap(Box::new(move || {
        if !ipc_ready_for_poll.get() {
            if get_ipc().is_some() {
                ipc_ready_for_poll.set(true);
            } else {
                return;
            }
        }
        if !connected.get() {
            send_command(FrontendCommand::Ready);
        }
    }) as Box<dyn Fn()>);

    let _ = window.set_interval_with_callback_and_timeout_and_arguments_0(
        poll.as_ref().unchecked_ref(),
        50,
    );
    poll.forget();
}

use api_types::resources::Resource;
use leptos::context::Provider;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::{Route, Router, Routes, A};
use leptos_router::path;
use ui::*;
use web_host_protocol::{BackendEvent, FrontendCommand};

pub mod api;
pub mod bridge;

#[derive(Clone)]
struct IpcState {
    connected: ReadSignal<bool>,
    log_entries: ReadSignal<Vec<String>>,
}

#[component]
pub fn App() -> impl IntoView {
    let (connected, set_connected) = signal(false);
    let (log_entries, set_log_entries) = signal(Vec::<String>::new());

    Effect::new(move |_| {
        bridge::set_backend_handler(move |event| match event {
            BackendEvent::Connected => {
                set_connected.set(true);
                set_log_entries.update(|entries| entries.push("Backend connected".to_string()));
            }
            BackendEvent::RandomNumber { request_id, value } => {
                set_log_entries
                    .update(|entries| entries.push(format!("Request #{request_id}: {value}")));
            }
        });
    });

    let ipc_state = IpcState {
        connected,
        log_entries,
    };

    view! {
        <Router>
            <Provider value=ipc_state>
                <div class="min-h-screen bg-[#191C1D]">
                    <Nav />
                    <main class="max-w-4xl mx-auto p-8">
                        <Routes fallback=|| "Not found">
                            <Route path=path!("/") view=HomePage />
                            <Route path=path!("/ipc") view=IpcPage />
                            <Route path=path!("/api") view=ApiPage />
                            <Route path=path!("/settings") view=SettingsPage />
                        </Routes>
                    </main>
                </div>
            </Provider>
        </Router>
    }
}

#[component]
fn Nav() -> impl IntoView {
    let ipc_state = expect_context::<IpcState>();

    view! {
        <nav class="bg-[#2D3131] border-b border-[#444748] px-8 py-4">
            <div class="max-w-4xl mx-auto flex items-center gap-6">
                <A href="/" attr:class="text-[#F6F7F5] text-xl font-semibold hover:text-[#4EC6F0]">
                    "Native Leptos"
                </A>
                <div class="flex gap-4 ml-auto">
                    <A href="/ipc" attr:class="text-[#A9ACAC] hover:text-[#F6F7F5]">
                        <span class="flex items-center gap-2">
                            "IPC"
                            <span class=move || {
                                if ipc_state.connected.get() {
                                    "w-2 h-2 rounded-full bg-[#3DFFC5]"
                                } else {
                                    "w-2 h-2 rounded-full bg-[#FF5449]"
                                }
                            }></span>
                        </span>
                    </A>
                    <A href="/api" attr:class="text-[#A9ACAC] hover:text-[#F6F7F5]">"API"</A>
                    <A href="/settings" attr:class="text-[#A9ACAC] hover:text-[#F6F7F5]">"Settings"</A>
                </div>
            </div>
        </nav>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="text-[#F6F7F5]">
            <h1 class="text-4xl font-bold mb-4">"Welcome"</h1>
            <p class="text-[#A9ACAC] text-lg mb-8">
                "A native desktop app with a Leptos frontend."
            </p>

            <div class="grid grid-cols-2 gap-6">
                <A href="/ipc" attr:class="block bg-[#2D3131] rounded-xl p-6 hover:bg-[#3D4141] transition-colors">
                    <h2 class="text-xl font-semibold mb-2">"IPC Demo"</h2>
                    <p class="text-[#A9ACAC]">"Communication with native host via postcard serialization."</p>
                </A>
                <A href="/api" attr:class="block bg-[#2D3131] rounded-xl p-6 hover:bg-[#3D4141] transition-colors">
                    <h2 class="text-xl font-semibold mb-2">"API Demo"</h2>
                    <p class="text-[#A9ACAC]">"Communication with REST API server."</p>
                </A>
            </div>
        </div>
    }
}

#[component]
fn IpcPage() -> impl IntoView {
    let ipc_state = expect_context::<IpcState>();
    let (request_count, set_request_count) = signal(0u32);

    let request_random = move |_| {
        let id = request_count.get();
        set_request_count.set(id + 1);
        bridge::send_command(FrontendCommand::RequestRandomNumber { request_id: id });
    };

    view! {
        <div class="text-[#F6F7F5]">
            <h1 class="text-4xl font-bold mb-4">"IPC Demo"</h1>
            <p class="text-[#A9ACAC] mb-8">
                "Communication with native host via postcard serialization."
            </p>

            <div class="flex items-center gap-4 mb-6">
                <span class="font-medium">"Status:"</span>
                <span class=move || if ipc_state.connected.get() { "text-[#3DFFC5]" } else { "text-[#FF5449]" }>
                    {move || if ipc_state.connected.get() { "Connected" } else { "Disconnected" }}
                </span>
            </div>

            <Button
                variant=ButtonVariant::Primary
                on_click=Callback::new(request_random)
                disabled=MaybeProp::from(Signal::derive(move || !ipc_state.connected.get()))
            >
                "Request Random Number"
            </Button>

            <div class="mt-6 bg-[#111] rounded-lg p-4 font-mono text-sm max-h-64 overflow-y-auto">
                {move || {
                    let entries = ipc_state.log_entries.get();
                    if entries.is_empty() {
                        view! { <div class="text-[#666]">"No messages yet"</div> }.into_any()
                    } else {
                        entries.into_iter().map(|entry| {
                            view! {
                                <div class="py-1 border-b border-[#333]">{entry}</div>
                            }
                        }).collect_view().into_any()
                    }
                }}
            </div>
        </div>
    }
}

#[component]
fn ApiPage() -> impl IntoView {
    let (resources, set_resources) = signal(Vec::<Resource>::new());
    let (new_resource_name, set_new_resource_name) = signal(String::new());
    let (api_status, set_api_status) = signal(String::new());
    let (show_toast, set_show_toast) = signal(false);
    let (toast_message, set_toast_message) = signal(String::new());
    let (toast_type, set_toast_type) = signal(ToastType::Success);

    let show_notification = move |message: String, kind: ToastType| {
        set_toast_message.set(message);
        set_toast_type.set(kind);
        set_show_toast.set(true);
    };

    let refresh_resources = move |_| {
        spawn_local(async move {
            match api::list_resources().await {
                Ok(list) => {
                    set_resources.set(list);
                    set_api_status.set("Loaded".to_string());
                }
                Err(error) => {
                    show_notification(format!("Error: {error}"), ToastType::Error);
                }
            }
        });
    };

    let do_create = move || {
        let name = new_resource_name.get();
        if name.is_empty() {
            return;
        }
        set_new_resource_name.set(String::new());
        spawn_local(async move {
            match api::create_resource(name.clone(), None).await {
                Ok(resource) => {
                    set_resources.update(|list| list.insert(0, resource));
                    show_notification(format!("Created {name}"), ToastType::Success);
                }
                Err(error) => {
                    show_notification(format!("Error: {error}"), ToastType::Error);
                }
            }
        });
    };

    let create_resource = move |_| {
        do_create();
    };

    let on_keypress = move |event: web_sys::KeyboardEvent| {
        if event.key() == "Enter" {
            do_create();
        }
    };

    let delete_resource = move |id: String, name: String| {
        spawn_local(async move {
            match api::delete_resource(&id).await {
                Ok(()) => {
                    set_resources.update(|list| list.retain(|resource| resource.id != id));
                    show_notification(format!("Deleted {name}"), ToastType::Success);
                }
                Err(error) => {
                    show_notification(format!("Error: {error}"), ToastType::Error);
                }
            }
        });
    };

    view! {
        <div class="text-[#F6F7F5]">
            <h1 class="text-4xl font-bold mb-4">"API Demo"</h1>
            <p class="text-[#A9ACAC] mb-8">
                "Communication with REST API server."
            </p>

            <div class="flex items-center gap-4 mb-6">
                <span class="font-medium">"Status:"</span>
                <span class="text-[#A9ACAC]">
                    {move || {
                        let status = api_status.get();
                        if status.is_empty() { "Not loaded".to_string() } else { status }
                    }}
                </span>
                <Button variant=ButtonVariant::Secondary on_click=Callback::new(refresh_resources)>
                    "Refresh"
                </Button>
            </div>

            <div class="flex gap-3 mb-6">
                <input
                    type="text"
                    placeholder="Resource name"
                    class="flex-1 px-4 py-3 bg-[#2D3131] text-[#F6F7F5] border border-[#444748] rounded-lg focus:border-[#4EC6F0] outline-none"
                    prop:value=move || new_resource_name.get()
                    on:input=move |event| set_new_resource_name.set(event_target_value(&event))
                    on:keypress=on_keypress
                />
                <Button variant=ButtonVariant::Primary on_click=Callback::new(create_resource)>
                    "Create"
                </Button>
            </div>

            <div class="bg-[#111] rounded-lg p-4 max-h-80 overflow-y-auto">
                {move || {
                    let list = resources.get();
                    if list.is_empty() {
                        view! { <div class="text-[#666]">"No resources. Click Refresh to load."</div> }.into_any()
                    } else {
                        list.into_iter().map(|resource| {
                            let id = resource.id.clone();
                            let name = resource.name.clone();
                            let name_for_delete = resource.name.clone();
                            let id_for_delete = resource.id.clone();
                            view! {
                                <div class="flex justify-between items-center py-3 border-b border-[#333]">
                                    <div>
                                        <div class="font-medium">{name}</div>
                                        <div class="text-[#666] text-xs font-mono">{id}</div>
                                    </div>
                                    <Button
                                        variant=ButtonVariant::Danger
                                        size=ButtonSize::Small
                                        on_click=Callback::new(move |_| delete_resource(id_for_delete.clone(), name_for_delete.clone()))
                                    >
                                        "Delete"
                                    </Button>
                                </div>
                            }
                        }).collect_view().into_any()
                    }
                }}
            </div>

            <Toast
                visible=show_toast
                toast_type=toast_type.get()
                title=toast_message.get()
                on_close=Callback::new(move |_| set_show_toast.set(false))
            />
        </div>
    }
}

#[component]
fn SettingsPage() -> impl IntoView {
    let (dark_mode, set_dark_mode) = signal(true);
    let (notifications, set_notifications) = signal(true);

    view! {
        <div class="text-[#F6F7F5]">
            <h1 class="text-4xl font-bold mb-4">"Settings"</h1>
            <p class="text-[#A9ACAC] mb-8">
                "Configure your preferences."
            </p>

            <div class="space-y-4">
                <div class="bg-[#2D3131] rounded-xl p-6 flex items-center justify-between">
                    <div>
                        <h3 class="font-medium">"Dark Mode"</h3>
                        <p class="text-[#A9ACAC] text-sm">"Use dark theme throughout the app."</p>
                    </div>
                    <Toggle
                        value=dark_mode
                        on_change=Callback::new(move |value| set_dark_mode.set(value))
                    />
                </div>

                <div class="bg-[#2D3131] rounded-xl p-6 flex items-center justify-between">
                    <div>
                        <h3 class="font-medium">"Notifications"</h3>
                        <p class="text-[#A9ACAC] text-sm">"Receive notifications for events."</p>
                    </div>
                    <Toggle
                        value=notifications
                        on_change=Callback::new(move |value| set_notifications.set(value))
                    />
                </div>
            </div>
        </div>
    }
}

use api_types::resources::Resource;
use leptos::prelude::*;
use leptos::task::spawn_local;
use web_host_protocol::{BackendEvent, FrontendCommand};

pub mod api;
pub mod bridge;

#[component]
pub fn App() -> impl IntoView {
    let (connected, set_connected) = signal(false);
    let (request_count, set_request_count) = signal(0u32);
    let (log_entries, set_log_entries) = signal(Vec::<String>::new());
    let (resources, set_resources) = signal(Vec::<Resource>::new());
    let (new_resource_name, set_new_resource_name) = signal(String::new());
    let (api_status, set_api_status) = signal(String::new());

    Effect::new(move |_| {
        bridge::set_backend_handler(move |event| match event {
            BackendEvent::Connected => {
                set_connected.set(true);
                set_log_entries.update(|entries| entries.push("Backend connected".to_string()));
            }
            BackendEvent::RandomNumber { request_id, value } => {
                set_log_entries.update(|entries| entries.push(format!("Request #{request_id}: {value}")));
            }
        });
    });

    let refresh_resources = move |_| {
        spawn_local(async move {
            match api::list_resources().await {
                Ok(list) => {
                    set_resources.set(list);
                    set_api_status.set("Loaded".to_string());
                }
                Err(error) => {
                    set_api_status.set(format!("Error: {error}"));
                }
            }
        });
    };

    let request_random = move |_| {
        let id = request_count.get();
        set_request_count.set(id + 1);
        bridge::send_command(FrontendCommand::RequestRandomNumber { request_id: id });
    };

    let do_create = move || {
        let name = new_resource_name.get();
        if name.is_empty() {
            return;
        }
        set_new_resource_name.set(String::new());
        spawn_local(async move {
            match api::create_resource(name, None).await {
                Ok(resource) => {
                    set_resources.update(|list| list.insert(0, resource));
                    set_api_status.set("Created".to_string());
                }
                Err(error) => {
                    set_api_status.set(format!("Error: {error}"));
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

    let delete_resource = move |id: String| {
        spawn_local(async move {
            match api::delete_resource(&id).await {
                Ok(()) => {
                    set_resources.update(|list| list.retain(|resource| resource.id != id));
                    set_api_status.set("Deleted".to_string());
                }
                Err(error) => {
                    set_api_status.set(format!("Error: {error}"));
                }
            }
        });
    };

    view! {
        <div style="font-family: system-ui, sans-serif; max-width: 800px; margin: 40px auto; padding: 20px; color: #e5e5e5;">
            <h1 style="margin-bottom: 8px; color: #fff;">"Native Leptos Demo"</h1>

            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 32px; margin-top: 24px;">
                <div>
                    <h2 style="font-size: 18px; margin-bottom: 16px; color: #fff;">"IPC Demo"</h2>
                    <p style="color: #999; margin-bottom: 16px; font-size: 14px;">
                        "Communication with native host via postcard serialization."
                    </p>

                    <div style="display: flex; align-items: center; gap: 12px; margin-bottom: 16px;">
                        <span style="font-weight: 500; color: #e5e5e5;">"Status:"</span>
                        <span style:color=move || if connected.get() { "#22c55e" } else { "#ef4444" }>
                            {move || if connected.get() { "Connected" } else { "Disconnected" }}
                        </span>
                    </div>

                    <button
                        on:click=request_random
                        disabled=move || !connected.get()
                        style="padding: 10px 20px; font-size: 14px; cursor: pointer; background: #3b82f6; color: white; border: none; border-radius: 6px; margin-bottom: 16px;"
                    >
                        "Request Random Number"
                    </button>

                    <div style="background: #111; border-radius: 8px; padding: 12px; font-family: monospace; font-size: 13px; max-height: 200px; overflow-y: auto; color: #e5e5e5;">
                        {move || {
                            let entries = log_entries.get();
                            if entries.is_empty() {
                                view! { <div style="color: #666;">"No messages yet"</div> }.into_any()
                            } else {
                                entries.into_iter().map(|entry| {
                                    view! {
                                        <div style="padding: 4px 0; border-bottom: 1px solid #333;">
                                            {entry}
                                        </div>
                                    }
                                }).collect_view().into_any()
                            }
                        }}
                    </div>
                </div>

                <div>
                    <h2 style="font-size: 18px; margin-bottom: 16px; color: #fff;">"API Demo"</h2>
                    <p style="color: #999; margin-bottom: 16px; font-size: 14px;">
                        "Communication with REST API server."
                    </p>

                    <div style="display: flex; align-items: center; gap: 12px; margin-bottom: 16px;">
                        <span style="font-weight: 500; color: #e5e5e5;">"API:"</span>
                        <span style="color: #999;">
                            {move || {
                                let status = api_status.get();
                                if status.is_empty() { "Not loaded".to_string() } else { status }
                            }}
                        </span>
                        <button
                            on:click=refresh_resources
                            style="padding: 6px 12px; font-size: 12px; cursor: pointer; background: #374151; color: white; border: none; border-radius: 4px;"
                        >
                            "Refresh"
                        </button>
                    </div>

                    <div style="display: flex; gap: 8px; margin-bottom: 16px;">
                        <input
                            type="text"
                            placeholder="Resource name"
                            style="flex: 1; padding: 10px; font-size: 14px; background: #1f2937; color: white; border: 1px solid #374151; border-radius: 6px;"
                            prop:value=move || new_resource_name.get()
                            on:input=move |event| set_new_resource_name.set(event_target_value(&event))
                            on:keypress=on_keypress
                        />
                        <button
                            on:click=create_resource
                            style="padding: 10px 20px; font-size: 14px; cursor: pointer; background: #22c55e; color: white; border: none; border-radius: 6px;"
                        >
                            "Create"
                        </button>
                    </div>

                    <div style="background: #111; border-radius: 8px; padding: 12px; font-size: 13px; max-height: 200px; overflow-y: auto;">
                        {move || {
                            let list = resources.get();
                            if list.is_empty() {
                                view! { <div style="color: #666;">"No resources. Click Refresh to load."</div> }.into_any()
                            } else {
                                list.into_iter().map(|resource| {
                                    let id = resource.id.clone();
                                    let id_for_delete = resource.id.clone();
                                    view! {
                                        <div style="display: flex; justify-content: space-between; align-items: center; padding: 8px 0; border-bottom: 1px solid #333;">
                                            <div>
                                                <div style="color: #e5e5e5; font-weight: 500;">{resource.name}</div>
                                                <div style="color: #666; font-size: 11px; font-family: monospace;">{id}</div>
                                            </div>
                                            <button
                                                on:click=move |_| delete_resource(id_for_delete.clone())
                                                style="padding: 4px 8px; font-size: 12px; cursor: pointer; background: #dc2626; color: white; border: none; border-radius: 4px;"
                                            >
                                                "Delete"
                                            </button>
                                        </div>
                                    }
                                }).collect_view().into_any()
                            }
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}

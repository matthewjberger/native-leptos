use leptos::prelude::*;

use crate::icons::CloseIcon;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastType {
    #[default]
    Success,
    Warning,
    Info,
    Error,
}

impl ToastType {
    pub fn background_color(&self) -> &'static str {
        match self {
            ToastType::Success => "#3DFFC5",
            ToastType::Warning => "#FAC800",
            ToastType::Info => "#F6F7F5",
            ToastType::Error => "#FF5449",
        }
    }

    pub fn text_color(&self) -> &'static str {
        "#191C1D"
    }
}

#[component]
pub fn Toast(
    visible: ReadSignal<bool>,
    toast_type: ToastType,
    title: String,
    #[prop(optional)] message: Option<String>,
    #[prop(optional)] on_click: Option<Callback<()>>,
    on_close: Callback<()>,
    #[prop(optional)] auto_dismiss_ms: Option<u32>,
    #[prop(optional)] icon: Option<Children>,
) -> impl IntoView {
    let background_color = toast_type.background_color();
    let text_color = toast_type.text_color();

    view! {
        <div class=move || format!(
            "fixed ease-linear w-[92%] left-8 bottom-8 {} flex z-30 rounded-2xl overflow-hidden",
            if visible.get() {
                "duration-100 opacity-100 pointer-events-auto"
            } else {
                "duration-150 opacity-0 pointer-events-none"
            }
        )>
            <button
                class="flex-1 flex flex-row px-10 items-center"
                style=format!("background-color: {}", background_color)
                on:click=move |_| {
                    if let Some(callback) = on_click {
                        callback.run(());
                    }
                }
            >
                {icon.map(|icon_fn| view! {
                    <div class="mr-4">
                        {icon_fn()}
                    </div>
                })}
                <div class="flex flex-col flex-1 items-start pl-4">
                    <h1
                        class="text-5xl leading-none pb-2"
                        style=format!("color: {}", text_color)
                    >
                        {title}
                    </h1>
                    {message.map(|msg| view! {
                        <p class="text-2xl" style=format!("color: {}", text_color)>
                            {msg}
                        </p>
                    })}
                </div>
            </button>
            <button
                class="p-10"
                style=format!("background-color: {}", background_color)
                on:click=move |_| on_close.run(())
            >
                <CloseIcon size=89 color=text_color />
            </button>
            {auto_dismiss_ms.map(|_| view! {
                <div class=move || format!(
                    "bg-[#00A47C] absolute h-4 bottom-0 delay-150 {}",
                    if visible.get() { "w-[100%] ease-linear duration-[2850ms]" } else { "w-[0%]" }
                )/>
            })}
        </div>
    }
}

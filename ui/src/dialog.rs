use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DialogType {
    #[default]
    Info,
    Warning,
    Danger,
}

impl DialogType {
    pub fn background_color(&self) -> &'static str {
        match self {
            DialogType::Info => "#323837",
            DialogType::Warning => "#FF8D44",
            DialogType::Danger => "#FF5449",
        }
    }
}

#[component]
pub fn Dialog(
    visible: ReadSignal<bool>,
    #[prop(optional, into)] dialog_type: Signal<DialogType>,
    #[prop(optional)] delay: bool,
    #[prop(optional, into)] remove_padding: Signal<bool>,
    #[prop(optional)] hide_card: bool,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <div
            class=move || format!(
                "fixed top-0 left-0 z-50 flex flex-col justify-end h-screen w-screen bg-black bg-opacity-60 transition-all ease-in duration-150 {}",
                if visible.get() {
                    format!("opacity-100 backdrop-blur-md {}", if delay { "delay-500" } else { "" })
                } else {
                    "opacity-0 pointer-events-none delay-200".to_string()
                }
            )
        >
            <div
                class=move || format!(
                    "relative w-full flex flex-col items-center rounded-t-3xl transition-all ease-in duration-300 {} {}",
                    if hide_card || !visible.get() { "-bottom-[80%]" } else { "delay-100 bottom-0" },
                    if remove_padding.get() { "py-0" } else { "py-16" }
                )
                style=move || format!(
                    "background-color: {};",
                    dialog_type.get().background_color()
                )
            >
                {children()}
            </div>
        </div>
    }
}

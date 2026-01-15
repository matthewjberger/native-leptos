use leptos::prelude::*;

#[component]
pub fn Toggle(
    value: ReadSignal<bool>,
    #[prop(optional)] disabled: MaybeProp<bool>,
    #[prop(optional)] on_label: &'static str,
    #[prop(optional)] off_label: &'static str,
    on_change: Callback<bool>,
) -> impl IntoView {
    let on_label = if on_label.is_empty() {
        "Enabled"
    } else {
        on_label
    };
    let off_label = if off_label.is_empty() {
        "Disabled"
    } else {
        off_label
    };

    view! {
        <div class="flex justify-between gap-8">
            <button
                class=move || format!(
                    "w-full flex items-center justify-center text-4xl border-2 rounded-[70px] py-6 px-9 border-[#F6F7F5] {} disabled:border-[#5C5F5F]",
                    if !value.get() {
                        "bg-[#F6F7F5] disabled:bg-[#5C5F5F] disabled:text-[#2D3131]"
                    } else {
                        "text-[#F6F7F5] disabled:bg-[#2D3131] disabled:text-[#5C5F5F]"
                    }
                )
                disabled=move || disabled.get().unwrap_or(false)
                on:click=move |_| on_change.run(false)
            >
                {off_label}
            </button>
            <button
                class=move || format!(
                    "w-full flex items-center justify-center text-4xl border-2 rounded-[70px] py-6 px-9 border-[#F6F7F5] {} disabled:border-[#5C5F5F]",
                    if value.get() {
                        "bg-[#F6F7F5] disabled:bg-[#5C5F5F] disabled:text-[#2D3131]"
                    } else {
                        "text-[#F6F7F5] disabled:bg-[#2D3131] disabled:text-[#5C5F5F]"
                    }
                )
                disabled=move || disabled.get().unwrap_or(false)
                on:click=move |_| on_change.run(true)
            >
                {on_label}
            </button>
        </div>
    }
}

#[component]
pub fn ToggleGroup(
    options: Vec<(String, &'static str)>,
    value: ReadSignal<String>,
    on_change: Callback<String>,
) -> impl IntoView {
    view! {
        <div class="flex">
            {options.into_iter().map(|(option_value, label)| {
                let option_value_clone = option_value.clone();
                let option_value_for_check = option_value.clone();
                view! {
                    <button
                        class=move || format!(
                            "flex-1 text-4xl mr-4 p-8 rounded-full {}",
                            if value.get() == option_value_for_check {
                                "bg-[#F6F7F5]"
                            } else {
                                "border border-[#C4C7C7] text-[#F6F7F5]"
                            }
                        )
                        on:click={
                            let value = option_value_clone.clone();
                            move |_| on_change.run(value.clone())
                        }
                    >
                        {label}
                    </button>
                }
            }).collect_view()}
        </div>
    }
}

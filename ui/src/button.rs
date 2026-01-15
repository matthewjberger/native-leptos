use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Outline,
    Danger,
    Warning,
    Ghost,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
    XLarge,
}

impl ButtonSize {
    fn padding_class(&self) -> &'static str {
        match self {
            ButtonSize::Small => "py-4 px-6",
            ButtonSize::Medium => "py-6 px-9",
            ButtonSize::Large => "py-8 px-12",
            ButtonSize::XLarge => "py-10 px-16",
        }
    }

    fn text_class(&self) -> &'static str {
        match self {
            ButtonSize::Small => "text-2xl",
            ButtonSize::Medium => "text-4xl",
            ButtonSize::Large => "text-5xl",
            ButtonSize::XLarge => "text-6xl",
        }
    }
}

impl ButtonVariant {
    fn base_class(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => "bg-[#4EC6F0] text-[#191C1D] border-[#4EC6F0]",
            ButtonVariant::Secondary => "bg-[#F6F7F5] text-[#191C1D] border-[#F6F7F5]",
            ButtonVariant::Outline => "bg-transparent text-[#F6F7F5] border-[#F6F7F5]",
            ButtonVariant::Danger => "bg-[#FF5449] text-[#191C1D] border-[#FF5449]",
            ButtonVariant::Warning => "bg-[#FF8D44] text-[#191C1D] border-[#FF8D44]",
            ButtonVariant::Ghost => "bg-transparent text-[#F6F7F5] border-transparent",
        }
    }

    fn disabled_class(&self) -> &'static str {
        match self {
            ButtonVariant::Primary
            | ButtonVariant::Secondary
            | ButtonVariant::Danger
            | ButtonVariant::Warning => {
                "disabled:bg-[#444748] disabled:text-[#5C5F5F] disabled:border-[#444748]"
            }
            ButtonVariant::Outline | ButtonVariant::Ghost => {
                "disabled:text-[#5C5F5F] disabled:border-[#5C5F5F]"
            }
        }
    }
}

#[component]
pub fn Button(
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] size: ButtonSize,
    #[prop(optional)] disabled: MaybeProp<bool>,
    #[prop(optional)] full_width: bool,
    #[prop(optional)] rounded_full: bool,
    #[prop(optional, into)] class: String,
    #[prop(optional)] on_click: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let rounded_class = if rounded_full {
        "rounded-full"
    } else {
        "rounded-2xl"
    };
    let width_class = if full_width { "w-full" } else { "" };

    view! {
        <button
            class=format!(
                "flex items-center justify-center border-2 transition-all {} {} {} {} {} {}",
                variant.base_class(),
                variant.disabled_class(),
                size.padding_class(),
                size.text_class(),
                rounded_class,
                width_class,
            ) + " " + &class
            disabled=move || disabled.get().unwrap_or(false)
            on:click=move |_| {
                if let Some(callback) = on_click {
                    callback.run(());
                }
            }
        >
            {children()}
        </button>
    }
}

#[component]
pub fn IconButton(
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] size: ButtonSize,
    #[prop(optional)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] class: String,
    #[prop(optional)] on_click: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let padding = match size {
        ButtonSize::Small => "p-4",
        ButtonSize::Medium => "p-6",
        ButtonSize::Large => "p-8",
        ButtonSize::XLarge => "p-10",
    };

    view! {
        <button
            class=format!(
                "flex items-center justify-center rounded-full border-2 transition-all {} {} {}",
                variant.base_class(),
                variant.disabled_class(),
                padding,
            ) + " " + &class
            disabled=move || disabled.get().unwrap_or(false)
            on:click=move |_| {
                if let Some(callback) = on_click {
                    callback.run(());
                }
            }
        >
            {children()}
        </button>
    }
}

#[component]
pub fn NavButton(
    #[prop(optional)] background: &'static str,
    #[prop(optional)] disabled: MaybeProp<bool>,
    #[prop(optional)] on_click: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let background = if background.is_empty() {
        "#191C1D"
    } else {
        background
    };

    view! {
        <button
            class="rounded-full p-10 disabled:opacity-20"
            style=format!("background-color: {}", background)
            disabled=move || disabled.get().unwrap_or(false)
            on:click=move |_| {
                if let Some(callback) = on_click {
                    callback.run(());
                }
            }
        >
            {children()}
        </button>
    }
}

#[component]
pub fn SelectButton(
    #[prop(optional)] disabled: MaybeProp<bool>,
    #[prop(optional)] on_click: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            class="flex items-center text-4xl text-[#F6F7F5] border-2 rounded-[70px] py-6 px-9 border-[#747878] disabled:opacity-20"
            disabled=move || disabled.get().unwrap_or(false)
            on:click=move |_| {
                if let Some(callback) = on_click {
                    callback.run(());
                }
            }
        >
            {children()}
        </button>
    }
}

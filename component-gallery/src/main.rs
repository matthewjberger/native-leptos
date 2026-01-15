use leptos::prelude::*;
use ui::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum ComponentCategory {
    #[default]
    Icons,
    Buttons,
    Dialog,
    Toast,
    Toggle,
}

impl ComponentCategory {
    fn all() -> Vec<ComponentCategory> {
        vec![
            ComponentCategory::Icons,
            ComponentCategory::Buttons,
            ComponentCategory::Dialog,
            ComponentCategory::Toast,
            ComponentCategory::Toggle,
        ]
    }

    fn label(&self) -> &'static str {
        match self {
            ComponentCategory::Icons => "Icons",
            ComponentCategory::Buttons => "Buttons",
            ComponentCategory::Dialog => "Dialog",
            ComponentCategory::Toast => "Toast",
            ComponentCategory::Toggle => "Toggle",
        }
    }
}

#[component]
fn App() -> impl IntoView {
    let (selected_category, set_selected_category) = signal(ComponentCategory::Icons);

    view! {
        <div class="flex h-screen bg-[#191C1D]">
            <Sidebar selected=selected_category set_selected=set_selected_category />
            <main class="flex-1 overflow-auto p-8">
                <ComponentViewer category=selected_category />
            </main>
        </div>
    }
}

#[component]
fn Sidebar(
    selected: ReadSignal<ComponentCategory>,
    set_selected: WriteSignal<ComponentCategory>,
) -> impl IntoView {
    view! {
        <aside class="w-80 bg-[#2D3131] border-r border-[#444748] flex flex-col">
            <div class="p-6 border-b border-[#444748]">
                <h1 class="text-[#F6F7F5] text-2xl font-semibold mt-2">"Component Gallery"</h1>
            </div>
            <nav class="flex-1 p-4">
                <h2 class="text-[#A9ACAC] text-sm uppercase tracking-wider mb-4 px-3">"Components"</h2>
                {ComponentCategory::all().into_iter().map(|category| {
                    view! {
                        <button
                            class=move || format!(
                                "w-full text-left px-4 py-3 rounded-lg mb-1 text-lg transition-colors {}",
                                if selected.get() == category {
                                    "bg-[#4EC6F0] text-[#191C1D]"
                                } else {
                                    "text-[#F6F7F5] hover:bg-[#444748]"
                                }
                            )
                            on:click=move |_| set_selected.set(category)
                        >
                            {category.label()}
                        </button>
                    }
                }).collect_view()}
            </nav>
        </aside>
    }
}

#[component]
fn ComponentViewer(category: ReadSignal<ComponentCategory>) -> impl IntoView {
    view! {
        <div class="text-[#F6F7F5]">
            {move || match category.get() {
                ComponentCategory::Icons => view! { <IconsShowcase /> }.into_any(),
                ComponentCategory::Buttons => view! { <ButtonsShowcase /> }.into_any(),
                ComponentCategory::Dialog => view! { <DialogShowcase /> }.into_any(),
                ComponentCategory::Toast => view! { <ToastShowcase /> }.into_any(),
                ComponentCategory::Toggle => view! { <ToggleShowcase /> }.into_any(),
            }}
        </div>
    }
}

#[component]
fn ShowcaseSection(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="mb-12">
            <h2 class="text-3xl font-semibold mb-6 text-[#4EC6F0]">{title}</h2>
            <div class="bg-[#2D3131] rounded-2xl p-8">
                {children()}
            </div>
        </div>
    }
}

#[component]
fn ShowcaseItem(label: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="mb-6 last:mb-0">
            <p class="text-[#A9ACAC] text-sm mb-3">{label}</p>
            <div class="flex flex-wrap items-center gap-4">
                {children()}
            </div>
        </div>
    }
}

#[component]
fn IconsShowcase() -> impl IntoView {
    view! {
        <h1 class="text-5xl font-bold mb-8">"Icons"</h1>
        <p class="text-[#A9ACAC] text-xl mb-8">"A collection of SVG icons for the UI."</p>

        <ShowcaseSection title="Action Icons">
            <div class="grid grid-cols-6 gap-8">
                <div class="flex flex-col items-center gap-2">
                    <div class="bg-[#444748] p-4 rounded-xl"><CheckIcon size=64 color="#F6F7F5" /></div>
                    <span class="text-sm text-[#A9ACAC]">"Check"</span>
                </div>
                <div class="flex flex-col items-center gap-2">
                    <div class="bg-[#444748] p-4 rounded-xl"><CloseIcon size=64 color="#F6F7F5" /></div>
                    <span class="text-sm text-[#A9ACAC]">"Close"</span>
                </div>
                <div class="flex flex-col items-center gap-2">
                    <div class="bg-[#444748] p-4 rounded-xl"><BackArrowIcon size=64 /></div>
                    <span class="text-sm text-[#A9ACAC]">"Back"</span>
                </div>
                <div class="flex flex-col items-center gap-2">
                    <div class="bg-[#444748] p-4 rounded-xl"><ForwardArrowIcon size=64 /></div>
                    <span class="text-sm text-[#A9ACAC]">"Forward"</span>
                </div>
                <div class="flex flex-col items-center gap-2">
                    <div class="bg-[#444748] p-4 rounded-xl"><TrashIcon size=64 color="#F6F7F5" /></div>
                    <span class="text-sm text-[#A9ACAC]">"Trash"</span>
                </div>
                <div class="flex flex-col items-center gap-2">
                    <div class="bg-[#444748] p-4 rounded-xl"><SearchIcon size=64 /></div>
                    <span class="text-sm text-[#A9ACAC]">"Search"</span>
                </div>
            </div>
        </ShowcaseSection>

        <ShowcaseSection title="Status Icons">
            <div class="grid grid-cols-6 gap-8">
                <div class="flex flex-col items-center gap-2">
                    <div class="bg-[#444748] p-4 rounded-xl"><InfoIcon size=64 color="#F6F7F5" /></div>
                    <span class="text-sm text-[#A9ACAC]">"Info"</span>
                </div>
                <div class="flex flex-col items-center gap-2">
                    <div class="bg-[#444748] p-4 rounded-xl"><WarningIcon size=64 color="#F6F7F5" /></div>
                    <span class="text-sm text-[#A9ACAC]">"Warning"</span>
                </div>
                <div class="flex flex-col items-center gap-2">
                    <div class="bg-[#444748] p-4 rounded-xl"><SpinnerIcon size=64 /></div>
                    <span class="text-sm text-[#A9ACAC]">"Spinner"</span>
                </div>
            </div>
        </ShowcaseSection>

        <ShowcaseSection title="UI Icons">
            <div class="grid grid-cols-6 gap-8">
                <div class="flex flex-col items-center gap-2">
                    <div class="bg-[#444748] p-4 rounded-xl"><MenuIcon /></div>
                    <span class="text-sm text-[#A9ACAC]">"Menu"</span>
                </div>
                <div class="flex flex-col items-center gap-2">
                    <div class="bg-[#444748] p-4 rounded-xl"><SettingsIcon /></div>
                    <span class="text-sm text-[#A9ACAC]">"Settings"</span>
                </div>
                <div class="flex flex-col items-center gap-2">
                    <div class="bg-[#444748] p-4 rounded-xl"><HomeIcon size=64 /></div>
                    <span class="text-sm text-[#A9ACAC]">"Home"</span>
                </div>
                <div class="flex flex-col items-center gap-2">
                    <div class="bg-[#444748] p-4 rounded-xl"><PlusIcon size=64 /></div>
                    <span class="text-sm text-[#A9ACAC]">"Plus"</span>
                </div>
                <div class="flex flex-col items-center gap-2">
                    <div class="bg-[#444748] p-4 rounded-xl"><MinusIcon size=64 /></div>
                    <span class="text-sm text-[#A9ACAC]">"Minus"</span>
                </div>
            </div>
        </ShowcaseSection>
    }
}

#[component]
fn ButtonsShowcase() -> impl IntoView {
    view! {
        <h1 class="text-5xl font-bold mb-8">"Buttons"</h1>
        <p class="text-[#A9ACAC] text-xl mb-8">"Various button styles and variants for different use cases."</p>

        <ShowcaseSection title="Button Variants">
            <div class="flex flex-wrap gap-4">
                <Button variant=ButtonVariant::Primary>"Primary"</Button>
                <Button variant=ButtonVariant::Secondary>"Secondary"</Button>
                <Button variant=ButtonVariant::Outline>"Outline"</Button>
                <Button variant=ButtonVariant::Danger>"Danger"</Button>
                <Button variant=ButtonVariant::Warning>"Warning"</Button>
                <Button variant=ButtonVariant::Ghost>"Ghost"</Button>
            </div>
        </ShowcaseSection>

        <ShowcaseSection title="Button Sizes">
            <div class="flex flex-wrap items-end gap-4">
                <Button size=ButtonSize::Small>"Small"</Button>
                <Button size=ButtonSize::Medium>"Medium"</Button>
                <Button size=ButtonSize::Large>"Large"</Button>
                <Button size=ButtonSize::XLarge>"XLarge"</Button>
            </div>
        </ShowcaseSection>

        <ShowcaseSection title="Button States">
            <div class="flex flex-wrap gap-4 mb-4">
                <Button>"Normal"</Button>
                <Button disabled=MaybeProp::from(Some(true))>"Disabled"</Button>
                <Button rounded_full=true>"Rounded Full"</Button>
            </div>
            <div class="mt-4">
                <Button full_width=true>"Full Width Button"</Button>
            </div>
        </ShowcaseSection>

        <ShowcaseSection title="Icon Buttons">
            <div class="flex flex-wrap gap-4">
                <IconButton variant=ButtonVariant::Primary>
                    <CheckIcon size=32 color="#191C1D" />
                </IconButton>
                <IconButton variant=ButtonVariant::Secondary>
                    <CloseIcon size=32 color="#191C1D" />
                </IconButton>
                <IconButton variant=ButtonVariant::Outline>
                    <BackArrowIcon size=32 />
                </IconButton>
                <IconButton variant=ButtonVariant::Danger>
                    <TrashIcon size=32 color="#191C1D" />
                </IconButton>
            </div>
        </ShowcaseSection>

        <ShowcaseSection title="Specialized Buttons">
            <div class="flex flex-wrap gap-4">
                <NavButton><BackArrowIcon /></NavButton>
                <SelectButton>"Select"</SelectButton>
            </div>
        </ShowcaseSection>
    }
}

#[component]
fn DialogShowcase() -> impl IntoView {
    let (show_info_dialog, set_show_info_dialog) = signal(false);
    let (show_warning_dialog, set_show_warning_dialog) = signal(false);
    let (show_danger_dialog, set_show_danger_dialog) = signal(false);

    view! {
        <h1 class="text-5xl font-bold mb-8">"Dialogs"</h1>
        <p class="text-[#A9ACAC] text-xl mb-8">"Modal dialogs for confirmations, alerts, and detailed views."</p>

        <ShowcaseSection title="Dialog Types">
            <ShowcaseItem label="Click to open dialogs">
                <button class="bg-[#4EC6F0] text-[#191C1D] px-6 py-3 rounded-lg" on:click=move |_| set_show_info_dialog.set(true)>"Info Dialog"</button>
                <button class="bg-[#FF8D44] text-[#191C1D] px-6 py-3 rounded-lg" on:click=move |_| set_show_warning_dialog.set(true)>"Warning Dialog"</button>
                <button class="bg-[#FF5449] text-[#191C1D] px-6 py-3 rounded-lg" on:click=move |_| set_show_danger_dialog.set(true)>"Danger Dialog"</button>
            </ShowcaseItem>
        </ShowcaseSection>

        <Dialog visible=show_info_dialog dialog_type=Signal::derive(|| DialogType::Info)>
            <h1 class="text-[#F6F7F5] text-5xl font-semibold mb-4">"Info Dialog"</h1>
            <p class="text-[#F6F7F5] text-2xl mb-8">"This is an informational dialog."</p>
            <button class="bg-[#4EC6F0] text-[#191C1D] px-8 py-4 rounded-full text-xl" on:click=move |_| set_show_info_dialog.set(false)>"Close"</button>
        </Dialog>

        <Dialog visible=show_warning_dialog dialog_type=Signal::derive(|| DialogType::Warning)>
            <h1 class="text-[#191C1D] text-5xl font-semibold mb-4">"Warning Dialog"</h1>
            <p class="text-[#191C1D] text-2xl mb-8">"This is a warning dialog."</p>
            <button class="bg-white text-[#191C1D] px-8 py-4 rounded-full text-xl" on:click=move |_| set_show_warning_dialog.set(false)>"Close"</button>
        </Dialog>

        <Dialog visible=show_danger_dialog dialog_type=Signal::derive(|| DialogType::Danger)>
            <h1 class="text-[#191C1D] text-5xl font-semibold mb-4">"Danger Dialog"</h1>
            <p class="text-[#191C1D] text-2xl mb-8">"This is a danger dialog."</p>
            <button class="bg-white text-[#191C1D] px-8 py-4 rounded-full text-xl" on:click=move |_| set_show_danger_dialog.set(false)>"Close"</button>
        </Dialog>
    }
}

#[component]
fn ToastShowcase() -> impl IntoView {
    let (show_success, set_show_success) = signal(false);
    let (show_warning, set_show_warning) = signal(false);
    let (show_info, set_show_info) = signal(false);
    let (show_error, set_show_error) = signal(false);

    view! {
        <h1 class="text-5xl font-bold mb-8">"Toast Notifications"</h1>
        <p class="text-[#A9ACAC] text-xl mb-8">"Temporary notifications for user feedback."</p>

        <ShowcaseSection title="Toast Types">
            <ShowcaseItem label="Click to show toasts">
                <button class="bg-[#3DFFC5] text-[#191C1D] px-6 py-3 rounded-lg" on:click=move |_| set_show_success.set(true)>"Success Toast"</button>
                <button class="bg-[#FAC800] text-[#191C1D] px-6 py-3 rounded-lg" on:click=move |_| set_show_warning.set(true)>"Warning Toast"</button>
                <button class="bg-[#F6F7F5] text-[#191C1D] px-6 py-3 rounded-lg" on:click=move |_| set_show_info.set(true)>"Info Toast"</button>
                <button class="bg-[#FF5449] text-[#191C1D] px-6 py-3 rounded-lg" on:click=move |_| set_show_error.set(true)>"Error Toast"</button>
            </ShowcaseItem>
        </ShowcaseSection>

        <Toast
            visible=show_success
            toast_type=ToastType::Success
            title="Success!".to_string()
            on_close=Callback::new(move |_| set_show_success.set(false))
        />

        <Toast
            visible=show_warning
            toast_type=ToastType::Warning
            title="Warning".to_string()
            on_close=Callback::new(move |_| set_show_warning.set(false))
        />

        <Toast
            visible=show_info
            toast_type=ToastType::Info
            title="Info".to_string()
            on_close=Callback::new(move |_| set_show_info.set(false))
        />

        <Toast
            visible=show_error
            toast_type=ToastType::Error
            title="Error".to_string()
            on_close=Callback::new(move |_| set_show_error.set(false))
        />
    }
}

#[component]
fn ToggleShowcase() -> impl IntoView {
    let (toggle_value, set_toggle_value) = signal(false);
    let (group_value, set_group_value) = signal("1x".to_string());

    view! {
        <h1 class="text-5xl font-bold mb-8">"Toggles"</h1>
        <p class="text-[#A9ACAC] text-xl mb-8">"Toggle switches and selection groups."</p>

        <ShowcaseSection title="Basic Toggle">
            <ShowcaseItem label="Enabled/Disabled Toggle">
                <div class="w-96">
                    <Toggle
                        value=toggle_value
                        on_change=Callback::new(move |value| set_toggle_value.set(value))
                    />
                </div>
            </ShowcaseItem>
            <ShowcaseItem label="Current state">
                <span class="text-xl">{move || if toggle_value.get() { "Enabled" } else { "Disabled" }}</span>
            </ShowcaseItem>
        </ShowcaseSection>

        <ShowcaseSection title="Toggle Group">
            <ShowcaseItem label="Multiplier Selection">
                <div class="w-full max-w-2xl">
                    <ToggleGroup
                        options=vec![
                            ("None".to_string(), "None"),
                            ("Half".to_string(), "Half"),
                            ("1x".to_string(), "1x"),
                            ("2x".to_string(), "2x"),
                        ]
                        value=group_value
                        on_change=Callback::new(move |value: String| set_group_value.set(value))
                    />
                </div>
            </ShowcaseItem>
            <ShowcaseItem label="Selected value">
                <span class="text-xl">{group_value}</span>
            </ShowcaseItem>
        </ShowcaseSection>
    }
}

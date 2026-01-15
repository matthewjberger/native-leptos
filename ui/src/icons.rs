use leptos::prelude::*;

#[component]
pub fn CloseIcon(
    #[prop(optional)] size: Option<i32>,
    #[prop(optional)] color: Option<&'static str>,
) -> impl IntoView {
    let size = size.unwrap_or(89);
    let color = color.unwrap_or("#191C1D");
    view! {
        <svg width=size height=size viewBox="0 0 89 89" fill="none">
            <path d="M70.1666 24.0035L64.9966 18.8335L44.4999 39.3302L24.0033 18.8335L18.8333 24.0035L39.3299 44.5002L18.8333 64.9968L24.0033 70.1668L44.4999 49.6702L64.9966 70.1668L70.1666 64.9968L49.6699 44.5002L70.1666 24.0035Z" fill=color/>
        </svg>
    }
}

#[component]
pub fn CheckIcon(
    #[prop(optional)] size: Option<i32>,
    #[prop(optional)] color: Option<&'static str>,
) -> impl IntoView {
    let size = size.unwrap_or(45);
    let color = color.unwrap_or("#2D3131");
    view! {
        <svg width=size height=size viewBox="0 0 45 45" fill="none">
            <path d="M17 30.1447L9.35504 22.4997L6.75171 25.0847L17 35.333L39 13.333L36.415 10.748L17 30.1447Z" fill=color/>
        </svg>
    }
}

#[component]
pub fn BackArrowIcon(
    #[prop(optional)] size: Option<i32>,
    #[prop(optional)] color: Option<&'static str>,
) -> impl IntoView {
    let size = size.unwrap_or(74);
    let color = color.unwrap_or("#F6F7F5");
    view! {
        <svg width=size height=size viewBox="0 0 74 74" fill="none">
            <path d="M61.6654 33.9173H24.1412L41.377 16.6815L36.9987 12.334L12.332 37.0007L36.9987 61.6673L41.3462 57.3198L24.1412 40.084H61.6654V33.9173Z" fill=color/>
        </svg>
    }
}

#[component]
pub fn ForwardArrowIcon(
    #[prop(optional)] size: Option<i32>,
    #[prop(optional)] color: Option<&'static str>,
) -> impl IntoView {
    let size = size.unwrap_or(74);
    let color = color.unwrap_or("#191C1D");
    view! {
        <svg width=size height=size viewBox="0 0 74 74" fill="none" class="rotate-180">
            <path d="M61.6654 33.9173H24.1412L41.377 16.6815L36.9987 12.334L12.332 37.0007L36.9987 61.6673L41.3462 57.3198L24.1412 40.084H61.6654V33.9173Z" fill=color/>
        </svg>
    }
}

#[component]
pub fn MenuIcon() -> impl IntoView {
    view! {
        <svg width="84" height="84" viewBox="0 0 84 84" fill="none">
            <path
                d="M41.3407 18.9907L41.985 19.2157L42.6324 18.9999C53.7928 15.28 62.3057 15.37 76 18.6705V67.2667C76 67.267 76 67.2673 76 67.2676C75.9986 67.269 75.9965 67.271 75.9936 67.2732C75.9855 67.2795 75.977 67.2829 75.9708 67.2842C75.9682 67.2848 75.9657 67.285 75.9631 67.285C75.9606 67.285 75.9555 67.2847 75.947 67.2825C64.0852 64.2543 55.1155 64.1796 42.0219 67.8195C30.1009 64.2397 21.0741 64.0752 8.04798 67.3057C8.03981 67.3077 8.03511 67.3079 8.03308 67.3079C8.03082 67.308 8.02877 67.3077 8.0265 67.3072C8.02123 67.3061 8.01344 67.3029 8.00581 67.2971C8.00318 67.295 8.0013 67.2932 8.00001 67.2919C8.00001 67.2916 8 67.2913 8 67.291V18.6643C21.7907 15.2796 30.1813 15.0939 41.3407 18.9907Z"
                stroke="currentColor"
                stroke-width="4"
                stroke-miterlimit="16"
            />
            <path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M67.5557 31.5809C60.9249 30.9511 56.5688 30.8536 48.7266 31.5809V26.7335C56.9124 25.9742 60.626 26.0751 67.5557 26.7333V31.5809Z"
                fill="currentColor"
            />
            <path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M67.5557 42.3407C60.9249 41.7137 56.5688 41.6166 48.7266 42.3407V37.491C56.9124 36.7352 60.626 36.8352 67.5557 37.4905V42.3407Z"
                fill="currentColor"
            />
            <path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M60.831 53.0995C55.9892 52.4843 53.5683 52.4843 48.7266 53.0995V48.1817C53.5683 47.5659 55.9892 47.566 60.831 48.181V53.0995Z"
                fill="currentColor"
            />
            <path
                d="M41.3293 66.5493L10.3957 64.5319L9.05078 20.8213L25.1901 17.459L41.3293 20.8213V66.5493Z"
                fill="currentColor"
            />
        </svg>
    }
}

#[component]
pub fn SettingsIcon() -> impl IntoView {
    view! {
        <svg width="84" height="84" viewBox="0 0 84 84" fill="none">
            <path d="M10.5 11L10.5 39L38.5 39L38.5 11L10.5 11ZM31.5 32L17.5 32L17.5 18L31.5 18L31.5 32ZM10.5 46L10.5 74L38.5 74L38.5 46L10.5 46ZM31.5 67L17.5 67L17.5 53L31.5 53L31.5 67ZM45.5 11L45.5 39L73.5 39L73.5 11L45.5 11ZM66.5 32L52.5 32L52.5 18L66.5 18L66.5 32ZM45.5 46L45.5 74L73.5 74L73.5 46L45.5 46ZM66.5 67L52.5 67L52.5 53L66.5 53V67Z" fill="currentColor"/>
        </svg>
    }
}

#[component]
pub fn InfoIcon(
    #[prop(optional)] size: Option<i32>,
    #[prop(optional)] color: Option<&'static str>,
) -> impl IntoView {
    let size = size.unwrap_or(116);
    let color = color.unwrap_or("#191C1D");
    view! {
        <svg width=size height=size viewBox="0 0 117 116" fill="none">
            <path d="M53.6667 33.8327H63.3334V43.4994H53.6667V33.8327ZM53.6667 53.166H63.3334V82.166H53.6667V53.166ZM58.5001 9.66602C31.8201 9.66602 10.1667 31.3193 10.1667 57.9994C10.1667 84.6794 31.8201 106.333 58.5001 106.333C85.1801 106.333 106.833 84.6794 106.833 57.9994C106.833 31.3193 85.1801 9.66602 58.5001 9.66602ZM58.5001 96.666C37.1851 96.666 19.8334 79.3144 19.8334 57.9994C19.8334 36.6844 37.1851 19.3327 58.5001 19.3327C79.8151 19.3327 97.1667 36.6844 97.1667 57.9994C97.1667 79.3144 79.8151 96.666 58.5001 96.666Z" fill=color/>
        </svg>
    }
}

#[component]
pub fn WarningIcon(
    #[prop(optional)] size: Option<i32>,
    #[prop(optional)] color: Option<&'static str>,
) -> impl IntoView {
    let size = size.unwrap_or(106);
    let color = color.unwrap_or("#171000");
    view! {
        <svg width=size height=size viewBox="0 0 106 106" fill="none">
            <path d="M53.0013 28.664L86.2588 86.1248H19.7438L53.0013 28.664ZM53.0013 11.0415L4.41797 94.9582H101.585L53.0013 11.0415ZM57.418 72.8748H48.5846V81.7082H57.418V72.8748ZM57.418 46.3748H48.5846V64.0415H57.418V46.3748Z" fill=color/>
        </svg>
    }
}

#[component]
pub fn TrashIcon(
    #[prop(optional)] size: Option<i32>,
    #[prop(optional)] color: Option<&'static str>,
) -> impl IntoView {
    let size = size.unwrap_or(56);
    let color = color.unwrap_or("#191C1D");
    view! {
        <svg width=size height=size viewBox="0 0 56 56" fill="none">
            <path
                d="M37.3327 21V44.3333H18.666L18.666 21H37.3327ZM33.8327 7L22.166 7L19.8327 9.33333H11.666V14L44.3327 14V9.33333H36.166L33.8327 7ZM41.9993 16.3333H13.9993L13.9993 44.3333C13.9993 46.9 16.0993 49 18.666 49H37.3327C39.8993 49 41.9993 46.9 41.9993 44.3333V16.3333Z"
                fill=color
            />
        </svg>
    }
}

#[component]
pub fn SpinnerIcon(
    #[prop(optional)] size: Option<i32>,
    #[prop(optional)] track_color: Option<&'static str>,
    #[prop(optional)] spinner_color: Option<&'static str>,
) -> impl IntoView {
    let size = size.unwrap_or(84);
    let track_color = track_color.unwrap_or("#444748");
    let spinner_color = spinner_color.unwrap_or("#FFB4AB");
    view! {
        <svg width=format!("{}px", size) height=format!("{}px", size) viewBox="0 0 800 800" class="inline animate-spin">
            <circle cx="400" cy="400" fill="none" r="360" stroke-width="84" stroke=track_color/>
            <circle cx="400" cy="400" fill="none" r="360" stroke-width="84" stroke=spinner_color stroke-dasharray="432 1400"/>
        </svg>
    }
}

#[component]
pub fn PlusIcon(
    #[prop(optional)] size: Option<i32>,
    #[prop(optional)] color: Option<&'static str>,
) -> impl IntoView {
    let size = size.unwrap_or(24);
    let color = color.unwrap_or("#F6F7F5");
    view! {
        <svg width=size height=size viewBox="0 0 24 24" fill="none">
            <path d="M19 13H13V19H11V13H5V11H11V5H13V11H19V13Z" fill=color/>
        </svg>
    }
}

#[component]
pub fn MinusIcon(
    #[prop(optional)] size: Option<i32>,
    #[prop(optional)] color: Option<&'static str>,
) -> impl IntoView {
    let size = size.unwrap_or(24);
    let color = color.unwrap_or("#F6F7F5");
    view! {
        <svg width=size height=size viewBox="0 0 24 24" fill="none">
            <path d="M19 13H5V11H19V13Z" fill=color/>
        </svg>
    }
}

#[component]
pub fn SearchIcon(
    #[prop(optional)] size: Option<i32>,
    #[prop(optional)] color: Option<&'static str>,
) -> impl IntoView {
    let size = size.unwrap_or(24);
    let color = color.unwrap_or("#F6F7F5");
    view! {
        <svg width=size height=size viewBox="0 0 24 24" fill="none">
            <path d="M15.5 14H14.71L14.43 13.73C15.41 12.59 16 11.11 16 9.5C16 5.91 13.09 3 9.5 3C5.91 3 3 5.91 3 9.5C3 13.09 5.91 16 9.5 16C11.11 16 12.59 15.41 13.73 14.43L14 14.71V15.5L19 20.49L20.49 19L15.5 14ZM9.5 14C7.01 14 5 11.99 5 9.5C5 7.01 7.01 5 9.5 5C11.99 5 14 7.01 14 9.5C14 11.99 11.99 14 9.5 14Z" fill=color/>
        </svg>
    }
}

#[component]
pub fn HomeIcon(
    #[prop(optional)] size: Option<i32>,
    #[prop(optional)] color: Option<&'static str>,
) -> impl IntoView {
    let size = size.unwrap_or(24);
    let color = color.unwrap_or("#F6F7F5");
    view! {
        <svg width=size height=size viewBox="0 0 24 24" fill="none">
            <path d="M10 20V14H14V20H19V12H22L12 3L2 12H5V20H10Z" fill=color/>
        </svg>
    }
}

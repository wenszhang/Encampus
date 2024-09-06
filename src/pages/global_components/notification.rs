use leptos::*;

#[derive(Clone, PartialEq)]
pub struct NotificationDetails {
    pub message: String,
    pub notification_type: NotificationType,
}

#[derive(Clone, PartialEq)]
pub enum NotificationType {
    // Success, // Commented out to pass linting checks due to not being used, let's add it back when we implement it
    Error,
    // Info,
    Warning,
}

#[component]
pub fn NotificationComponent(
    notification_details: NotificationDetails,
    on_close: impl Fn() + 'static,
) -> impl IntoView {
    let class_name = match notification_details.notification_type {
        // NotificationType::Success => "bg-green-500",
        NotificationType::Error => "bg-red-500",
        // NotificationType::Info => "bg-blue-500",
        NotificationType::Warning => "bg-yellow-500",
    };

    view! {
        <div class={format!("rounded p-4 text-white {}", class_name)}>
            <span>{&notification_details.message}</span>
            <button class="ml-4" on:click=move |_| on_close()>
                {"✖"}
            </button>
        </div>
    }
}

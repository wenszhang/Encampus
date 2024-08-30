use leptos::*;

#[derive(Clone, PartialEq)]
pub struct NotificationDetails {
    pub message: String,
    pub notification_type: NotificationType,
}

#[derive(Clone, PartialEq)]
pub enum NotificationType {
    Success,
    Error,
    Info,
    Warning,
}

#[component]
pub fn Notification(notification_details: NotificationDetails) -> impl IntoView {
    let class_name = match notification_details.notification_type {
        NotificationType::Success => "bg-green-500",
        NotificationType::Error => "bg-red-500",
        NotificationType::Info => "bg-blue-500",
        NotificationType::Warning => "bg-yellow-500",
    };

    view! {
        <div class={format!("rounded p-4 text-white {}", class_name)}>
            {&notification_details.message}
        </div>
    }
}
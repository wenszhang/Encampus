use crate::resources::images::svgs::cancel_icon::CancelIcon;
use crate::resources::images::svgs::error_icon::ErrorIcon;
use crate::resources::images::svgs::warning_icon::WarningIcon;

use leptos::*;

#[derive(Clone, PartialEq)]
pub struct NotificationDetails {
    pub message: String,
    pub notification_type: NotificationType,
}

#[derive(Clone, PartialEq)]
pub enum NotificationType {
    Error,
    // Info,
    Warning,
}

#[component]
pub fn NotificationComponent(
    notification_details: NotificationDetails,
    on_close: impl Fn() + 'static,
) -> impl IntoView {
    let notification_type = notification_details.notification_type.clone();

    let class_name = match notification_type {
        NotificationType::Error => {
            "bg-errorNotification-bg border border-errorNotification-details"
        }
        // NotificationType::Info => "bg-blue-500",
        NotificationType::Warning => {
            "bg-warningNotification-bg border border-warningNotification-details"
        }
    };

    let button_class = match notification_type {
        NotificationType::Warning => "ml-4 text-warningNotification-details",
        NotificationType::Error => "text-errorNotification-details",
    };

    let message_class = match notification_type {
        NotificationType::Warning => "text-warningNotification-details",
        NotificationType::Error => "text-errorNotification-details text-sm",
    };

    view! {
      <div class=format!("rounded p-4 text-white flex items-center {}", class_name)>
        <div class="pr-2">
          {move || match notification_details.notification_type {
            NotificationType::Error => view! { <ErrorIcon size="25px" /> }.into_view(),
            NotificationType::Warning => view! { <WarningIcon size="20px" /> }.into_view(),
          }}
        </div>
        <span class=message_class>{&notification_details.message} </span>
         // Space
         <span class="grow"> </span>
        <button class=button_class on:click=move |_| on_close()>
            <div class="top-1"> <CancelIcon size="1.3em"/> </div>
        </button>

      </div>
    }
}

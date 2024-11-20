// Requests and creates push notifications
use chrono::NaiveDateTime;
use js_sys::Reflect;
use leptos::{component, logging, view, IntoView, ServerFnError, SignalGetUntracked};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Notification, NotificationOptions, Window};

use crate::{
    data::{
        database::{
            announcement_functions::get_announcement_list, class_functions::get_users_classes,
        },
        global_state::User,
    },
    expect_logged_in_user,
};

// Stores the cutoff time for filtering announcements, initialized as None
static CUTOFF_TIME: Lazy<Mutex<Option<NaiveDateTime>>> = Lazy::new(|| Mutex::new(None));

/// Helper function to check if the user is authenticated, returning a Result instead of a view
fn get_authenticated_user() -> Result<User, JsValue> {
    let auth_context = crate::app::expect_auth_context();
    if auth_context.get_untracked().is_unauthenticated() {
        Err(JsValue::from_str("User not authenticated"))
    } else {
        Ok(auth_context.get_untracked().get_user().unwrap().clone())
    }
}

/// Set up notification permissions while logging errors
/// Should run on page load
pub async fn configure_notifications(window: &Window) -> Result<(), JsValue> {
    // Check if Notifications are supported
    let is_supported = move || {
        Reflect::has(&JsValue::from(window), &JsValue::from_str("Notification")).unwrap_or(false)
    };

    if !is_supported() {
        logging::log!("Notifications are not supported in this browser.");
        return Err(JsValue::from_str("Notifications not supported"));
    }

    // Request or check notification permissions
    let permission = match request_notification_permission().await {
        Ok(permission) => permission,
        Err(err) => {
            logging::log!("Failed to request notification permission: {:?}", err);
            return Err(err);
        }
    };

    match permission.as_str() {
        "granted" => {
            logging::log!("Notifications permission granted.");

            // Get authenticated user
            let user = match get_authenticated_user() {
                Ok(user) => user,
                Err(err) => {
                    logging::log!("User not authenticated. Redirecting to login...");
                    return Err(err);
                }
            };

            let user_id = user.id;
            let role = user.role.clone();

            if let Some((time, _, _)) = get_newest_announcement_for_user(user_id, role)
                .await
                .unwrap_or(None)
            {
                let mut cutoff_time = CUTOFF_TIME.lock().unwrap();
                *cutoff_time = Some(time);
            }
            Ok(())
        }
        "denied" => {
            logging::log!("Notifications permission denied by the user.");
            Err(JsValue::from_str("Notifications denied"))
        }
        _ => {
            logging::log!("Notifications permission is in default state.");
            Err(JsValue::from_str("Notifications permission not granted"))
        }
    }
}

/// Submits a push notification with a title and body
pub fn create_push_notification(title: &str, body: &str) -> Result<(), JsValue> {
    let options = NotificationOptions::new();
    options.set_body(body);

    match Notification::new_with_options(title, &options) {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

/// Helper for requesting permissions for notifications
async fn request_notification_permission() -> Result<String, JsValue> {
    let promise = Notification::request_permission().unwrap();
    let result = JsFuture::from(promise).await?;
    Ok(result.as_string().unwrap_or_else(|| "default".to_string()))
}

/// Function to get the newest announcement title and contents from all classes a user is enrolled in
pub async fn get_newest_announcement_for_user(
    user_id: i32,
    role: String,
) -> Result<Option<(NaiveDateTime, String, String)>, ServerFnError> {
    // Acquire and drop the lock before the await
    let cutoff_time_val = {
        let cutoff_time = CUTOFF_TIME.lock().unwrap();
        *cutoff_time
    };

    let classes = get_users_classes(user_id, role).await?;
    let mut latest_announcement: Option<(NaiveDateTime, String, String)> = None;

    for class in classes {
        let announcements =
            get_announcement_list(class.id)
                .await?
                .into_iter()
                .filter(|announcement| match cutoff_time_val {
                    Some(time) => announcement.time > time,
                    None => true,
                });

        for announcement in announcements {
            if latest_announcement.is_none()
                || announcement.time > latest_announcement.as_ref().unwrap().0
            {
                latest_announcement =
                    Some((announcement.time, announcement.title, announcement.contents));
            }
        }
    }

    // Update the cutoff time to the latest announcement time if available
    if let Some((time, _, _)) = latest_announcement {
        let mut cutoff_time = CUTOFF_TIME.lock().unwrap();
        *cutoff_time = Some(time);
    }

    Ok(latest_announcement)
}

/// Sends a push notification for the newest announcement for the user
pub async fn send_newest_announcement_notification() -> Result<(), JsValue> {
    // Get authenticated user
    let user = match get_authenticated_user() {
        Ok(user) => user,
        Err(err) => {
            logging::log!("User not authenticated. Unable to send announcement notification.");
            return Err(err);
        }
    };

    let user_id = user.id;
    let role = user.role.clone();

    // Get the newest announcement for the user
    if let Some((_, title, contents)) = get_newest_announcement_for_user(user_id, role)
        .await
        .unwrap_or(None)
    {
        // Create a push notification with the announcement details
        create_push_notification(&title, &contents)?;
    }

    Ok(())
}

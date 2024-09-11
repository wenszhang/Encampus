/**
 * Component for the login page where users can login to their account
 */
use leptos::{ev::SubmitEvent, *};

use crate::data::{database::user_functions::login, global_state::GlobalState};
use crate::pages::global_components::notification::{
    NotificationComponent, NotificationDetails, NotificationType,
};

#[component]
pub fn LoginPage() -> impl IntoView {
    let (username, set_username) = create_signal("".to_string());
    // let (password, set_password) = create_signal("".to_string());
    let (login_error, set_login_error) = create_signal(None::<NotificationDetails>);

    // Input event handler for controlled components
    let on_input = |setter: WriteSignal<String>| {
        move |ev| {
            setter(event_target_value(&ev));
        }
    };

    let login_action = create_action(|username: &String| {
        let username = username.to_owned();
        async {
            let user = login(username.clone()).await.unwrap_or_default();
            (username, user.id, user.firstname, user.role)
        }
    });

    create_effect(move |_| {
        let global_state = expect_context::<GlobalState>();
        if let Some(userInfo) = login_action.value()() {
            if userInfo.1 == 0 {
                set_login_error.set(Some(NotificationDetails {
                    message: "Failed Signing In, User doesn't exist.".to_string(),
                    notification_type: NotificationType::Error,
                }));
            } else {
                global_state.authenticated.set(true);
                global_state.user_name.set(Some(userInfo.0));
                global_state.id.set(Some(userInfo.1));
                global_state.first_name.set(Some(userInfo.2));
                global_state.role.set(Some(userInfo.3));

                // The variable definition is required
                // We might want to consider writing a short util that wraps navigate code to make it shorter, i.e. navigate_to("/classes")
                let navigate = leptos_router::use_navigate();
                match global_state.role.get().unwrap_or_default().as_str() {
                    "student" => navigate("/classes", Default::default()),
                    "instructor" => navigate("/classes", Default::default()), // Change to instructor page when implemented
                    "admin" => navigate("/classes", Default::default()), // Change to admin page when implemented
                    _ => navigate("/login", Default::default()),
                }
            }
        }
    });

    // Form submission handler
    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();
        login_action.dispatch(username());
    };

    let notification_view = move || {
        login_error.get().map(|details| {
            view! {
                <NotificationComponent
                    notification_details={details.clone()}
                    on_close={move || set_login_error(None)}
                />
            }
        })
    };

    view! {
        <form on:submit=on_submit>
            <div class="flex flex-col justify-center items-center h-screen">
                <div class="bg-white p-20 rounded-lg shadow-md w-96">
                    <div class="flex justify-center items-center">
                        <img src={format!("/{}", "images/logo.png")} alt="Logo" class="h-16"/>
                    </div>
                    <h1 class="text-2xl font-semibold text-center mb-4">
                        Login
                    </h1>
                    {notification_view}
                    <div class="mb-4">
                        <label for="username" class="block text-gray-700 font-bold mb-2">
                            Username:
                        </label>
                        <input
                            type="text"
                            id="username"
                            placeholder="Enter your Username"
                            required
                            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:border-blue-500"
                            on:input=on_input(set_username)
                            prop:value=username
                        />
                    </div>
                    // <div class="mb-4 opacity-50">
                    //     <label for="password" class="block text-gray-700 font-bold mb-2">
                    //         Password:
                    //     </label >
                    //     <input
                    //         type="password"
                    //         id="password"
                    //         placeholder="Enter your Password"
                    //         required
                    //         class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:border-indigo-500 bg-gray-200 text-gray-500 cursor-not-allowed"
                    //         on:input=on_input(set_password)
                    //         prop:value=password
                    //         disabled
                    //     />
                    // </div>
                    <button
                        type="submit"
                        class="w-full bg-blue-500 hover:bg-blue-600 text-white py-2 px-4 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
                    >
                        Submit
                    </button>
                    <div class="mt-4 text-sm text-gray-600 text-center">
                        Please enter your username.
                    </div>
                    <div class="mt-1 text-sm text-gray-600 text-center">
                        Click here to <a href="/register" class="text-blue-500">register</a>.
                    </div>
                </div>
            </div>
        </form>
    }
}

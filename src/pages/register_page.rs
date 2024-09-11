use crate::data::database::user_functions::add_user;
use crate::data::database::user_functions::User;
use crate::data::global_state::GlobalState;
use crate::pages::global_components::notification::{
    NotificationComponent, NotificationDetails, NotificationType,
};
/**
 * Component for the login page where users can login to their account
 */
use leptos::{ev::SubmitEvent, *};

#[component]
pub fn RegisterPage() -> impl IntoView {
    let (username, set_username) = create_signal("".to_string());
    let (first_name, set_first_name) = create_signal("".to_string());
    let (last_name, set_last_name) = create_signal("".to_string());
    let (user_id, set_user_id) = create_signal(0);
    let (login_error, set_login_error) = create_signal(None::<NotificationDetails>);
    // let (role, set_role) = create_signal("".to_string());

    let on_input = |setter: WriteSignal<String>| {
        move |ev| {
            setter(event_target_value(&ev));
        }
    };

    let new_user = User {
        username: username.get(),
        firstname: first_name.get(),
        lastname: last_name.get(),
        role: "student".to_string(),
        id: 0,
    };

    let new_user_action = create_action(move |new_user: &User| {
        let new_user = new_user.clone();
        async move {
            match add_user(new_user).await {
                Ok(id) => {
                    set_user_id(id);
                }
                Err(_) => {
                    set_login_error(Some(NotificationDetails {
                        message: "Failed adding user, username already exists.".to_string(),
                        notification_type: NotificationType::Error,
                    }));
                }
            }
        }
    });

    create_effect(move |_| {
        let global_state = expect_context::<GlobalState>();
        if let Some(_id) = new_user_action.value()() {
            if user_id.get() > 0 {
                global_state.authenticated.set(true);
                global_state.user_name.set(Some(username.get()));
                global_state.id.set(Some(user_id.get()));
                global_state.first_name.set(Some(first_name.get()));
                global_state.role.set(Some("student".to_string()));

                let navigate = leptos_router::use_navigate();
                match global_state.role.get().unwrap_or_default().as_str() {
                    "student" => navigate("/classes", Default::default()),
                    "teacher" => navigate("/classes", Default::default()), // Change to instructor page when implemented
                    "admin" => navigate("/classes", Default::default()), // Change to admin page when implemented
                    _ => navigate("/login", Default::default()),
                }
            }
        }
    });

    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();
        new_user_action.dispatch(new_user.clone());
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
                <h1 class="text-2xl font-semibold text-center mb-2">
                    Create Account
                </h1>
                <div class="mt-4 text-sm text-gray-600 text-center mb-4">
                    Please fill out the following information to create an account.
                </div>
                {notification_view}
                <div class="mb-4">
                    <label for="username" class="flex row-auto text-gray-700 font-bold mb-2">
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
                <div class="mb-4">
                    <label for="username" class="flex row-auto text-gray-700 font-bold mb-2">
                        First Name:
                    </label>
                    <input
                        type="text"
                        id="first_name"
                        placeholder="First Name"
                        required
                        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:border-blue-500"
                        on:input=on_input(set_first_name)
                        prop:value=first_name
                    />
                </div>
                <div class="mb-4">
                    <label for="username" class="flex row-auto text-gray-700 font-bold mb-2">
                        Last Name:
                    </label>
                    <input
                        type="text"
                        id="last_name"
                        placeholder="Last Name"
                        required
                        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:border-blue-500"
                        on:input=on_input(set_last_name)
                        prop:value=last_name
                    />
                </div>
                // <div class="mb-5">
                //     <label for="role" class="flex row-auto text-gray-700 font-bold mb-2">
                //         Role:
                //     </label>
                //     <select
                //         id="role"
                //         required
                //         class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:border-blue-500"
                //         on:input=on_input(set_role)
                //         prop:value=role
                //     >
                //         <option value="student">Student</option>
                //         <option value="teacher">Teacher</option>
                //     </select>
                // </div>
                <button
                    type="submit"
                    class="w-full bg-blue-500 hover:bg-blue-600 text-white py-2 px-4 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
                >
                    Register
                </button>
                <div class="mt-4 text-sm text-gray-600 text-center">
                    Already have an account? <a href="/login" class="text-blue-500">Login</a>
                    </div>
            </div>
        </div>
    </form>
    }
}

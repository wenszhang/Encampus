/**
 * Component for the login page where users can login to their account
 */
use leptos::{ev::SubmitEvent, *};

use crate::data::database::user_functions::get_user_info;
use crate::data::database::user_functions::User;
use crate::{data::database::security_functions::login_signup, data::global_state::GlobalState};

#[component]
pub fn LoginPage() -> impl IntoView {
    let (username, set_username) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());
    let (first_name, set_first_name) = create_signal("".to_string());

    // Input event handler for controlled components
    let on_input = |setter: WriteSignal<String>| {
        move |ev| {
            setter(event_target_value(&ev));
        }
    };
    let global_state = expect_context::<GlobalState>();

    // Form submission handler
    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();

        let _login = create_resource(username, |username| async {
            login_signup(username).await.unwrap_or_default();
        });

        let user_name = username.clone();
        let user = create_resource(user_name, |user_name| async {
            get_user_info(user_name).await.unwrap_or_else(|_| User {
                username: "Failed".to_string(),
                firstname: "Failed".to_string(),
                lastname: "Failed".to_string(),
                id: 0,
            })
        });

        global_state.user_name.set(Some(username.get()));
        global_state.authenticated.set(true);
        global_state
            .first_name
            .set(Some(user.get().unwrap_or_default().firstname));

        // let user = create_resource(username, |username| async {
        //     get_user_info(username).await.unwrap().first_name
        // });

        // global_state.first_name.set(Some(user.get().unwrap()));

        // The variable definition is required
        // We might want to consider writing a short util that wraps navigate code to make it shorter, i.e. navigate_to("/classes")
        let navigate = leptos_router::use_navigate();
        navigate("/classes", Default::default());
    };

    view! {
        <form on:submit=on_submit>
            <div class="flex flex-col justify-center items-center h-screen">
                <div class="bg-white p-20 rounded-lg shadow-md">
                    <div class="flex justify-center items-center">
                        <img src={format!("/{}", "images/logo.png")} alt="Logo" class="h-16"/>
                    </div>
                    <h1 class="text-2xl font-semibold text-center mb-4">
                        Login
                    </h1>
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
                    <div class="mb-4 opacity-50">
                        <label for="password" class="block text-gray-700 font-bold mb-2">
                            Password:
                        </label >
                        <input
                            type="password"
                            id="password"
                            placeholder="Enter your Password"
                            required
                            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:border-indigo-500 bg-gray-200 text-gray-500 cursor-not-allowed"
                            on:input=on_input(set_password)
                            prop:value=password
                            disabled
                        />
                    </div>
                    <button
                        type="submit"
                        class="w-full bg-blue-500 hover:bg-blue-600 text-white py-2 px-4 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
                    >
                        Submit
                    </button>
                    <div class="mt-4 text-sm text-gray-600 text-center">
                        Please enter your username.
                        </div>
                </div>
            </div>
        </form>
    }
}

#[cfg(feature = "ssr")]
#[derive(sqlx::FromRow)]
pub struct Name(String);

#[server(SetUserFirstName)]
pub async fn set_user_first_name(username: String) -> Result<String, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let Name(name) = sqlx::query_as("select firstname from users where username = '$1'")
        .bind(username)
        .fetch_one(&pool)
        .await
        .expect("failed getting user");

    Ok(name)
}

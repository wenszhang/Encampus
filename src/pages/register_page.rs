/**
 * Component for the login page where users can login to their account
 */
use leptos::{ev::SubmitEvent, *};

use crate::data::{database::user_functions::login_signup, global_state::GlobalState};

#[component]
pub fn RegisterPage() -> impl IntoView {
    let (username, set_username) = create_signal("".to_string());

    let on_input = |setter: WriteSignal<String>| {
        move |ev| {
            setter(event_target_value(&ev));
        }
    };

    view! {
        <form>
        <div class="flex flex-col justify-center items-center h-screen">
            <div class="bg-white p-20 rounded-lg shadow-md">
                <div class="flex justify-center items-center">
                    <img src={format!("/{}", "images/logo.png")} alt="Logo" class="h-16"/>
                </div>
                <h1 class="text-2xl font-semibold text-center mb-2">
                    Create Account
                </h1>
                <div class="mt-4 text-sm text-gray-600 text-center mb-4">
                    Please fill out the following information to create an account.
                    </div>
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
                        on:input=on_input(set_username)
                        prop:value=username
                    />
                </div>
                <div class="mb-4">
                    <label for="username" class="flex row-auto text-gray-700 font-bold mb-2">
                        Last Name:
                    </label>
                    <input
                        type="text"
                        id="username"
                        placeholder="Last Name"
                        required
                        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:border-blue-500"
                        on:input=on_input(set_username)
                        prop:value=username
                    />
                </div>
                <div class="mb-5">
                    <label for="role" class="flex row-auto text-gray-700 font-bold mb-2">
                        Role:
                    </label>
                    <select
                        id="role"
                        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:border-blue-500"
                    >
                        <option value="student">Student</option>
                        <option value="teacher">Teacher</option>
                    </select>
                </div>
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

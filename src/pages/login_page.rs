use leptos::{component, create_action, view, Input, IntoView};

use crate::database_functions::add_student;

#[component]
pub fn LoginPage() -> impl IntoView {
    let add_student = create_action(|input: &String| {
        // the input is a reference, but we need the Future to own it
        // this is important: we need to clone and move into the Future
        // so it has a 'static lifetime
        let input = input.to_owned();
        async move { add_student(input).await }
    });

    let input_ref = create_node_ref::<Input>();
    view! {
        <form
            on:submit=move |ev|{
                ev.prevent_default(); // don't reload the page...
                let input = input_ref.get().expect("input to exist");
                add_student.dispatch(input.value());
            }>
            <div class="flex flex-col justify-center items-center h-screen">
            <div class="bg-white p-20 rounded-lg shadow-md text-center">
                <img src={format!("/{}", "logo.png".to_string())} alt="Logo" class="h-8 mr-2"/>
                <h1 class="text-2xl font-semibold mb-4">Login</h1>
                <div class="mb-4">
                        <label for="username" class="block text-gray-700 font-bold mb-2">
                            Username:
                        </label>
                        <input type="text" id="username" placeholder="Enter your Username" required class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:border-indigo-500" />
                        //<ActionForm action=login_username>

                    </div>
                    <div class="mb-4">
                        <label for="password" class="block text-gray-300 font-bold mb-2">
                            Password:
                        </label>
                        <input type="password" id="password" placeholder="Enter your Password" required class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:border-indigo-500 disabled:bg-gray-100 disabled:text-gray-500 disabled:border-gray-200"
                        disabled
                        style="opacity: 0.4;" // assists with making password appear inactive.
                        />
                        // class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:border-indigo-500" /> active password tailwind code.
                    </div>
                    <div>
                        <a href=&format!("classes")>
                            <button type="submit" class="w-full bg-blue-500 hover:bg-blue-600 text-white py-2 px-4 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">Submit</button>
                        </a>
                    </div>

                     <div class="mt-4 text-sm text-gray-600 text-center">
                       Please enter your username.
                     </div>
                    // <div class="mt-4 text-sm text-gray-600 text-center">
                    //     Dont have an account? <a href="#" class="text-blue-500 hover:underline">Sign up here</a>
                    // </div>
                </div>
            </div>
        </form>
        <div>
        </div>
    }
}

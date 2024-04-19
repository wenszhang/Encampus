use leptos::{component, view, IntoView};

#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <form>
            <div class="flex flex-col justify-center items-center h-screen">
                <div class="bg-white p-20 rounded-lg shadow-md">
                <div class="text-center"> LOGO HERE </div>
                    <h1 class="text-2xl font-semibold text-center mb-4">Login</h1>
                    <div class="mb-4">
                        <label for="username" class="block text-gray-700 font-bold mb-2">
                            Username:
                        </label>
                        <input type="text" id="username" placeholder="Enter your Username" required class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:border-indigo-500" />
                    </div>
                    <div class="mb-4">
                        <label for="password" class="block text-gray-700 font-bold mb-2">
                            Password:
                        </label>
                        <input type="password" id="password" placeholder="Enter your Password" required class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:border-indigo-500" />
                    </div>
                    <div>
                        <a href=&format!("classes")>
                            <button type="submit" class="w-full bg-blue-500 hover:bg-blue-600 text-white py-2 px-4 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">Submit</button>
                        </a>
                    </div>
                    <div class="mt-4 text-sm text-gray-600 text-center">
                        Dont have an account? <a href="#" class="text-blue-500 hover:underline">Sign up here</a>
                    </div>
                </div>
            </div>
        </form>
        <div>
        </div>
    }
}

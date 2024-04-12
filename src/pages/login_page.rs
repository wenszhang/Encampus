use leptos::{component, view, IntoView};

#[component]

pub fn LoginPage() -> impl IntoView {
    view! {
        <form>
            <div class="text-center flex flex-col gap-5 justify-center items-center h-screen">
            <h1 class=""> Encampus </h1>
                <div class="mb-4">
                    <label for="username" class="block text-gray-700 font-bold" >
                        Username:
                    </label>
                    <input type= "text" id="username" placeholder="Enter your Username" required class="px-3 border border-x-gray-300 border-solid rounded-md focus:outline-none" />
                </div>
                <div>
                    <label class="text-gray-700 font-bold" for="password">
                        Password:
                    </label>
                    <input type="password" id="password" placeholder="Enter your Password" required class=" px-3 border border-x-gray-300 border-solid rounded-md focus:outline-none" />
                </div>
                <div>
                    <button type="submit" class= "bg-gradient-to-r from-purple-600 to-blue-500 text-white py-3 px-20 rounded-full duration-300 ease-in-out hover:from-blue-500 hover:to-purple-600" > Submit </button>
                </div>
            </div>
        </form>
    }
}

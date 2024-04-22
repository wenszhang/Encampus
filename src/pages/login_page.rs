use leptos::{
    component, create_node_ref, create_signal, ev::SubmitEvent, html::Input, view, IntoView,
    NodeRef,
};

#[component]
pub fn LoginPage() -> impl IntoView {
    let (username, set_name) = create_signal("".to_string());
    let input_element: NodeRef<Input> = create_node_ref();
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let username_val = input_element().expect("username input not found").value();
        set_name(username_val);
    };
    view! {
        <form onsubmit="return false;">
            <div class="flex flex-col justify-center items-center h-screen">
                <div class="bg-white p-20 rounded-lg shadow-md">
                <div class="text-center"> LOGO HERE </div>
                    <h1 class="text-2xl font-semibold text-center mb-4">Login</h1>
                    <div class="mb-4">
                        <label for="username" class="block text-gray-700 font-bold mb-2">
                            Username:
                        </label>
                        <input type="text"
                            //on:input=move |e|{ set_name(event_target_value(&e))}
                            id="username" placeholder="Enter your Username" required class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:border-indigo-500" />
                    </div>
                    <div class="mb-4">
                        <label for="password" class="block text-gray-300 font-bold mb-2">
                            Password:
                        </label>
                        <input type="password" id="password" placeholder="Enter your Password" required class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:border-indigo-500 disabled:bg-gray-100 disabled:text-gray-500 disabled:border-gray-200"
                        disabled
                        style="opacity: 0.4;" // assists with making password apear inactive.
                        />
                        // class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:border-indigo-500" /> active password tailwind code.
                    </div>
                    <div>
                        <a href="/classes" >

                        <form on:submit=on_submit>
                            <input type="text"
                            username_val=username
                            node_ref=input_element
                            //class="w-full bg-blue-500 hover:bg-blue-600 text-white py-2 px-4 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">
                            />
                            <button type="submit" value="Submit" class="w-full bg-blue-500 hover:bg-blue-600 text-white py-2 px-4 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">Submit</button>
                        </form>

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

        <script>
            document.getElementById("username").addEventListener("input", function() {
                var usernameInput = this.value.trim();
                var submitLink = document.getElementById("submitLink");

                if(usernameInput.value != ""){
                    submitLink.classList.remove("pointer-events-none");
                    submitLink.classList.remove("opacity-50");
                } else {
                    // If username is empty, disable the link
                    submitLink.classList.add("pointer-events-none");
                    submitLink.classList.add("opacity-50");
                }
            })
        </script>
    }
}

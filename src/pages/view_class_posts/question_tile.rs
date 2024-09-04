/**
 * QuestionTile component, displaying a tile for one post
 */
use crate::data::database::post_functions::Post;
use leptos::*;
use leptos_router::A;

#[component]
pub fn QuestionTile(post: Post) -> impl IntoView {
    view! {
        <A href=format!("{}", post.post_id)>
            <div class="relative bg-card-bg rounded-lg shadow-lg p-6 flex flex-col items-center justify-between text-lg font-semibold h-60 w-85 transition-transform duration-300 hover:scale-105 hover:bg-gray-100 hover:shadow-xl overflow-hidden">

                <div class="text-xs absolute top-0 left-0 w-full h-12 bg-card-header rounded-t-lg shadow-md z-10 flex items-center">
                    //<div class="text-sm font-medium text-gray-700">Name{}</div> // Get the students name here?
                    <span class="px-2 py-1 bg-gray-200 text-gray-600 rounded-full mr-2">Example tag</span>
                    <span class="px-2 py-1 bg-gray-200 text-gray-600 rounded-full mr-2">Tag1</span>
                    <span class="px-2 py-1 bg-gray-200 text-gray-600 rounded-full">Tag2</span>
                </div>

                <div class="flex-grow flex items-center justify-center mt-6">
                    <p class="text-center">{post.title}</p>
                </div>
            </div>
        </A>
    }
}
// Old style for reference
// #[component]
// pub fn QuestionTile(post: Post) -> impl IntoView {
//     view! {
//         <A href=format!("{}", post.post_id)>
//         <div class="relative bg-white rounded-lg shadow-lg p-6 flex flex-col items-center justify-between text-lg font-semibold h-60 w-85 transition-transform duration-300 hover:scale-105 hover:bg-gray-100 hover:shadow-xl overflow-hidden ">
//             <div class="absolute top-4 left-6 flex flex-col items-start space-y-2 z-10">
//                 <div class="text-sm font-medium text-gray-700">{}</div> // Get the students name here?
//                 <div class="flex space-x-2">
//                     <p class="text-xs bg-gray-200 text-gray-600 px-2 py-1 rounded-full">Example tag</p>
//                     <span class="text-xs bg-gray-200 text-gray-600 px-2 py-1 rounded-full">Tag1</span>
//                     <span class="text-xs bg-gray-200 text-gray-600 px-2 py-1 rounded-full">Tag2</span>
//                 </div>
//             </div>
//             <div class="w-[calc(100%+3rem)] border-t-2 border-gray-200 mt-8 -mx-6" style="box-shadow: 0 2px 4px rgba(0, 0, 0, 0.15);"></div> // Use vanilla inline css
//             <div class="flex-grow flex items-center justify-center">
//                 {post.title}
//             </div>
//         </div>
//     </A>
//     }
// }

#[component]
pub fn UnansweredQuestionTile(post: Post) -> impl IntoView {
    view! {
        <A href=format!("{}", post.post_id)>
            <div class="relative bg-customRed rounded-lg shadow-lg p-6 flex flex-col items-center justify-between text-lg font-semibold h-60 w-85 transition-transform duration-300 hover:scale-105 hover:bg-customRed-HOVER hover:shadow-xl overflow-hidden">
                <div class="absolute top-4 left-6 flex flex-col items-start space-y-2 z-10">
                    <div class="text-sm font-medium text-gray-700">{}</div> // Get the students name here?
                    <div class="flex space-x-2">
                        <p class="text-xs bg-gray-200 text-gray-600 px-2 py-1 rounded-full">Example tag</p>
                        <span class="text-xs bg-gray-200 text-gray-600 px-2 py-1 rounded-full">Tag1</span>
                        <span class="text-xs bg-red-500 hover:bg-red-700 text-white px-2 py-1 rounded-full">Unresolved</span>
                    </div>
                </div>
                <div class="w-[calc(100%+3rem)] border-t-2 border-gray-200 mt-8 -mx-6" style="box-shadow: 0 2px 4px rgba(0, 0, 0, 0.15);"></div> // Use vanilla inline css
                <div class="flex-grow flex items-center justify-center">
                    {post.title}
                </div>
            </div>
        </A>
    }
}

#[component]
pub fn PrivateQuestionTile(post: Post) -> impl IntoView {
    view! {
        <A href=format!("{}", post.post_id)>
            <div class="relative rounded-lg shadow-lg p-6 flex flex-col items-center justify-between text-lg font-semibold h-60 w-85 transition-transform duration-300 hover:scale-105 hover:shadow-xl overflow-hidden">
                <div class="absolute top-4 left-6 flex flex-col items-start space-y-2 z-10">
                    <div class="text-sm font-medium text-gray-700">{}</div> // Get the students name here?
                    <div class="flex space-x-2">
                        <p class="text-xs bg-gray-200 text-gray-600 px-2 py-1 rounded-full">Example tag</p>
                        <span class="text-xs bg-gray-200 text-gray-600 px-2 py-1 rounded-full">Tag1</span>
                        <span class="text-xs bg-customPurple text-white px-2 py-1 rounded-full">Private</span>
                    </div>
                </div>
                <div class="w-[calc(100%+3rem)] border-t-2 border-gray-200 mt-8 -mx-6" style="box-shadow: 0 2px 4px rgba(0, 0, 0, 0.15);"></div> // Use vanilla inline css
                <div class="flex-grow flex items-center justify-center">
                    {post.title}
                </div>
            </div>
        </A>
    }
}

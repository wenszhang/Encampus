/**
 * create post icon component, used in header next to the label "create new post" when creating a new post.
 */
use leptos::{component, view, IntoView};

#[component]
pub fn CreatePostIcon(
    /// css style size (E.g. sized="20px")
    /// (Note: Currently only supports static strings.
    ///  If you require dynamic sizing edit this component to take in a closure instead.)
    size: &'static str,
) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" width=size height=size viewBox="0 0 24 24"><path fill="currentColor" d="M21 14v5a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5v2H5v14h14v-5z"/>
        <path fill="currentColor" d="M21 7h-4V3h-2v4h-4v2h4v4h2V9h4z"/>
        </svg>
    }
}

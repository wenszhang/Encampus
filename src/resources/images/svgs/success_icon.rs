/**
 * Success icon component, used create post and the notifications.
 */
use leptos::{component, view, IntoView};

#[component]
pub fn SuccessIcon(
    /// css style size (E.g. sized="20px")
    /// (Note: Currently only supports static strings.
    ///  If you require dynamic sizing edit this component to take in a closure instead.)
    size: &'static str,
) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" width=size height=size viewBox="0 0 48 48"><defs><mask id="ipTSuccess0">
        <g fill="none" stroke="#fff" stroke-linecap="round" stroke-linejoin="round" stroke-width="4"><path fill="#555555" d="m24 4l5.253 3.832l6.503-.012l1.997 6.188l5.268 3.812L41 24l2.021 6.18l-5.268 3.812l-1.997 6.188l-6.503-.012L24 44l-5.253-3.832l-6.503.012l-1.997-6.188l-5.268-3.812L7 24l-2.021-6.18l5.268-3.812l1.997-6.188l6.503.012z"/>
        <path d="m17 24l5 5l10-10"/></g></mask></defs><path fill="#5D993E" d="M0 0h48v48H0z" mask="url(#ipTSuccess0)"/></svg>
    }
}

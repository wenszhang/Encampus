/**
 * instructor endoresed icon component, used in instuctor endorsed pill inside on a question tile.
 */
use leptos::{component, view, IntoView};

#[component]
pub fn InstructorEndorsedIcon(
    /// css style size (E.g. sized="20px")
    /// (Note: Currently only supports static strings.
    ///  If you require dynamic sizing edit this component to take in a closure instead.)
    size: &'static str,
) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" width=size height=size viewBox="0 0 21 21">
            <g fill="none" fill-rule="evenodd" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5">
                <path d="M14.857 3.79a8 8 0 1 0 2.852 3.24"/>
                <path d="m6.5 9.5l3 3l8-8"/>
            </g>
        </svg>
    }
}

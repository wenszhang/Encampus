/**
 * Strikethrough icon component, used in the rich text editor
 */
use leptos::{component, view, IntoView};

#[component]
pub fn StrikethroughIcon(
    /// css style size (E.g. sized="20px")
    /// (Note: Currently only supports static strings.
    ///  If you require dynamic sizing edit this component to take in a closure instead.)
    size: &'static str,
) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" width=size height=size viewBox="0 0 16 16">
        <path fill="currentColor" d="M10.023 10h1.274q.01.12.01.25a2.56 2.56 0 0 1-.883 1.949q-.426.375-1.03.588A4.1 4.1 0 0 1 8.028 13a4.62 4.62 0 0 1-3.382-1.426q-.29-.388 0-.724q.29-.334.735-.13q.515.544 1.213.876q.699.33 1.449.33q.956 0 1.485-.433q.53-.435.53-1.14a1.7 1.7 0 0 0-.034-.353M5.586 7a2.5 2.5 0 0 1-.294-.507a2.3 2.3 0 0 1-.177-.934q0-.544.228-1.015t.633-.816t.955-.537A3.7 3.7 0 0 1 8.145 3q.867 0 1.603.33q.735.332 1.25.861q.24.423 0 .692q-.24.27-.662.102a3.4 3.4 0 0 0-.978-.669a2.9 2.9 0 0 0-1.213-.242q-.81 0-1.302.375t-.492 1.036q0 .354.14.596q.138.243.374.426q.236.184.515.324q.179.09.362.169zM2.5 8h11a.5.5 0 1 1 0 1h-11a.5.5 0 0 1 0-1"/>
        </svg>
    }
}

use leptos::{component, view, IntoView};
use leptos_meta::Stylesheet;

#[component]
pub fn TestPage() -> impl IntoView {
    
    view!{
        <Stylesheet id="testpage" href="/pkg/encampus.css"/>

        <div class="header">
            <div class="logo">"ENCAMPUS"</div>
            <div class="search">
                <input type="text" placeholder="Ask me anything..."/>
            </div>
            <div class="user">
                <span>"LONGNAME"</span>
                <div class="menu">"☰"</div>
            </div>
        </div>
        <div class="tiles">
            <div class="tile">"Math 3210"</div>
            <div class="tile">"Class 3124"</div>
            <div class="tile">"Class 4123"</div>
            <div class="tile">"Class 3214"</div>
            <div class="tile">"Class 1243"</div>
            <div class="tile">"Class 2341"</div>
        </div>
    }
}

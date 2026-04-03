use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="section header">
            <div class="section-border top-border">
                <span class="corner tl">"\u{2554}"</span>
                <span class="line"></span>
                <span class="corner tr">"\u{2557}"</span>
            </div>
            <div class="section-content header-content">
                <h1>
                    <span class="pipe">"\u{2551}"</span>
                    <span class="title">"Austin Sievert"</span>
                    <span class="pipe">"\u{2551}"</span>
                </h1>
            </div>
            <div class="section-border mid-border">
                <span class="corner tl">"\u{2560}"</span>
                <span class="line"></span>
                <span class="corner tr">"\u{2563}"</span>
            </div>
        </header>
    }
}

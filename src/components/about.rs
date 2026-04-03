use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section class="section about">
            <div class="section-content">
                <div class="section-label">
                    <span class="pipe">"\u{2551}"</span>
                    <span class="label-text">"GENERAL SPECIFICATIONS"</span>
                    <span class="pipe">"\u{2551}"</span>
                </div>
                <div class="spec-grid">
                    <div class="spec-row">
                        <span class="pipe">"\u{2551}"</span>
                        <span class="spec-key">"PRIMARY LANG"</span>
                        <span class="spec-val">"C, Rust"</span>
                        <span class="pipe">"\u{2551}"</span>
                    </div>
                    <div class="spec-row">
                        <span class="pipe">"\u{2551}"</span>
                        <span class="spec-key">"DOMAIN"</span>
                        <span class="spec-val">"Embedded Systems, IoT, Wireless Communications, Systems Engineering"</span>
                        <span class="pipe">"\u{2551}"</span>
                    </div>
                    <div class="spec-row">
                        <span class="pipe">"\u{2551}"</span>
                        <span class="spec-key">"EDITOR"</span>
                        <span class="spec-val">"Emacs"</span>
                        <span class="pipe">"\u{2551}"</span>
                    </div>
                </div>
            </div>
            <div class="section-border mid-border">
                <span class="corner tl">"\u{2560}"</span>
                <span class="line"></span>
                <span class="corner tr">"\u{2563}"</span>
            </div>
        </section>
    }
}

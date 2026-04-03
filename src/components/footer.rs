use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="section footer">
            <div class="section-content footer-content">
                <div class="footer-row">
                    <span class="pipe">"\u{2551}"</span>
                    <div class="footer-inner">
                        <a href="https://github.com/Arsievert" target="_blank" rel="noopener" class="footer-link">
                            "[ GitHub ]"
                        </a>
                    </div>
                    <span class="pipe">"\u{2551}"</span>
                </div>
                <div class="footer-row">
                    <span class="pipe">"\u{2551}"</span>
                    <span class="footer-meta">"REV 1.0 \u{2014} BUILT WITH RUST + WASM + CLAUDE"</span>
                    <span class="pipe">"\u{2551}"</span>
                </div>
            </div>
            <div class="section-border bottom-border">
                <span class="corner tl">"\u{255a}"</span>
                <span class="line"></span>
                <span class="corner tr">"\u{255d}"</span>
            </div>
        </footer>
    }
}

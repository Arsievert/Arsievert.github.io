use leptos::prelude::*;

use crate::components::about::About;
use crate::components::demo::Demo;
use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::projects::Projects;
use crate::components::theme_toggle::ThemeToggle;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <ThemeToggle/>
        <div class="blueprint">
            <div class="container">
                <Header/>
                <About/>
                <Projects/>
                <Demo/>
                <Footer/>
            </div>
        </div>
    }
}

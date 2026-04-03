use leptos::prelude::*;

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let (is_light, set_is_light) = signal(false);

    let toggle = move |_| {
        let new_val = !is_light.get();
        set_is_light.set(new_val);
        if let Some(doc) = document().document_element() {
            if new_val {
                let _ = doc.class_list().add_1("light");
            } else {
                let _ = doc.class_list().remove_1("light");
            }
        }
    };

    let label = move || if is_light.get() { "[ DARK ]" } else { "[ LIGHT ]" };

    view! {
        <button class="theme-toggle" on:click=toggle>
            {label}
        </button>
    }
}

use leptos::prelude::*;

struct Project {
    name: &'static str,
    description: &'static str,
    highlights: &'static [&'static str],
    language: &'static str,
    license: &'static str,
    url: &'static str,
}

const PROJECTS: &[Project] = &[
    Project {
        name: "cfgpack",
        description: "MessagePack-based configuration library for embedded systems",
        highlights: &[
            "Zero heap allocation",
            "195+ unit tests",
            "Schema versioning & migration",
        ],
        language: "C",
        license: "MIT",
        url: "https://github.com/Arsievert/cfgpack",
    },
    Project {
        name: ".emacs.d",
        description: "Personal Emacs configuration",
        highlights: &[
            "Custom Lisp modules",
        ],
        language: "Emacs Lisp",
        license: "GPL-3.0",
        url: "https://github.com/Arsievert/.emacs.d",
    },
    Project {
        name: "messagepack_to_json",
        description: "Bidirectional MessagePack / JSON format converter",
        highlights: &[
            "Streaming conversion",
        ],
        language: "Rust",
        license: "MIT",
        url: "https://github.com/Arsievert/messagepack_to_json",
    },
];

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <section class="section projects">
            <div class="section-content">
                <div class="section-label">
                    <span class="pipe">"\u{2551}"</span>
                    <span class="label-text">"REPOSITORIES"</span>
                    <span class="pipe">"\u{2551}"</span>
                </div>
                <div class="project-list">
                    {PROJECTS.iter().map(|p| {
                        view! {
                            <div class="project-card">
                                <span class="pipe">"\u{2551}"</span>
                                <div class="project-inner">
                                    <div class="project-header">
                                        <a href={p.url} target="_blank" rel="noopener" class="project-name">
                                            {format!("\u{251c}\u{2500} {}", p.name)}
                                        </a>
                                        <span class="project-meta">
                                            {format!("{} | {}", p.language, p.license)}
                                        </span>
                                    </div>
                                    <div class="project-desc">{p.description}</div>
                                    <div class="project-highlights">
                                        {p.highlights.iter().map(|h| {
                                            view! {
                                                <span class="highlight">{format!("[{}]", h)}</span>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>
                                <span class="pipe">"\u{2551}"</span>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
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

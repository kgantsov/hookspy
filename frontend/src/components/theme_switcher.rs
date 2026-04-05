use web_sys::window;
use yew::prelude::*;

struct ThemeOption {
    id: &'static str,
    color: &'static str,
    label: &'static str,
}

const THEMES: &[ThemeOption] = &[
    ThemeOption {
        id: "gray",
        color: "#2f81f7",
        label: "Gray Dark (default)",
    },
    ThemeOption {
        id: "gray-light",
        color: "#f6f8fa",
        label: "Gray Light",
    },
    ThemeOption {
        id: "indigo",
        color: "#6366f1",
        label: "Indigo",
    },
    ThemeOption {
        id: "sky",
        color: "#0ea5e9",
        label: "Electric Sky",
    },
    ThemeOption {
        id: "teal",
        color: "#14b8a6",
        label: "Seafoam / Teal",
    },
    ThemeOption {
        id: "lime",
        color: "#84cc16",
        label: "Lime / Hacker",
    },
    ThemeOption {
        id: "blue",
        color: "#58a6ff",
        label: "Soft Blue",
    },
    ThemeOption {
        id: "amber",
        color: "#f59e0b",
        label: "Amber / Honey",
    },
];

fn apply_theme(theme_id: &str) {
    if let Some(win) = window() {
        // Set data-theme on <html>
        if let Some(doc) = win.document() {
            if let Some(root) = doc.document_element() {
                // "gray" is the default — remove the attribute so :root rules apply
                if theme_id == "gray" {
                    let _ = root.remove_attribute("data-theme");
                } else {
                    let _ = root.set_attribute("data-theme", theme_id);
                }
            }
        }
        // Persist to localStorage
        if let Ok(Some(storage)) = win.local_storage() {
            let _ = storage.set_item("hookspy-theme", theme_id);
        }
    }
}

fn read_saved_theme() -> String {
    window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|s| s.get_item("hookspy-theme").ok().flatten())
        .unwrap_or_else(|| "gray".to_string())
}

#[component]
pub fn ThemeSwitcher() -> Html {
    let current = use_state(read_saved_theme);

    // Apply whichever theme was saved (runs once on mount)
    {
        let current = current.clone();
        use_effect_with((), move |_| {
            apply_theme(&current);
            || ()
        });
    }

    html! {
        <div class="theme-switcher">
            { for THEMES.iter().map(|t| {
                let is_active = *current == t.id;
                let theme_id = t.id;
                let current = current.clone();

                let onclick = Callback::from(move |_: MouseEvent| {
                    current.set(theme_id.to_string());
                    apply_theme(theme_id);
                });

                let style = format!("--swatch-color: {}", t.color);
                let class = if is_active {
                    "theme-swatch active"
                } else {
                    "theme-swatch"
                };

                html! {
                    <button
                        key={t.id}
                        {class}
                        {style}
                        title={t.label}
                        {onclick}
                    />
                }
            })}
        </div>
    }
}

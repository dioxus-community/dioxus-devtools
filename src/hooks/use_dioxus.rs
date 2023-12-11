use dioxus::core::ScopeState;

/// Get the Dioxus description.
pub fn use_dioxus(_cx: &ScopeState) -> &'static str {
    "Dioxus is a Rust library for building apps that run on desktop, web, mobile, and more."
}

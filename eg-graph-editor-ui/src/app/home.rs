use crate::app::editor::Editor;
use leptos::{component, view, IntoView};

#[component]
pub fn Home() -> impl IntoView {
    view! { <Editor/> }
}

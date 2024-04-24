use leptos::*;
use leptos_router::*;

#[component]
pub fn PageNotFound() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center gap-3 p-3">
            <h1 class="fg-color-0 text-center text-3xl">Page not found</h1>
            <A href="/" class="fg-color-0 bg-slate-500 rounded-md p-2">
                "Go back to home page"
            </A>
        </div>
    }
}

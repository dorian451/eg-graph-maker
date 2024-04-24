pub mod editor;
pub mod header;
pub mod home;
pub mod pagenotfound;

use self::{header::Header, home::Home, pagenotfound::PageNotFound};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html class="w-full h-full flex"/>
        <Body class="flex-grow flex flex-col"/>

        <Router>
            <Header/>

            <main class="flex flex-grow bg-color-0 flex-col align-middle pt-4">
                <Routes>
                    <Route path="/" view=move || view! { <Home/> }/>
                    <Route path="/*any" view=PageNotFound/>

                </Routes>
            </main>
        </Router>
    }
}

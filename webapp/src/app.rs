use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::{
    home::HomePage, not_found::NotFoundPage, profile::ProfilePage, search::SearchPage,
    vehicle::VehiclePage,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html lang="en" dir="ltr" attr:data-theme="light"/>

        <Title text="Flow Car Studio"/>

        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <Stylesheet id="leptos" href="/pkg/flowcar.css"/>

        <Link
            href="https://cdn.jsdelivr.net/npm/bootstrap@5.2.3/dist/css/bootstrap.min.css"
            rel="stylesheet"
            integrity="sha384-rbsA2VBKQhggwzxH7pPCaAqO46MgnOM80zW1RWuH61DGLwZJEdK2Kadq2F9CUG65"
            crossorigin="anonymous"
        />
        <Link
            rel="stylesheet"
            href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.11.3/font/bootstrap-icons.min.css"
        />

        <script
            src="https://cdn.jsdelivr.net/npm/bootstrap@5.2.3/dist/js/bootstrap.bundle.min.js"
            integrity="sha384-kenU1KFdBIe4zVF0s0G1M5b4hcpxyD9F7jL+jjXkk+Q2h455rYXK/7HAuoJl+0I4"
            crossorigin="anonymous"
        ></script>

        <Router>
            <Routes>
                <Route path="/" view=HomePage/>
                <Route path="/profile" view=ProfilePage/>
                <Route path="/search" view=SearchPage/>
                <Route path="/search/:page" view=SearchPage/>
                <Route path="/vehicle/:id" view=VehiclePage/>
                <Route path="/:not_found" view=NotFoundPage/>
            </Routes>
        </Router>
    }
}

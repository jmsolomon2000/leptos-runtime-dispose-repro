use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::server;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/runtime-dispose-repro.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (load_count_sig, load_count_write) = create_signal(0);
    let resource_1 = create_resource(move || {load_count_sig.get() }, move |_| {server::server_fn_1()});
    let resource_2 = create_resource(move || { load_count_sig.get() }, move |_| {server::server_fn_2()});
    let resource_3 = create_resource(move || { load_count_sig.get() }, move |_| {server::server_fn_3()});
    let on_click = move |_| load_count_write.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <div>
            <button on:click=on_click  >Click</button>
        </div>
        <Suspense fallback=move || view! { <option>"Loading..."</option> }>
            <div>
                { move || resource_1.get() }
            </div>
        </Suspense>
        <Suspense fallback=move || view! { <option>"Loading..."</option> }>
            <div>
                { move || resource_2.get() }
            </div>
        </Suspense>
        <Suspense fallback=move || view! { <option>"Loading..."</option> }>
            <div>
                { move || resource_3.get() }
            </div>
        </Suspense>
    }
}

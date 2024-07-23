use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1>"Welcome to Leptos!"</h1>
        <Counter/>
    }
}

#[island]
fn Counter() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);
    view! {
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <div>
            <Stylesheet id="leptos" href="/pkg/beu.css"/>
            <Title text="Being you"/>
            <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
            <Meta name="description" content="Being you"/>
            <Router>
                <main>
                    <Routes>
                        <Route path="" view=HomePage/>
                    </Routes>
                </main>
            </Router>
        </div>
    }
}

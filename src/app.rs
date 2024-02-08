use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <nav>
            <h1>"Reserch App"</h1>
        </nav>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/research-leptos-ssr.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
        <Script>
            {"
            const video = document.getElementById('video')
            navigator.mediaDevices.getUserMedia({
                video: true,
                audio: false,
            }).then(stream => {
                video.srcObject = stream;
                video.play()
            }).catch(error => {
                console.log(error)
            })
            "}
        </Script>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    /* let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1); */

    view! {
        <Header />
        <h1>"Welcome to Leptos!"</h1>
        /* <button on:click=on_click>"Click Me: " {count}</button> */
        <video id="video"></video>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}

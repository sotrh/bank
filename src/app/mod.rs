use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod end_screen;
mod start_screen;
mod game_screen;

use crate::app::start_screen::StartScreen;
use crate::app::game_screen::GameScreen;
use crate::app::end_screen::EndScreen;

#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    score: u32,
}

#[derive(Debug, Clone)]
pub struct PlayerContext(ReadSignal<Vec<Player>>, WriteSignal<Vec<Player>>);

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (players, set_players) = create_signal(Vec::<Player>::new());
    provide_context(PlayerContext(players, set_players));

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/bank.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=StartScreen/>
                    <Route path="/game" view=GameScreen/>
                    <Route path="/end" view=EndScreen/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
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

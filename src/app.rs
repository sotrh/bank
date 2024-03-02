use leptos::*;
use leptos_meta::*;
use leptos_router::*;

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
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let PlayerContext(players, set_players) = use_context().unwrap();
    let (name, set_name) = create_signal("".to_owned());
    let (error, set_error) = create_signal("");

    let add_name = move |_| {
        let name = name.get();

        if error.with(|error| error.len() > 0) || name.len() == 0 {
            return;
        }

        set_players.update(move |players| {
            players.push(Player { name, score: 0 });
        });
    };

    let name_input = move |ev| {
        set_name(event_target_value(&ev));

        // Maybe disable the button on error?
        if players.with(|players| {
            players
                .iter()
                .find(|player| player.name == name.get())
                .is_some()
        }) {
            set_error.set("Player name already taken");
        } else {
            set_error.set("")
        }
    };

    view! {
        <h1>"Bank"</h1>
        <input type="text"
            on:input=name_input
            prop:value=name
        />
        <button on:click=add_name>"Add Player"</button>
        <div>{error}</div>
        <ul>
            <For
                each=players
                key=|player| player.name.clone()
                children=move |player| {
                    view! {
                        <li>{player.name}</li>
                    }
                }
            />
        </ul>
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

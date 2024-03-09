use leptos::*;
use leptos_router::use_navigate;

use super::Player;

use super::PlayerContext;

/// Renders the home page of your application.
#[component]
pub(crate) fn StartScreen() -> impl IntoView {
    let (play_pressed, set_play_pressed) = create_signal(false);

    let start_game = || {
        let navigate = use_navigate();
        navigate("/game", Default::default());
    };

    view! {
        <h1>"Bank"</h1>
        {move || if play_pressed.get() {
            view! {
                <PlayerPopup start_game=start_game/>
            }
        } else {
            view! {
                <button on:click=move |_| set_play_pressed.set(true)>
                    "Play?"
                </button>
            }.into_view()
        }}
    }
}

#[component]
fn PlayerPopup<F>(start_game: F) -> impl IntoView
where
    F: Fn() + 'static,
{
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

    let disable_button = {
        let players = players.clone();
        move || players.get().len() < 2
    };

    view! {
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
        <button disabled=disable_button on:click=move |_| start_game() >
            "Start Game"
        </button>
    }
}

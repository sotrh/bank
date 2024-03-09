use leptos::*;

use crate::app::PlayerContext;

#[component]
pub fn GameScreen() -> impl IntoView {
    let (round, set_round) = create_signal(1);
    let (score, set_score) = create_signal(0);

    view! {
        <div>{round}</div>
        <h1>{score}</h1>
        <div>"(game score)"</div>
        <div>
            <Players />
            <Keypad />
        </div>
        <Footer />
    }
}


#[component]
fn Players() -> impl IntoView {
    let (current_player, set_current_player) = create_signal(0);
    let PlayerContext(players, set_players) = use_context().unwrap();
    
    view! {
        <div>"Chelsea's turn " {current_player}</div>
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


#[component]
fn Keypad() -> impl IntoView {
    view! {
        <div id="keypad">
            <button>"2"</button>
            <button>"3"</button>
            <button>"4"</button>
            <button>"5"</button>
            <button>"6"</button>
            <button>"7"</button>
            <button>"8"</button>
            <button>"9"</button>
            <button>"10"</button>
            <button>"11"</button>
            <button>"12"</button>
            <button>"Doubles!"</button>
        </div>
    }
}


#[component]
fn Footer() -> impl IntoView {
    view! {
        <button>"Bank"</button>
        <button>"Toggle Keypad"</button>
        <button>"Info"</button>
    }
}
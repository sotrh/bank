use leptos::*;

#[component]
pub fn GameScreen() -> impl IntoView {

    view! {
        <div>
            <Keypad />
        </div>
        <Footer />
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
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let value = use_state(|| 0);
    
    let on_increment = {
        let value = value.clone();
        Callback::from(move |_| value.set(*value + 1))
    };

    let on_decrement = {
        let value = value.clone();
        Callback::from(move |_| value.set(*value - 1))
    };

    html! {
        <>
        <h1>{ "Yew framework" }</h1>
        <p>{ "The basic Elm language counter demo is converted to Rust." }</p>
        <p>{ "for Nathan Coulter (2023)" }</p>
        <div>
        <button onclick={on_decrement}>{"-"}</button>
        { *value }
        <button onclick={on_increment}>{"+"}</button>
        </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
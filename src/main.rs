use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div>
        <h1>{ "swan.sh" }</h1>
        <p>{ "Placeholder Site for swan.sh" }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
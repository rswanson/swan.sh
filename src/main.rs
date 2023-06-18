use stylist::css;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div class={css!(r#"
            align-items: center;
            background-color: #5ec5f5;
            display: flex;
            flex-direction: column;
            justify-content: center;
            position: absolute;
            width: 300px;
            height: 200px;
            z-index: 15;
            top: 50%;
            left: 50%;
            margin: -100px 0 0 -150px; 
            font-family: 'Roboto Mono', monospace;
            font-weight: 400;
            line-height: 1.2;

        "#)}>
        <img src="https://swan.sh/static/swan.png" alt="swan.sh logo" class={css!(r#"
            width: 75;
            height: 75px;
            margin: 0;
            padding: 0;
            "#)} />
        <h1 class={css!(r#"
            font-size: 3rem;
            text-transform: uppercase;
            margin: 0;
            padding: 0;
            "#)}>{ "swan.sh" }</h1>
        <p class={css!(r#"
            font-size: 1rem;
            margin: 0;
            padding: 0;
            "#)}>{ "Placeholder Site for swan.sh" }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

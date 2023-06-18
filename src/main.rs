use yew::prelude::*;
use stylist::yew::Global;
use stylist::css;

#[function_component]
fn App() -> Html {
    html! {
        <>
        <Global css={css!(r#"
                font-family: 'Roboto Mono', monospace;
                font-weight: 400;
                line-height: 1.2;
                background-color: #1cc4fc;
            "#)} />
            <div class={css!(r#"
                align-items: center;
                display: flex;
                flex-direction: column;
                justify-content: center;
                position: absolute;
                width: 500px;
                height: 400px;
                z-index: 15;
                top: 50%;
                left: 50%;
                margin-top: -200px;
                margin-left: -250px;
                border-radius: 20px;
            "#)}>
            <img src="https://swan.sh/static/swan.png" alt="swan.sh logo" class={css!(r#"
                width: 150px;
                height: 150px;
                border-radius: 50%;
                outline: 5px solid #fff;
                "#)} />
            <h1 class={css!(r#"
                font-size: 3rem;
                text-transform: lowercase;
                margin: 0;
                padding: 0;
                "#)}>{ "swan.sh" }</h1>
            <p class={css!(r#"
                font-size: 1rem;
                margin: 0;
                padding: 0;
                "#)}>{ "coming soon™" }</p>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

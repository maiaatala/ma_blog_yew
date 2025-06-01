use yew::prelude::*;


#[function_component(App)]
fn app() -> Html {
    let test = std::env!("TEST_VAR");

    html! {
        <>
            <h1>{ "Yew with Trunk Env" }</h1>
            <p>{ format!("TEST_VAR = {}", test) }</p>
        </>
    }
}


fn main() {
    console_error_panic_hook::set_once(); // optional but useful
    yew::Renderer::<App>::new().render();
}

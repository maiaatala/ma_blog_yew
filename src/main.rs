use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;
mod routes;
mod layout;
mod models;
mod api;

use crate::pages::{about::About, home::Home};
use crate::routes::Route;
use crate::layout::Layout;

fn switch(route: Route) -> Html {
    let content = match route {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::NotFound => html! { <h1>{ "404 - Not Found" }</h1> },
    };

    html! { <Layout>{ content }</Layout> }
}

#[function_component(App)]
fn app() -> Html {
    //let test = std::env!("TEST_VAR");

    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}


fn main() {
    console_error_panic_hook::set_once(); // optional but useful
    yew::Renderer::<App>::new().render();
}

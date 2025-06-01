// src/components/navbar.rs
use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::Route;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let open = use_state(|| false);

    let toggle = {
        let open = open.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation(); 
            open.set(!*open);
        })
    };

    html! {
        <header onclick={Callback::from({
            let open = open.clone();
            move |_| open.set(false)
        })}>
            <nav class="navbar">
                <h1 class="logo">{ "HTMX Page" }</h1>
                <span _="on click from elsewhere remove .open from #navLinks">
                    <div
                        class="hamburger"
                        _="on click toggle .open on #navLinks"
                    >
                        <span></span>
                        <span></span>
                        <span></span>
                    </div>
                         <nav
                            class={classes!("nav-links", if *open { Some("open") } else { None })}
                            id="navLinks"
                            onclick={toggle}
                        >
                            <Link<Route> to={Route::Home}>
                                <button>{ "Home" }</button>
                            </Link<Route>>
                            <Link<Route> to={Route::NotFound}>
                                <button>{ "Apps" }</button>
                            </Link<Route>>
                            <Link<Route> to={Route::About}>
                                <button>{ "Sobre" }</button>
                            </Link<Route>>
                        </nav>
                </span>
            </nav>
        </header>
    }
}


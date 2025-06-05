use yew::prelude::*;

#[function_component(Loading)]
pub fn loading() -> Html {
    html! {
        <div class="loader-svg">
            <div class="top-layer">
                <div class="mid">
                    <svg width="75" height="75" viewBox="0 0 31 31">
                        <polygon fill="transparent" stroke-width="2" points="15,0 30,30 0,30" />
                    </svg>
                </div>
            </div>
            <div class="bottom-layer">
                <div class="top">
                    <svg width="75" height="75" viewBox="0 0 31 31">
                        <polygon fill="transparent" stroke-width="2" points="15,0 30,30 0,30" />
                    </svg>
                </div>
                <div class="bot">
                    <svg width="75" height="75" viewBox="0 0 31 31">
                        <polygon fill="transparent" stroke-width="2" points="15,0 30,30 0,30" />
                    </svg>
                </div>
            </div>
        </div>
    }
}


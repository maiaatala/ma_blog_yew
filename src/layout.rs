use yew::prelude::*;
use crate::components::navbar::Navbar;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    html! {
        <>
            <Navbar />
            <main id="content-swap">
                { for props.children.iter() }
            </main>
        </>
    }
}


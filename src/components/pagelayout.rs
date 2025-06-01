use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DefaultPageLayoutProps {
    #[prop_or_default]
    pub left_content: Option<Html>,

    #[prop_or_default]
    pub right_content: Option<Html>,

    #[prop_or_default]
    pub children: Children,
}

#[function_component(DefaultPageLayout)]
pub fn default_page_layout(props: &DefaultPageLayoutProps) -> Html {
    html! {
        <section>
            <aside>
                { props.left_content.clone().unwrap_or_default() }
            </aside>
            <main>
                { for props.children.iter() }
            </main>
            <aside>
                { props.right_content.clone().unwrap_or_default() }
            </aside>
        </section>
    }
}


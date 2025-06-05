// src/pages/home.rs
use yew::prelude::*;
use yew_hooks::use_infinite_scroll;
use crate::models::models::ShortPost;   // adjust path if needed
use crate::api::fetch_posts;
use crate::components::loading::Loading;

#[function_component(Home)]
pub fn home() -> Html {
    // ─── 1) State hooks ───────────────────────────────────────────────────────
    let posts = use_state(|| Vec::<ShortPost>::new());
    let page = use_state(|| 1_usize);
    let has_more = use_state(|| true);
    let loading = use_state(|| false);

    // A “sentinel” div that we'll watch for infinite scroll:
    let sentinel_ref = use_node_ref();

    // ─── 2) on_load_more closure ──────────────────────────────────────────────
    // This runs whenever we need to fetch “page” if has_more==true and not loading.
    let on_load_more = {
        let posts = posts.clone();
        let page = page.clone();
        let has_more = has_more.clone();
        let loading = loading.clone();

        // Note: this is a plain Rust closure (Fn()), not a Yew Callback.
        move || {
            if *has_more && !*loading {
                let posts = posts.clone();
                let page = page.clone();
                let has_more = has_more.clone();
                let loading = loading.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    loading.set(true);
                    let current_page = *page;
                    if let Ok(res) = fetch_posts(current_page, 15).await {
                        // Append new items:
                        let mut new_posts = (*posts).clone();
                        new_posts.extend(res.items);

                        // Calculate total pages:
                        let total_pages = (res.totalItems + 15 - 1) / 15;
                        has_more.set(current_page + 1 < total_pages);

                        // Commit new state:
                        posts.set(new_posts);
                        page.set(current_page + 1);
                    }
                    loading.set(false);
                });
            }
        }
    };

    // ─── 3) use_infinite_scroll to load pages 2, 3, ... ───────────────────────
    {
        let on_load_more = on_load_more.clone();
        use_infinite_scroll(sentinel_ref.clone(), on_load_more);
    }

    // ─── 4) use_effect once to load the very first page immediately ───────────
    {
        let on_load_more = on_load_more.clone();
        // This effect runs exactly once (deps = ()), so it loads page 1 on mount.
        use_effect_with((), move |&()| {
            on_load_more();
            || ()
        });

    }

    // ─── 5) Render ─────────────────────────────────────────────────────────────
    html! {
        <main id="content-swap">
            <div class="posts-grid">
                { 
                    for (*posts).iter().map(|p| html! {
                    <article  class="postcard-position">
                        <div class="postcard-wrapper">
                            <img src={p.image.clone()} alt={p.title.clone()} />
                            <div class="metadata">
                                <h3>{ &p.title }</h3>
                                <p class="description">{ &p.description }</p>
                                <div class="author">
                                  <p>{ &p.createdAt }</p>
                                  <p>{"•"}</p>
                                  <p>{ &p.author.name }</p>
                                </div>
                            </div>
                        </div>
                    </article>
                    }) 
                }
            </div>

            // Show a spinner if we’re in the middle of a fetch:
            {
                if *loading {
                    html! { <Loading /> }
                } else {
                    html! {}
                }
            }

            // The invisible sentinel at the bottom that triggers the next page load:
            <div ref={sentinel_ref} style="height: 1px; width: 1px;" />
        </main>
    }
}


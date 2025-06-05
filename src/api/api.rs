use crate::models::models::ShortPostPaginated;
use gloo_net::http::Request;

// This pulls the API_URL from your build-time environment (.env)
const API_URL: &str = env!("API_URL");

pub async fn fetch_posts(
    page: usize,
    items_per_page: usize,
) -> Result<ShortPostPaginated, gloo_net::Error> {
    let url = format!(
        "{}/posts?itemsPerPage={}&page={}",
        API_URL,
        items_per_page,
        page
    );

    Request::get(&url)
        .send()
        .await?
        .json::<ShortPostPaginated>()
        .await
}


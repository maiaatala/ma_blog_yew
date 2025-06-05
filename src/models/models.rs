use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct ShortPost {
    pub id: String,
    pub createdAt: String,
    pub title: String,
    pub description: String,
    pub image: String,
    pub author: Author,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Author {
    pub name: String,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct ShortPostPaginated {
    pub totalItems: usize,
    pub page: usize,
    pub items: Vec<ShortPost>,
}


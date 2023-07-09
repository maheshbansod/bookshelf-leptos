use getset::Getters;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Getters, Clone)]
#[getset(get = "pub")]
pub struct Bookshelf {
    books: Vec<Book>,
}

#[derive(Debug, Serialize, Deserialize, Getters, Clone, Default)]
#[getset(get = "pub")]
pub struct Book {
    id: String,
    cover_src: String,
    title: String,
    authors: Vec<String>,
    description: String,
}

impl Bookshelf {
    pub fn new() -> Self {
        Self { books: vec![] }
    }

    pub fn add_book(&mut self, book: Book) {
        leptos::log!("Adding book: {:?} in {:?}", book, self.books());
        self.books.push(book);
    }
}

impl From<RawResponseDoc> for Book {
    fn from(value: RawResponseDoc) -> Self {
        Self {
            id: value.key,
            title: value.title,
            authors: value.author_name,
            ..Book::default()
        }
    }
}

#[derive(Serialize, Deserialize)]
struct RawResponseDoc {
    key: String,
    title: String,
    author_name: Vec<String>,
}
#[derive(Serialize, Deserialize)]
struct RawResponse {
    docs: Vec<RawResponseDoc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Error)]
pub enum BookshelfError {
    #[error("Request error")]
    RequestError,
}

impl From<reqwest::Error> for BookshelfError {
    fn from(_value: reqwest::Error) -> Self {
        BookshelfError::RequestError
    }
}

pub async fn search_book(query: &str) -> Result<Vec<Book>, BookshelfError> {
    let url = "https://openlibrary.org/search.json";
    let client = reqwest::Client::new();
    let results: RawResponse = client
        .get(url)
        .query(&[
            ("q", query),
            ("fields", "key, title, author_name"),
            ("limit", "10"),
        ])
        .send()
        .await?
        .json()
        .await?;
    let books = results.docs.into_iter().map(Book::from).collect();
    leptos::log!("{books:?}");
    Ok(books)
}

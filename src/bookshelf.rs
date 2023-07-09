use getset::Getters;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Getters, Clone, Serialize, Deserialize)]
#[getset(get = "pub")]
pub struct Bookshelf {
    books: Vec<Book>,
}

#[derive(Debug, Serialize, Deserialize, Getters, Clone, Default)]
#[getset(get = "pub")]
pub struct Book {
    id: String,
    cover_src: Option<CoverSrc>,
    title: String,
    authors: Vec<String>,
    first_publish_year: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CoverSrc(String);
impl From<u32> for CoverSrc {
    fn from(value: u32) -> Self {
        CoverSrc(format!("https://covers.openlibrary.org/w/id/{value}-M.jpg"))
    }
}

impl CoverSrc {
    pub fn get(&self) -> &String {
        &self.0
    }
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
            cover_src: value.cover_i.map(|c| c.into()),
            first_publish_year: value.first_publish_year,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct RawResponseDoc {
    key: String,
    title: String,
    author_name: Vec<String>,
    cover_i: Option<u32>,
    first_publish_year: u16,
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
    let results: Result<RawResponse, _> = client
        .get(url)
        .query(&[
            ("q", query),
            (
                "fields",
                "key, first_publish_year, title, author_name, cover_i",
            ),
            ("limit", "10"),
        ])
        .send()
        .await?
        .json()
        .await;
    match results {
        Ok(results) => {
            let books = results.docs.into_iter().map(Book::from).collect();
            leptos::log!("{books:?}");
            Ok(books)
        }
        Err(err) => {
            leptos::error!("{err:?}");
            Err(BookshelfError::RequestError)
        }
    }
}

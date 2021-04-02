use bitsors::websocket::*;

#[cfg(test)]
#[derive(serde_derive::Deserialize)]
pub struct BitsoBooks {
    pub payload: Vec<Book>,
}

#[cfg(test)]
#[derive(serde_derive::Deserialize)]
pub struct Book {
    #[serde(rename = "book")]
    pub name: String,
}

#[tokio::test]
async fn all_books_and_proper_name() {
    let current_books = reqwest::get("https://api.bitso.com/v3/available_books/")
        .await
        .unwrap()
        .json::<BitsoBooks>()
        .await
        .unwrap();

    assert_eq!(current_books.payload.len(), Books::COUNT);

    let mut current_books: Vec<&str> = current_books
        .payload
        .iter()
        .map(|b| b.name.as_str())
        .collect();
    current_books.sort_unstable();

    let mut enum_books: Vec<String> = Books::iter().map(|b| b.to_string()).collect();
    enum_books.sort_unstable();

    assert_eq!(current_books, enum_books);
}

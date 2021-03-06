use std::net::TcpListener;

use arg_parsing::kanji_to_filename;
use pages::Pages;
use pdf_creation::kanji_to_png;

pub mod arg_parsing;
pub mod pages;
pub mod pdf_creation;

#[derive(PartialEq, Eq)]
pub enum KanjiToPngErrors {
    FileNotFound,
    Undefined,
}

pub fn find_free_port() -> Option<u16> {
    (8000..55000).find(|port| TcpListener::bind(("127.0.0.1", *port)).is_ok())
}

pub async fn launch_browser(url: &str) {
    std::thread::sleep(std::time::Duration::from_millis(300));
    if webbrowser::open(url).is_ok() {}
}

pub fn create_pages(kanjis: &str) -> (Pages, Vec<char>) {
    let mut pages = Pages::default();
    pages.add_page();
    let mut skipped_kanji = Vec::<char>::with_capacity(10);

    for kanji in kanjis.chars() {
        if let Err(e) = kanji_to_png(&mut pages, &kanji_to_filename(kanji)) {
            if e == KanjiToPngErrors::FileNotFound {
                skipped_kanji.push(kanji);
            }
        }
    }
    (pages, skipped_kanji)
}

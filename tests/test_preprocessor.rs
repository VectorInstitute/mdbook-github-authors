use mdbook::MDBook;

#[test]
fn github_authors_works() {
    // Tests that the remove-emphasis example works as expected.

    // Workaround for https://github.com/rust-lang/mdBook/issues/1424
    let book = MDBook::load("./test_book").unwrap();
    book.build().unwrap();
    let ch1 = std::fs::read_to_string("book/chapter_1.html").unwrap();
    assert!(ch1.contains("This has light emphasis and bold emphasis."));
}

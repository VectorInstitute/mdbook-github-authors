use mdbook::MDBook;

#[test]
fn github_authors_works() {
    // Tests that the github-authors example works as expected.

    // Workaround for https://github.com/rust-lang/mdBook/issues/1424
    std::env::set_current_dir("test_book").unwrap();
    let book = MDBook::load(".").unwrap();
    book.build().unwrap();
    let ch1 = std::fs::read_to_string("book/chapter_1/index.html").unwrap();
    let ch1_1 = std::fs::read_to_string("book/chapter_1/sub_chapter_1.html").unwrap();
    let ch1_2 = std::fs::read_to_string("book/chapter_1/sub_chapter_2.html").unwrap();

    // chapter 1
    assert!(
        ch1.contains("<hr style=\"border: none; border-top: 1px solid #ddd; margin: 20px 0;\">")
    );
    assert!(ch1.contains("<strong>Contributors:</strong>"));
    assert!(ch1.contains("<a href=\"https://github.com/nerdai\">"));
    // chapter 1.1
    assert!(!ch1_1.contains("<strong>Contributors:</strong>"));
    // chapter 1.2
    assert!(ch1_2.contains("<a href=\"https://github.com/nerdai\">"));
    assert!(ch1_2.contains("<a href=\"https://github.com/emersodb\">"));
}

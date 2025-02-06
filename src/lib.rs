//! # `mdbook-github-authors`
//!
//! This crate produces a preprocessor for the [rust-lang mdbook](https://github.com/rust-lang/mdBook)
//! project that lists authors via their Github profiles in a Contributor section
//! appended to the bottom of a Chapter.
//!
//! ## Basic Usage
//!
//! First, install the crate:
//!
//! ```sh
//! cargo install mdbook-github-authors
//! ```
//!
//! Next, and as with all preprocessor extensions, to include `mdbook-github-authors`
//! in your book, add the following to your `book.toml`:
//!
//! ```sh
//! [preprocessor.github-authors]
//! command = "mdbook-github-authors"
//! ```
//!
//! In order to add an author or list of authors in your chapter, there are two
//! supported helpers:
//!
//! ```markdown
//! <!-- for single author -->
//! {{#author <github-username>}}
//!
//! <!-- for multiple authors -->
//! {{#authors <comma-separated-list-of-usernames>>}}
//! ```
//!
//! For more details see the project's [README](https://github.com/VectorInstitute/mdbook-github-authors)

pub mod github_authors;

pub use github_authors::GithubAuthorsPreprocessor;

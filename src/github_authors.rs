use mdbook::book::Book;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use once_cell::sync::Lazy;
use regex::{CaptureMatches, Captures, Regex};

#[derive(Default)]
pub struct GithubAuthorsPreprocessor;

/// A preprocess for expanding "authors" helper.
///
/// {{#author <github-username>}}
impl GithubAuthorsPreprocessor {
    pub(crate) const NAME: &'static str = "github-author";

    pub fn new() -> Self {
        GithubAuthorsPreprocessor
    }
}

impl Preprocessor for GithubAuthorsPreprocessor {
    fn name(&self) -> &str {
        Self::NAME
    }

    #[allow(unused_variables)]
    fn run(&self, ctx: &PreprocessorContext, book: Book) -> anyhow::Result<Book> {
        // Gameplan:
        // 1. Find all authors helper using reg-ex in chapter content, using `find_authors`
        // 2. Sequentially erase all authors helpers from the content
        // 3. Use handlebar template `authors.hbs` and render the found authors
        // 4. Take the rendered html string and add it to the end of the chapter content
        todo!()
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
struct Author<'a> {
    start_index: usize,
    end_index: usize,
    github_username: &'a str,
}

impl<'a> Author<'a> {
    #[allow(dead_code, unused_variables)]
    fn from_capture(cap: Captures<'a>) -> Option<Author<'a>> {
        todo!()
    }
}

#[allow(dead_code)]
struct AuthorIter<'a>(CaptureMatches<'a, 'a>);

impl<'a> Iterator for AuthorIter<'a> {
    type Item = Author<'a>;
    fn next(&mut self) -> Option<Author<'a>> {
        for cap in &mut self.0 {
            if let Some(inc) = Author::from_capture(cap) {
                return Some(inc);
            }
        }
        None
    }
}

#[allow(dead_code)]
fn find_authors(contents: &str) -> AuthorIter<'_> {
    // lazily compute following regex
    // r"\\\{\{#.*\}\}|\{\{#([a-zA-Z0-9]+)\s*([^}]+)\}\}")?;
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(
            r"(?x)              # insignificant whitespace mode
        \\\{\{\#.*\}\}      # match escaped link
        |                   # or
        \{\{\s*             # link opening parens and whitespace
        \#([a-zA-Z0-9_]+)   # link type
        \s+                 # separating whitespace
        ([^}]+)             # link target path and space separated properties
        \}\}                # link closing parens",
        )
        .unwrap()
    });

    AuthorIter(RE.captures_iter(contents))
}

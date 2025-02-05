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
        // 5. Figure out if need to make a cli for this and use CmdPreprocessor
        todo!()
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
enum AuthorLinkType<'a> {
    SingleAuthor(&'a str),
    MultipleAuthors(&'a str),
}

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
struct AuthorLink<'a> {
    start_index: usize,
    end_index: usize,
    link_type: AuthorLinkType<'a>,
    input: &'a str,
}

impl<'a> AuthorLink<'a> {
    #[allow(dead_code, unused_variables)]
    fn from_capture(cap: Captures<'a>) -> Option<AuthorLink<'a>> {
        let link_type = match (cap.get(0), cap.get(1), cap.get(2)) {
            (_, Some(typ), Some(author)) if typ.as_str() == "author" => {
                Some(AuthorLinkType::SingleAuthor(author.as_str()))
            }
            (_, Some(typ), Some(authors_list)) if typ.as_str() == "authors" => {
                Some(AuthorLinkType::MultipleAuthors(authors_list.as_str()))
            }
            _ => None,
        };
        todo!()
    }
}

#[allow(dead_code)]
struct AuthorLinkIter<'a>(CaptureMatches<'a, 'a>);

impl<'a> Iterator for AuthorLinkIter<'a> {
    type Item = AuthorLink<'a>;
    fn next(&mut self) -> Option<AuthorLink<'a>> {
        for cap in &mut self.0 {
            if let Some(inc) = AuthorLink::from_capture(cap) {
                return Some(inc);
            }
        }
        None
    }
}

#[allow(dead_code)]
fn find_authors(contents: &str) -> AuthorLinkIter<'_> {
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

    AuthorLinkIter(RE.captures_iter(contents))
}

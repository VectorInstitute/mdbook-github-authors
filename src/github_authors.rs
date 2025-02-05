use handlebars::{to_json, Handlebars};
use mdbook::book::{Book, BookItem};
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use once_cell::sync::Lazy;
use regex::{CaptureMatches, Captures, Regex};
use serde::Serialize;
use serde_json::value::Map;

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
    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> anyhow::Result<Book> {
        // Gameplan:
        // 1. Find all authors helper using reg-ex in chapter content, using `find_author_links`
        // 2. Sequentially erase all authors helpers from the content
        // 3. Use handlebar template `authors.hbs` and render the found authors
        // 4. Take the rendered html string and add it to the end of the chapter content
        // 5. Figure out if need to make a cli for this and use CmdPreprocessor
        let src_dir = ctx.root.join(&ctx.config.book.src);

        book.for_each_mut(|section: &mut BookItem| {
            if let BookItem::Chapter(ref mut ch) = *section {
                let (mut content, github_authors) = remove_all_links(&ch.content);

                // get contributors html section
                let mut data = Map::new();
                data.insert("authors".to_string(), to_json(github_authors));
                let mut handlebars = Handlebars::new();

                // register template from a file and assign a name to it
                handlebars
                    .register_template_file("contributors", "./template/author.hbs")
                    .unwrap();

                let contributors_html = handlebars.render("contributors", &data).unwrap();
                println!("{:?}", contributors_html);
                content.push_str(contributors_html.as_str());

                // mutate chapter content
                ch.content = content;
            }
        });

        Ok(book)
    }
}

fn remove_all_links(s: &str) -> (String, Vec<GithubAuthor>) {
    let mut previous_end_index = 0;
    let mut replaced = String::new();
    let mut github_authors_vec = vec![];

    for link in find_author_links(s) {
        // remove the author link from the chapter content
        replaced.push_str(&s[previous_end_index..link.start_index]);
        replaced.push_str("");
        previous_end_index = link.end_index;

        // store the author usernames to create the contributors section with handlebars
        let these_authors = match link.link_type {
            AuthorLinkType::SingleAuthor(author) => {
                vec![GithubAuthor {
                    username: author.to_string(),
                }]
            }
            AuthorLinkType::MultipleAuthors(author_list) => author_list
                .split(",")
                .map(|username| GithubAuthor {
                    username: username.to_string(),
                })
                .collect(),
        };

        github_authors_vec.extend(these_authors);
    }

    replaced.push_str(&s[previous_end_index..]);
    (replaced, github_authors_vec)
}

#[derive(PartialEq, Debug, Clone, Serialize)]
pub struct GithubAuthor {
    username: String,
}

#[derive(PartialEq, Debug, Clone)]
enum AuthorLinkType<'a> {
    SingleAuthor(&'a str),
    MultipleAuthors(&'a str),
}

#[derive(PartialEq, Debug, Clone)]
struct AuthorLink<'a> {
    start_index: usize,
    end_index: usize,
    link_type: AuthorLinkType<'a>,
    link_text: &'a str,
}

impl<'a> AuthorLink<'a> {
    fn from_capture(cap: Captures<'a>) -> Option<AuthorLink<'a>> {
        let link_type = match (cap.get(0), cap.get(1), cap.get(2)) {
            (_, Some(typ), Some(author))
                if ((typ.as_str() == "author") && (!author.as_str().trim().is_empty())) =>
            {
                Some(AuthorLinkType::SingleAuthor(author.as_str().trim()))
            }
            (_, Some(typ), Some(authors_list))
                if ((typ.as_str() == "authors") && (!authors_list.as_str().trim().is_empty())) =>
            {
                Some(AuthorLinkType::MultipleAuthors(
                    authors_list.as_str().trim(),
                ))
            }
            _ => None,
        };

        link_type.and_then(|lnk_type| {
            cap.get(0).map(|mat| AuthorLink {
                start_index: mat.start(),
                end_index: mat.end(),
                link_type: lnk_type,
                link_text: mat.as_str(),
            })
        })
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
fn find_author_links(contents: &str) -> AuthorLinkIter<'_> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use rstest::*;

    #[fixture]
    fn simple_book_content() -> String {
        "Some random text with and more text ... {{#author foo}} {{#authors bar,baz  }}".to_string()
    }

    #[rstest]
    fn test_find_links_no_author_links() -> Result<()> {
        let s = "Some random text without link...";
        assert!(find_author_links(s).collect::<Vec<_>>() == vec![]);
        Ok(())
    }

    #[rstest]
    fn test_find_links_partial_link() -> Result<()> {
        let s = "Some random text with {{#playground...";
        assert!(find_author_links(s).collect::<Vec<_>>() == vec![]);
        let s = "Some random text with {{#include...";
        assert!(find_author_links(s).collect::<Vec<_>>() == vec![]);
        let s = "Some random text with \\{{#include...";
        assert!(find_author_links(s).collect::<Vec<_>>() == vec![]);
        Ok(())
    }

    #[rstest]
    fn test_find_links_empty_link() -> Result<()> {
        let s = "Some random text with {{#author  }} and {{#authors   }} {{}} {{#}}...";
        println!("{:?}", find_author_links(s).collect::<Vec<_>>());
        assert!(find_author_links(s).collect::<Vec<_>>() == vec![]);
        Ok(())
    }

    #[rstest]
    fn test_find_links_unknown_link_type() -> Result<()> {
        let s = "Some random text with {{#my_author ar.rs}} and {{#auth}} {{baz}} {{#bar}}...";
        assert!(find_author_links(s).collect::<Vec<_>>() == vec![]);
        Ok(())
    }

    #[rstest]
    fn test_find_links_simple_author_links(simple_book_content: String) -> Result<()> {
        let res = find_author_links(&simple_book_content[..]).collect::<Vec<_>>();
        println!("\nOUTPUT: {res:?}\n");

        assert_eq!(
            res,
            vec![
                AuthorLink {
                    start_index: 40,
                    end_index: 55,
                    link_type: AuthorLinkType::SingleAuthor("foo"),
                    link_text: "{{#author foo}}",
                },
                AuthorLink {
                    start_index: 56,
                    end_index: 78,
                    link_type: AuthorLinkType::MultipleAuthors("bar,baz"),
                    link_text: "{{#authors bar,baz  }}",
                },
            ]
        );
        Ok(())
    }

    #[rstest]
    fn test_remove_all_links(simple_book_content: String) -> Result<()> {
        let (c, authors) = remove_all_links(&simple_book_content[..]);

        assert_eq!(c, "Some random text with and more text ...  ");
        assert_eq!(
            authors,
            vec![
                GithubAuthor {
                    username: "foo".to_string()
                },
                GithubAuthor {
                    username: "bar".to_string()
                },
                GithubAuthor {
                    username: "baz".to_string()
                }
            ]
        );
        Ok(())
    }
}

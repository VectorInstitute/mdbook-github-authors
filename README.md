# mdbook-github-authors

----------------------------------------------------------------------------------------

[![Lint](https://github.com/VectorInstitute/mdbook-github-authors/actions/workflows/lint.yml/badge.svg)](https://github.com/VectorInstitute/mdbook-github-authors/actions/workflows/lint.yml)
[![Test Docs](https://github.com/VectorInstitute/mdbook-github-authors/actions/workflows/test_docs.yml/badge.svg)](https://github.com/VectorInstitute/mdbook-github-authors/actions/workflows/test_docs.yml)
[![Test Lib](https://github.com/VectorInstitute/mdbook-github-authors/actions/workflows/test.yml/badge.svg)](https://github.com/VectorInstitute/mdbook-github-authors/actions/workflows/test.yml)
![GitHub License](https://img.shields.io/github/license/VectorInstitute/mdbook-github-authors)
![GitHub Release](https://img.shields.io/github/v/release/VectorInstitute/mdbook-github-authors)

A preprocessor for [mdbook](https://rust-lang.github.io/mdBook/) that creates
chapter-level contributor sections featuring authors' GitHub profiles.

## Installation

```bash
cargo install mdbook-github-authors
```

## Usage

1. Add to your `book.toml`:

```toml
[preprocessor.github-authors]
command = "mdbook-github-authors"
```

1. Add contributors/authors using these helpers in your markdown:

```markdown
<!-- Single author -->
{{#author username}}

<!-- Multiple authors -->
{{#authors username1,username2,username3}}
```

The preprocessor will generate a "Contributors" section at the bottom of each chapter
listing the GitHub profiles of specified authors.

## Examples

```markdown
# My Chapter

Content here...

{{#authors rust-lang,contributors}}
```

Will render as:

//! Parses marp markdown to remove yaml header and insert links to rendered slides.
use mdbook_preprocessor::book::Book;
use mdbook_preprocessor::errors::Result;
use mdbook_preprocessor::{Preprocessor, PreprocessorContext};
use pulldown_cmark::{
    CowStr, Event, HeadingLevel, LinkType, MetadataBlockKind, Options, Parser, Tag, TagEnd,
};
use regex::Regex;
use std::io;

fn main() {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("supports") => {
            // Supports all renderers.
            return;
        }
        Some(arg) => {
            eprintln!("unknown argument: {arg}");
            std::process::exit(1);
        }
        None => {}
    }

    if let Err(e) = handle_preprocessing() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

struct Marp;

impl Preprocessor for Marp {
    fn name(&self) -> &str {
        "marp"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        book.for_each_chapter_mut(|ch| {
            let title = match &ch.number {
                Some(_) => {
                    let lec_num = ch.number.as_ref().unwrap().first().unwrap();
                    format!("Lecture {}: {}", lec_num, ch.name)
                },
                None => ch.name.clone(),
            };

            match process_marp_header(
                &ch.content,
                &title,
                ch.source_path.as_ref().unwrap().file_stem().unwrap().to_str().unwrap(),
            ) {
                Ok(s) => {
                    if !s.is_empty() {
                        ch.content = s;
                    }
                }
                Err(e) => eprintln!("failed to process chapter: {e:?}"),
            }
        });
        Ok(book)
    }
}

// ANCHOR: process_marp_header
fn process_marp_header(content: &String, title: &str, filename: &str) -> Result<String> {
    // check if it's actually a marp file
    let marp_true = Regex::new(r"marp:\s*true").unwrap();
    if !marp_true.is_match(content.as_str()) {
        return Ok(String::new());
    }

    let mut events = vec![];
    let mut in_header = false;

    let slide_target = format!("../slides/{}.html", &filename);
    let pdf_target = format!("../pdfs/{}.pdf", &filename);

    for event in Parser::new_ext(&content, Options::all()) {
        if event == Event::Start(Tag::MetadataBlock(MetadataBlockKind::YamlStyle)) {
            in_header = true;
        }

        // remove the header for now, but might want to look for specific values
        if !in_header {
            events.push(event.clone());
        }

        if event == Event::End(TagEnd::MetadataBlock(MetadataBlockKind::YamlStyle)) {
            in_header = false;

            // add the page heading
            events.push(Event::Start(Tag::Heading {
                level: HeadingLevel::H1,
                id: None,
                classes: vec![],
                attrs: vec![],
            }));

            events.push(Event::Text(CowStr::Borrowed(title)));
            events.push(Event::End(TagEnd::Heading(HeadingLevel::H1)));

            // add the link to the html slides
            events.push(Event::Start(Tag::Link {
                link_type: LinkType::Inline,
                dest_url: CowStr::Borrowed(&slide_target),
                title: CowStr::Borrowed(""),
                id: CowStr::Borrowed(""),
            }));
            events.push(Event::Text(CowStr::Borrowed("HTML Slides")));
            events.push(Event::End(TagEnd::Link));
            events.push(Event::Text(CowStr::Borrowed(" | ")));

            // and the pdf slides
            events.push(Event::Start(Tag::Link {
                link_type: LinkType::Inline,
                dest_url: CowStr::Borrowed(&pdf_target),
                title: CowStr::Borrowed(""),
                id: CowStr::Borrowed(""),
            }));
            events.push(Event::Text(CowStr::Borrowed("PDF Slides")));
            events.push(Event::End(TagEnd::Link));
            events.push(Event::SoftBreak);
        }
    }

    // insert the link to the slides

    let mut buf = String::with_capacity(content.len());

    Ok(pulldown_cmark_to_cmark::cmark(events.iter(), &mut buf).map(|_| buf)?)
}
// ANCHOR_END: process_marp_header

pub fn handle_preprocessing() -> Result<()> {
    let pre = Marp;
    let (ctx, book) = mdbook_preprocessor::parse_input(io::stdin())?;

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn no_marp() {
        let contents = "---\nmarp: false\n---\n# Fake Title\n";
        let result = process_marp_header(&contents.to_string(), "Test", "something/there.md");
        assert!(!result.is_err());
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn marp_present() {
        let contents = "---\nmarp: true\n---\n# Fake Title\n";
        let result = process_marp_header(&contents.to_string(), "Test", "something/there.md");
        assert!(!result.is_err());
        assert_eq!(result.unwrap().trim(), "# Test\n\n[HTML Slides](../slides/something/there.md.html) | [PDF Slides](../pdfs/something/there.md.pdf)\n# Fake Title");
    }
}

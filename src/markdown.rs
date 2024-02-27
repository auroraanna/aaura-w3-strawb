use std::{
    path::Path,
    fs::read_to_string
};
use pulldown_cmark::{
    Parser,
    Options,
    Event,
    CowStr,
    Tag
};
use pulldown_cmark_frontmatter::FrontmatterExtractor;
use maud::{
    html,
    Markup,
    PreEscaped
};
use slug::slugify;
use crate::base::{
    base,
    Frontmatter
};

pub async fn page_from_md(md_path: &Path) -> Markup {
    let md = read_to_string(md_path).unwrap();

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(&md, options);

    let mut heading_level = 0;
    let parser = parser.filter_map(| event |
        match event {
            Event::Start(Tag::Heading(level, ..)) => {
                let level = level as u8;
                heading_level = level;
                None
            },
            Event::Text(text) => {
                if heading_level != 0 {
                    let slug = slugify(&text);
                    let new_heading = Event::Html(CowStr::from(format!(
                        "<h{heading_level} id=\"{slug}\">{text}<a href=\"#{slug}\" aria-label=\"Anchor link for the heading: {text}\">🔗</a></h{heading_level}>"
                    )));
                    heading_level = 0;
                    return Some(new_heading);
                }
                Some(Event::Text(text))
            },
            _ => Some(event),
        }
    );

    let mut extractor = FrontmatterExtractor::new(parser);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, &mut extractor);
    let code_block = extractor.frontmatter.unwrap().code_block.unwrap();
    let frontmatter: Frontmatter = toml::from_str(&code_block.source).unwrap();
    eprintln!("{}", frontmatter.title);

    base(Some(frontmatter), html! {
        (PreEscaped(html_output))
    }).await
}
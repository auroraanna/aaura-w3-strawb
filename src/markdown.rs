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
    MyFrontmatter
};
use axum::{
    extract,
    body::Body
};
use http::status::StatusCode;

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

    let extractor = FrontmatterExtractor::new(parser);

    let code_block = extractor.frontmatter.as_ref().unwrap().code_block.as_ref().unwrap();
    let my_frontmatter: MyFrontmatter = toml::from_str(&code_block.source).unwrap();
    eprintln!("Markdown page title: {}", my_frontmatter.title);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, extractor);

    base(Some(my_frontmatter), html! {
        (PreEscaped(html_output))
    }).await
}

use axum_macros::debug_handler;

#[debug_handler]
pub async fn handle_md(extract::Path(filename): extract::Path<String>) -> Result<Body, StatusCode> {
    eprintln!("{}", filename);

    let md_path = Path::new(&format!("markdown/{}.md", filename)).to_owned();
    if !&md_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(page_from_md(&md_path).await.into_string().into())
}
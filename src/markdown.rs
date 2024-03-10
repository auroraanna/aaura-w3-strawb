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

struct MdPage {
    frontmatter: MyFrontmatter,
    html: String
}

impl MdPage {
    fn new(md_path: &Path) -> Self {
        dbg!(&md_path);
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
    
        let my_frontmatter: MyFrontmatter;
        let mut html_output = String::new();
        {
            let mut extractor = FrontmatterExtractor::new(parser);
    
            let code_block = extractor.extract_buffered().unwrap().code_block.as_ref().unwrap();
            my_frontmatter = toml::from_str(&code_block.source).unwrap();
            eprintln!("Markdown page title: {}", my_frontmatter.title);
                    
            pulldown_cmark::html::push_html(&mut html_output, extractor);
        }
    
        MdPage {
            frontmatter: my_frontmatter,
            html: html_output
        }
    }
}

use axum_macros::debug_handler;

#[debug_handler]
pub async fn handle_md(extract::Path(query_path): extract::Path<String>) -> Result<Body, StatusCode> {
    dbg!(&query_path);
    let stripped_query_path = query_path.strip_suffix("/").unwrap();
    let path = if Path::new(&("markdown/".to_owned() + stripped_query_path + ".md")).exists() {
        stripped_query_path.to_owned()
    } else {
        query_path + "index"
    };
    dbg!(&path);
    let internal_md_path = Path::new(&format!("markdown/{}.md", path)).to_owned();
    if !&internal_md_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let md_page = MdPage::new(&internal_md_path);
    Ok(base(Some(md_page.frontmatter), html! {
        (PreEscaped(md_page.html))
    }).await.into_string().into())
}

use std::{
    collections::HashMap,
    fs::{
        read_dir,
        read_to_string
    }, 
    path::Path,
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
    body::Body,
    response::{
        IntoResponse,
        AppendHeaders
    }
};
use http::{
    status::StatusCode,
    header::CONTENT_TYPE
};
use chrono::{
    DateTime,
    Utc,
};
use axum_macros::debug_handler;

#[derive(Debug, Clone)]
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

pub struct MdRoot {
    sub_dirs: HashMap<String, HashMap<String, MdPage>>,
    pages: HashMap<String, MdPage>,
    latest_date: DateTime<Utc>
}

impl MdRoot {
    fn new() -> Self {
        let mut md_root = MdRoot {
            sub_dirs: HashMap::new(),
            pages: HashMap::new(),
            latest_date: DateTime::default(),
        };

        for entry in read_dir("markdown").unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            dbg!(&path);
            if path.is_dir() {
                let sub_dir_name = path.file_name().unwrap();
                dbg!(sub_dir_name);
                let mut sub_dir_pages: HashMap<String, MdPage> = HashMap::new();
                for page in read_dir(&path).unwrap() {
                    let page = page.unwrap();
                    dbg!(page.path());
                    let md_page = if page.path().is_dir() {
                        MdPage::new(&page.path().join("index.md"))          
                    } else {
                        MdPage::new(&page.path())       
                    };
                    sub_dir_pages.insert(
                        page.path().file_stem().unwrap().to_str().unwrap().to_owned(), 
                        md_page
                    );
                }
                md_root.sub_dirs.insert(sub_dir_name.to_str().unwrap().to_owned(), sub_dir_pages);
            } else {
                md_root.pages.insert(
                    path.file_stem().unwrap().to_str().unwrap().to_owned(),
                    MdPage::new(&path)
                );
            }
        }

        eprintln!("{}", md_root.pages.get("contact").unwrap().frontmatter.title);
        eprintln!("{}", md_root.sub_dirs.get("blog").unwrap().get("i-bought-a-thinkpad").unwrap().frontmatter.title);
        eprintln!("{}", md_root.sub_dirs.get("blog").unwrap().get("starship-velociraptor-and-amazing-album").unwrap().frontmatter.title);

        let mut latest_date: DateTime<Utc> = DateTime::UNIX_EPOCH;
        for (_md_dir_name, md_dir) in md_root.sub_dirs.iter() {
            for (_md_page_name, md_page) in md_dir.iter() {
                if md_page.frontmatter.date_published.unwrap() > latest_date {
                    latest_date = md_page.frontmatter.date_published.unwrap();
                }
            }
        }
        md_root.latest_date = latest_date;

        md_root
    }
}

lazy_static::lazy_static! {
    pub static ref MAUD_VERSION: String = cargo_metadata::MetadataCommand::new().exec().unwrap().packages.iter().find_map(|package| {
        if package.name == "maud" {
            Some(package.version.to_string())
        } else {
            None
        }
    }).unwrap();
    pub static ref MD_ROOT: MdRoot = MdRoot::new();
}

#[debug_handler]
pub async fn handle_top_lvl_md_page(
    extract::Path(md_page): extract::Path<String>
) -> impl IntoResponse {
    eprintln!("handle_top_lvl_md_page");
    dbg!(&md_page);

    for (md_dir_name, md_dir) in MD_ROOT.sub_dirs.iter() {
        if md_dir_name == &md_page {
            return md_page_list(md_dir_name).await.into_response()
        }
    }

    let md_page = MD_ROOT.pages
        .get(&md_page).unwrap().clone();

    base(
        Some(md_page.frontmatter),
        html! { (PreEscaped(md_page.html)) }
    ).await.into_response()
}

#[debug_handler]
pub async fn handle_sub_lvl_md_page(
    extract::Path((md_dir, md_page)): extract::Path<(String, String)>
) -> impl IntoResponse {
    eprintln!("handle_sub_lvl_md_page");
    dbg!(&md_page);

    let md_page = MD_ROOT.sub_dirs.get(&md_dir).unwrap()
        .get(&md_page).unwrap().clone();


    return (
        StatusCode::OK,
        base(
            Some(md_page.frontmatter),
            html! { (PreEscaped(md_page.html)) }
        ).await
    );
}

pub async fn md_page_list(md_dir: &str) -> impl IntoResponse {
    base(Some(MyFrontmatter {
        title: md_dir.to_string(),
        date_published: None,
        description: Some(format!("A list of all posts under /{md_dir}/.")),
        keywords: None
    }), html! {
        ol .md_dir_list {
            @for (key, val) in MD_ROOT.sub_dirs.get(md_dir).unwrap().iter() {
                li {
                    @let formatted_date = val.frontmatter.date_published.unwrap().format("%Y-%m-%d");
                    time datetime=(formatted_date) {
                        (formatted_date)
                    }
                    a href=(key.to_owned() + "/") { (val.frontmatter.title) }
                }
            }
        }
    }).await
}

pub async fn atom_feed() -> impl IntoResponse {
    (
        AppendHeaders([
            (CONTENT_TYPE, "application/atom+xml; charset=utf8")
        ]),
        // html! can also be used to generate XML with this hack.
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>".to_owned() + &html! {
            feed xmlns="http://www.w3.org/2005/Atom" xml:lang="en" xml:base=(crate::BASE_URL) {
                id { (crate::BASE_URL) }
                title { "Anna Aurora" }
                //updated { (MD_ROOT.lock().unwrap().latest_date.to_rfc3339()) }
                updated { (MD_ROOT.latest_date.to_rfc3339()) }
                author {
                    name { "Anna Aurora" }
                    email { "anna@annaaurora.eu" }
                    uri { (crate::BASE_URL) }
                }
                link rel="self" href=(crate::BASE_URL.to_owned() + "atom.xml") type="application/atom+xml" title="Atom feed for Anna Aurora's website" {}
                link rel="related" href=(crate::BASE_URL) type="text/html" title="Anna Aurora's website" {}
                generator uri="https://maud.lambda.xyz/" version=(MAUD_VERSION.clone()) { "Maud" }
                icon { "static/favicon.webp" }
                subtitle { "The feed for pages on Anna Aurora's website containing flow text or artwork that have an associated publication date, e.g. blog entries." }

                @for (md_dir_name, md_dir) in MD_ROOT.sub_dirs.iter() {
                    @for (md_page_name, md_page) in md_dir.iter() {
                        entry {
                            @let date = md_page.frontmatter.date_published.unwrap().to_rfc3339();
                            @let page_url = format!("{}{}/{}/", crate::BASE_URL, md_dir_name, md_page_name);

                            id { (page_url) }
                            title { (md_page.frontmatter.title) }
                            published { (date) }
                            updated { (date) }
                            summary { (md_page.frontmatter.description.as_ref().expect("Markdown pages in subdirectories need to contain a description in their frontmatter.")) }
                            content type="text/html" src=(page_url) {}
                        }
                    }
                }
            }
        }.into_string()
    )
}
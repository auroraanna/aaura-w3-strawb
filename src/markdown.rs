use std::{
    collections::HashMap,
    fs::{
        read_dir,
        read_to_string
    }, 
    path::Path
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
    PreEscaped
};
use slug::slugify;
use crate::{
    base::{
        base,
        MyFrontmatter
    },
    ENV_VARS
};
use axum::{
    extract,
    body::Body,
    response::{
        Response,
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
use tower_http::services::fs::{
    ServeFile,
    ServeFileSystemResponseBody
};
use tower::Service;
use axum_macros::debug_handler;
use core::cmp::Ordering;
use indexmap::map::IndexMap;

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
        let mut inside_myaudio = false;
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
                Event::Start(Tag::Image(link_type, dest_url, title)) => {
                    let new_dest_url: CowStr<'_> = if dest_url.ends_with(".png") {
                        (dest_url.strip_suffix(".png").unwrap().to_owned() + "-lossier.webp").into()
                    } else if dest_url.ends_with(".webp") {
                        (dest_url.strip_suffix(".webp").unwrap().to_owned() + "-lossier.webp").into()
                    } else if dest_url.ends_with(".jpg") {
                        (dest_url.strip_suffix(".jpg").unwrap().to_owned() + "-lossier.jpg").into()
                    } else {
                        dest_url
                    };
                    Some(Event::Start(Tag::Image(link_type, new_dest_url, title)))
                },
                Event::Html(node) => {
                    Some(Event::Html(
                        if node.to_string() == "<myaudio>\n" {
                            inside_myaudio = true;
                            "".into()
                        } else if inside_myaudio {
                            inside_myaudio = false;
                            let src = node.to_string().strip_suffix(".flac\n").unwrap().to_owned() + "-lossier.opus";
                            format!("<audio controls=\"\" src=\"{}\">", src).into()
                        } else if node.to_string() == "</myaudio>\n" {
                            "</audio>\n".into()
                        } else {
                            node
                        }
                    ))
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

    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.frontmatter.date_published;
        let b = other.frontmatter.date_published;

        if a < b {
            return Ordering::Greater;
        } else if a > b {
            return Ordering::Less;
        } else {
            // a isn't smaller or greater than b so they have to be equal.
            return Ordering::Equal;
        }
    }
}

pub struct MdRoot {
    sub_dirs: HashMap<String, IndexMap<String, MdPage>>,
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

        eprintln!("{:?}", ENV_VARS.web_data_dir.join("markdown"));
        for entry in read_dir(ENV_VARS.web_data_dir.join("markdown")).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            dbg!(&path);
            if path.is_dir() {
                let sub_dir_name = path.file_name().unwrap();
                dbg!(sub_dir_name);
                let mut sub_dir_pages: IndexMap<String, MdPage> = IndexMap::new();
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

        for (_md_dir_name, md_dir) in md_root.sub_dirs.iter_mut() {
            md_dir.sort_by(|_page_a_key, page_a, _page_b_key, page_b| page_a.cmp(page_b));
        }

        eprintln!("{}", md_root.pages.get("contact").unwrap().frontmatter.title);
        eprintln!("{}", md_root.sub_dirs.get("blog").unwrap().get("i-bought-a-thinkpad").unwrap().frontmatter.title);
        eprintln!("{}", md_root.sub_dirs.get("blog").unwrap().get("starship-velociraptor-an-amazing-album").unwrap().frontmatter.title);

        // Pick latest of each md_dir (already sorted) and sort those.
        let mut latest_date_per_md_dir: Vec<DateTime<Utc>> = Vec::new();
        for (_md_dir_name, md_dir) in md_root.sub_dirs.iter() {
            let mut i = 0;
            for (_md_page_name, md_page) in md_dir.iter() {
                i += 1;
                if i == 1 {
                    latest_date_per_md_dir.push(md_page.frontmatter.date_published.unwrap());
                }
            }
        }
        latest_date_per_md_dir.sort();
        md_root.latest_date = latest_date_per_md_dir[latest_date_per_md_dir.len() - 1];

        md_root
    }
}

lazy_static::lazy_static! {
    pub static ref MAUD_VERSION: String = include_str!(concat!(
        env!("OUT_DIR"),
        "/maud_version.txt"
    )).to_owned();
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

pub async fn handle_md_media(
    req: extract::Request<Body>
) -> Result<Response<ServeFileSystemResponseBody>, Response<Body>> {
    let mut rel_path_string = urlencoding::decode(&req.uri().to_string()).unwrap().into_owned();
    rel_path_string.remove(0);
    let rel_path = Path::new(&rel_path_string);

    let folder = ENV_VARS.web_data_dir.join("markdown");
    let file_name = rel_path.file_name().unwrap();
    let path = Path::new(&folder)
        .join(rel_path.parent().unwrap())
        .join(file_name);
    eprintln!("{}", path.display());

    match ServeFile::new(path).call(req).await {
        Ok(res) => Ok(res),
        Err(e) => Err((
            StatusCode::NOT_FOUND,
            format!("{}", e)
        ).into_response())
    }
}

pub async fn md_page_list(md_dir: &str) -> impl IntoResponse {
    base(Some(MyFrontmatter {
        atom_id_parts: None,
        title: md_dir.to_string(),
        date_published: None,
        date_published_time_precision: None,
        description: Some(format!("A list of all posts under /{md_dir}/.")),
        keywords: None
    }), html! {
        ol .md_dir_list {
            @for (key, val) in MD_ROOT.sub_dirs.get(md_dir).unwrap().iter() {
                li {
                    @let formatted_date = val.frontmatter.date_published.unwrap().format(
                        val.frontmatter.human_date_format_placeholder()
                    );
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
                updated { (MD_ROOT.latest_date.to_rfc3339()) }
                author {
                    name { "Anna Aurora" }
                    email { "anna@annaaurora.eu" }
                    uri { (crate::BASE_URL) }
                }
                link rel="self" href=(crate::BASE_URL.to_owned() + "atom.xml") type="application/atom+xml" title="Atom feed for Anna Aurora's website" {}
                link rel="related" href=(crate::BASE_URL) type="text/html" title="Anna Aurora's website" {}
                generator uri="https://maud.lambda.xyz/" version=(MAUD_VERSION.clone()) { "Maud" }
                icon { "/static/favicon.png" }
                subtitle { "The feed for pages on Anna Aurora's website containing flow text or artwork that have an associated publication date, e.g. blog entries." }

                @for (md_dir_name, md_dir) in MD_ROOT.sub_dirs.iter() {
                    @for (md_page_name, md_page) in md_dir.iter() {
                        entry {
                            @let date = md_page.frontmatter.date_published.unwrap();
                            @let page_url = format!("{}{}/{}/", crate::BASE_URL, md_dir_name, md_page_name);
                            @let atom_id_parts = md_page.frontmatter.atom_id_parts.as_ref().expect(
                                "Markdown pages in subdirectories nee dto contain an id in their frontmatter."
                            );
                            // Even though the email may change, it was definitely Anna's at the date following.
                            @let page_id = format!("tag:{},{}:{}", atom_id_parts.email, date.format("%F"), atom_id_parts.object);

                            id { (page_id) }
                            title { (md_page.frontmatter.title) }
                            published { (date.to_rfc3339()) }
                            updated { (date.to_rfc3339()) }
                            summary { (md_page.frontmatter.description.as_ref().expect("Markdown pages in subdirectories need to contain a description in their frontmatter.")) }
                            content type="html" src=(page_url) {}
                        }
                    }
                }
            }
        }.into_string()
    )
}

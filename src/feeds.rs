use axum::response::{
    IntoResponse,
    AppendHeaders
};
use http::header::CONTENT_TYPE;
use maud::{
    html,
    Markup
};
use crate::markdown::{
    MAUD_VERSION,
    MD_ROOT
};
use serde::Serialize;

#[derive(Serialize)]
struct Author {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<String>
}

#[derive(Serialize)]
struct Hub {
    _type: String,
    url: String,
}

#[derive(Serialize)]
struct Attachment {
    url: String,
    mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size_in_bytes: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration_in_seconds: Option<usize>,
}

#[derive(Serialize)]
struct Item {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    // at least of of html or text
    #[serde(skip_serializing_if = "Option::is_none")]
    content_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    banner_image: Option<String>,
    // Not sure how to specify in types that these must be formatted as RFC3339
    #[serde(skip_serializing_if = "Option::is_none")]
    date_published: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_modified: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authors: Option<Vec<Author>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attachments: Option<Vec<Attachment>>,
    // How to type extensions?
}

#[derive(Serialize)]
struct JsonFeed {
    version: String,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    home_page_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    feed_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    favicon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authors: Option<Vec<Author>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expired: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hubs: Option<Vec<Hub>>,
    items: Vec<Item>,
}

#[derive(Clone)]
pub struct RenderedFeeds {
    atom: String,
    json: String
}

lazy_static::lazy_static! {
    pub static ref FEEDS: RenderedFeeds = {
        let mut entries: Vec<Markup> = vec![];
        let mut items: Vec<Item> = vec![];
        for (md_dir_name, md_dir) in MD_ROOT.sub_dirs.iter() {
            for (md_page_name, md_page) in md_dir.iter() {
                let date = md_page.frontmatter.date_published.unwrap();
                let date_fmt_rfc3339 = date.to_rfc3339();
                let page_url = format!("{}{}/{}/", crate::BASE_URL, md_dir_name, md_page_name);
                let atom_id_parts = md_page.frontmatter.atom_id_parts.as_ref().expect(
                    "Markdown pages in subdirectories nee dto contain an id in their frontmatter."
                );
                // Even though the email may change, it was definitely Anna's at the date following.
                let page_id = format!("tag:{},{}:{}", atom_id_parts.email, date.format("%F"), atom_id_parts.object);
                let summary = md_page.frontmatter.description.as_ref().expect("Markdown pages in subdirectories need to contain a description in their frontmatter.");

                entries.push(
                    html! {
                        entry {
                            id { (page_id) }
                            title { (md_page.frontmatter.title) }
                            published { (date_fmt_rfc3339) }
                            updated { (date_fmt_rfc3339) }
                            summary { (summary) }
                            link rel="alternate" type="html" href=(page_url) {}
                            content type="html" { (md_page.html) }
                        }
                    }
                );

                items.push(
                    Item {
                        id: page_id,
                        url: Some(page_url),
                        external_url: None,
                        title: Some(md_page.frontmatter.title.clone()),
                        content_html: Some(md_page.html.clone()),
                        content_text: None,
                        summary: Some(summary.to_string()),
                        image: None,
                        banner_image: None,
                        date_published: Some(date.to_rfc3339()),
                        date_modified: None,
                        authors: None,
                        tags: md_page.frontmatter.keywords.clone(),
                        language: None,
                        attachments: None
                    }
                );
            }
        }

        let rendered_atom_feed =
        // html! can also be used to generate XML with this hack.
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>".to_owned() + &html! {
            feed xmlns="http://www.w3.org/2005/Atom" xml:lang="en" xml:base=(crate::BASE_URL) {
                id { (crate::BASE_URL) }
                title { "Anna Aurora Kitsüne" }
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
                subtitle { "A feed for pages on Anna Aurora's website containing flow text or artwork that have an associated publication date, e.g. blog entries." }
                @for entry in entries {
                    (entry)
                }
            }
        }.into_string();

        let jsonfeed = JsonFeed {
            version: String::from("https://jsonfeed.org/version/1.1"),
            title: String::from("Anna Aurora"),
            home_page_url: Some(String::from("https://annaaurora.eu/")),
            feed_url: Some(String::from("https://annaaurora.eu/feed.json")),
            description: Some(String::from("A feed for pages on Anna Aurora's website containing flow text or artwork that have an associated publication date, e.g. blog entries.")),
            user_comment: None,
            next_url: None,
            icon: Some(String::from("https://annaaurora.eu/static/favicons/512x512.png")),
            favicon: Some(String::from("https://annaaurora.eu/static/favicons/64x64.png")),
            authors: Some(vec![
                Author {
                    name: Some(String::from("Anna Aurora Kitsüne")),
                    url: Some(String::from("https://annaaurora.eu/")),
                    avatar: Some(String::from("https://annaaurora.eu/")),
                }
            ]),
            language: Some(String::from("en")),
            expired: Some(false),
            hubs: None,
            items
        };

        RenderedFeeds {
            atom: rendered_atom_feed,
            json: serde_json::to_string(&jsonfeed).unwrap()
        }
    };
}

pub async fn atom() -> impl IntoResponse {
    (
        AppendHeaders([
            (CONTENT_TYPE, "application/atom+xml; charset=utf8")
        ]),
        FEEDS.atom.clone()
    )
}

pub async fn json() -> impl IntoResponse {
    (
        AppendHeaders([
            (CONTENT_TYPE, "application/feed+json; charset=utf8")
        ]),
        FEEDS.json.clone()
    )
}

use axum::{
    response::{
        AppendHeaders,
        IntoResponse
    },
    body::Body
};
use http::header::CONTENT_SECURITY_POLICY;
use serde::Deserialize;
use maud::{
    html,
    DOCTYPE,
    Markup
};
use chrono::{
    DateTime,
    Utc
};
use crate::{
    header::header,
    footer::footer
};
use base64::{
    Engine,
    engine::general_purpose
};
use rand::{
    rngs::OsRng,
    RngCore
};

fn nonce() -> String {
    let mut rng = OsRng::default();
    let mut nonce = [0u8; 16];
    rng.fill_bytes(&mut nonce);
    general_purpose::STANDARD.encode(&nonce)
}

#[derive(Debug, Clone, Deserialize)]
pub struct MyFrontmatter {
    pub title: String,
    pub date_published: Option<DateTime<Utc>>,
    pub date_published_time_precision: Option<bool>,
    pub description: Option<String>,
    pub keywords: Option<Vec<String>>
}

impl MyFrontmatter {
    pub fn human_date_format_placeholder(&self) -> &str {
        match &self.date_published_time_precision {
            Some(p) => {
                if *p {
                    "%F %H:%M:%S%:z"
                } else {
                    "%F"
                }
            },
            None => "%F %H:%M:%S%:z",
        }
    }
}

pub async fn base(frontmatter: Option<MyFrontmatter>, content: Markup) -> impl IntoResponse {
    let nonce = nonce();

    let html = html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf8";
                meta author="Anna Aurora";
                meta generator=("aaura-w3-strawb");
                @match frontmatter {
                    Some(ref fm) => {
                        title { (format!("Anna Aurora's website - {}", fm.title)) }
                        @match &fm.description {
                            Some(desc) => {
                                meta name="description" content=(desc);
                            },
                            None => {}
                        }
                        @match &fm.keywords {
                            Some(keyw) => {
                                meta name="keywords" content=(keyw.join(","));
                            },
                            None => {}
                        }
                    },
                    None => {
                        title { "Anna Aurora's website" }
                    }
                }
                meta name="author" content="Anna Aurora";
                meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=5";
                link rel="stylesheet" href="/static/global.css";
                link rel="icon" type="image/png" sizes="36x30" href="/static/favicon.png";
            }
            body {
                (header(&nonce).await)
                main {
                    @match frontmatter {
                        Some(ref fm) => {
                            @if fm.title != "Index" {
                                h1 { (fm.title) }
                            }
                            @match fm.date_published {
                                Some(date) => {
                                    p {
                                        "Published at "
                                        time datetime=(format!("{}", date.to_rfc3339())) { (format!("{}", date.format(
                                            fm.human_date_format_placeholder()
                                        ))) }
                                    }
                                },
                                None => {}
                            }
                        },
                        None => {}
                    }
                    (content)
                }
                (footer().await)
            }
        }
    }.into_string();

    (
        AppendHeaders([(
            CONTENT_SECURITY_POLICY,
            format!("{} style-src 'self' 'nonce-{}';", crate::COMMON_CSP.to_owned(), nonce)
        )]),
        Body::new(html)
    )
}

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

#[derive(Deserialize)]
pub struct MyFrontmatter {
    pub title: String,
    pub date_published: Option<DateTime<Utc>>
}

pub async fn base(frontmatter: Option<MyFrontmatter>, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf8";
                meta author="Anna Aurora";
                meta generator=("annaaurora.eu-cranberry");
                @match frontmatter {
                    Some(ref fm) => {
                        title { (format!("Anna Aurora's website - {}", fm.title)) }
                    },
                    None => {
                        title { "Anna Aurora's website" }
                    }
                }
                meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=5";
                link rel="stylesheet" href="/static/global.css";
            }
            body {
                (header().await)
                main {
                    @match frontmatter {
                        Some(ref fm) => {
                            h1 { (fm.title) }
                            @match fm.date_published {
                                Some(date) => {
                                    p {
                                        "Published at "
                                        time datetime=(format!("{}", date.to_rfc3339())) { (format!("{}", date.format("%F %H:%M:%S%:z"))) }
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
    }
}
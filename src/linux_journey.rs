use axum::response::IntoResponse;
use chrono::{
    naive::NaiveDate,
    Utc
};
use maud::{
    html,
    Markup
};
use std::{
    path::Path,
    fs::read_to_string
};
use serde::Deserialize;
use crate::{
    base::{
        base,
        MyFrontmatter
    },
    ENV_VARS
};

#[derive(Deserialize)]
struct Distro {
    name: String,
    website_url: String
}

#[derive(Deserialize)]
struct Date {
    datetime: NaiveDate,
    approximate: bool
}

#[derive(Deserialize)]
struct JourneyEntry {
    date_installed: Date,
    distro: Distro
}

pub async fn linux_journey() -> impl IntoResponse {
    let journey_json_string = read_to_string(Path::new(&ENV_VARS.web_data_dir).join("static/linux-journey.json")).unwrap();
    let journey: Vec<JourneyEntry> = serde_json::from_str(&journey_json_string).unwrap();

    base(
        Some(MyFrontmatter {
            atom_id_parts: None,
            title: "Linux journey".to_string(),
            date_published: None,
            date_published_time_precision: None,
            description: Some("Anna Aurora's personal timeline of Linux distributions that she's used, including information on when she started using them and for how long she used them.".to_string()),
            keywords: Some(vec![
                "linux-distros".to_owned(),
                "timeline".to_owned(),
                "personal-linux-journey".to_owned()
            ])
        }),
        html! {
            p { "My personal timeline of Linux distributions that I've used." }
            table {
                thead {
                    tr {
                        th { "Date intalled" }
                        th { "Duration used (d)" }
                        th { abbr title="operating system" { "OS" } }
                    }
                }
                tbody {
                    @for (i, entry) in journey.iter().enumerate() {
                        tr {
                            td {
                                @if entry.date_installed.approximate {
                                    "≅"
                                } @else {
                                    "="
                                }
                                (entry.date_installed.datetime)
                            }
                            td {({
                                let next_date: NaiveDate = if i == (journey.len() - 1) {
                                    Utc::now().date_naive()
                                } else {
                                    journey[i + 1].date_installed.datetime
                                };
                                (next_date - entry.date_installed.datetime).num_days()
                            })}
                            td {
                                a href=(entry.distro.website_url) {
                                    (entry.distro.name)
                                }
                            }
                        }
                    }
                }
            }
        }
    ).await
}

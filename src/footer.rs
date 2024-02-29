use maud::{
    html,
    Markup
};
use std::fs::read_to_string;
use crate::ENV_VARS;

async fn ring() -> Vec<String> {
    // If there is no path given, get the json from the official download
    let string_ring: String = match &ENV_VARS.bcdg_json {
        None => reqwest::get("https://artemislena.eu/services/downloads/beCrimeDoGay.json").await.unwrap().text().await.unwrap(),
        Some(p) => read_to_string(p).unwrap()
    };
    serde_json::from_str::<Vec<String>>(&string_ring).unwrap()
}

pub async fn footer() -> Markup {
    let self_url = "https://annaaurora.eu/webrings/be-crime-do-gay-webring";
    let ring = ring().await;
    let self_index = ring.iter().position(|x| x == self_url).unwrap();
    let prev_url: &str;
    let next_url: &str;
    if self_index + 1 == ring.len() {
        next_url = &ring[0];
        prev_url = &ring[self_index - 1];
    } else if self_index == 0 {
        next_url = &ring[self_index + 1];
        prev_url = &ring[ring.len() - 1];
    } else {
        next_url = &ring[self_index + 1];
        prev_url = &ring[self_index - 1];
    }

    html! {
        footer {
            div #be_crime_do_gay {
                span { "💸🔥 🏳️‍🌈 Be crime do gay webring" }
                nav {
                    a href=(prev_url) { "Go left" }
                    a href=(next_url) { "Go right" }
                }
            }
            a #th50_kb_club href="https://250kb.club/annaaurora-eu" {
                div {
                    div {
                        span { "cute" } br; span { "member" }
                    }
                }
                div {
                    span { "250kB Club" }
                }
            }
            a #fh12_kb_club href="https://512kb.club" {
                div {
                    span { "512KB Club" }
                }
                div {
                    span { "Green Team" }
                }
            }
            a #one_mb_club href="https://1mb.club/" { "1MB Club" }
        }
    }
}
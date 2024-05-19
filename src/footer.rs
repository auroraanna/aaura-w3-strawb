use maud::{
    html,
    Markup
};
use std::{
    fs::read_to_string,
    env::current_exe
};
use crate::ENV_VARS;

fn ring() -> Vec<String> {
    let string_ring: String = read_to_string(&ENV_VARS.bcdg_json).unwrap();
    serde_json::from_str::<Vec<String>>(&string_ring).unwrap()
}

pub async fn footer() -> Markup {
    let self_url = &(crate::BASE_URL.strip_suffix("/").unwrap());
    let ring = ring();
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
            div #footer_text {
                p {
                    "You were served by "
                    code { (format!("{}", current_exe().unwrap().display())) }
                    "."
                }
                p {
                    "Using "
                    a href="https://maud.lambda.xyz/" { "maud" }
                    " for HTML templating."
                }
            }
        }
    }
}

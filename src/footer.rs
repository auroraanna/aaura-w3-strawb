use maud::{
    html,
    Markup
};

pub async fn footer() -> Markup {
    let self_url = "https://annaaurora.eu/webrings/be-crime-do-gay-webring";
    let ring = reqwest::get("https://artemislena.eu/services/downloads/beCrimeDoGay.json").await.unwrap()
        .json::<Vec<String>>().await.unwrap();
    let self_index = ring.iter().position(|x| x == self_url).unwrap();
    let mut prev_url = "";
    let mut next_url = "";
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
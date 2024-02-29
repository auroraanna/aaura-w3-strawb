mod base;
mod header;
mod footer;
mod markdown;
mod linux_journey;

use std::{
    env,
    path::{
        Path,
        PathBuf
    }
};
use maud::{
    html,
    Markup
};
use tower_http::services::{
    ServeDir,
    ServeFile
};
use fontconfig::Fontconfig;
use axum::{
    routing::get,
    Router
};
use crate::{
    base::base,
    markdown::page_from_md,
    linux_journey::linux_journey
};

struct EnvVars {
    bind_address: String,
    bcdg_json: Option<PathBuf>
}

impl EnvVars {
    fn new() -> Self {
        let bind_address_key = "ANNAAURORA_EU_CRANBERRY_BIND_ADDRESS";
        let bind_address = match env::var(bind_address_key) {
            Ok(address) => address,
            Err(e) => {
                let address = "localhost:60021".to_string();
                eprintln!("{}: {}, using {} instead", bind_address_key, e, address);
                address
            }
        };
    
        let bcdg_json_key: &str = "ANNAAURORA_EU_CRANBERRY_BCDG_JSON_PATH";
        let bcdg_json: Option<PathBuf> = match env::var(bcdg_json_key) {
            Ok(p) => Some(Path::new(&p).to_owned()),
            Err(e) => {
                eprintln!("{}: {}, it is very advisable that you set this for decent page load times", bcdg_json_key, e);
                None
            }
        };

        Self { bind_address, bcdg_json }
    }
}

lazy_static::lazy_static! {
    static ref ENV_VARS: EnvVars = EnvVars::new();
}

async fn index() -> Markup {
    base(None, html! { }).await
}

fn comic_neue_bold() -> PathBuf {
    let fc = Fontconfig::new().unwrap();
    let font = fc.find("Comic Neue", Some("Bold")).unwrap();
    eprintln!("{}", font.path.display());
    font.path
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .nest_service("/fonts/ComicNeue-Bold", ServeFile::new(&comic_neue_bold()))
        .nest_service("/static/", ServeDir::new("static"))
        .route("/license", get(page_from_md(Path::new("./markdown/license.md")).await))
        .route("/contact", get(page_from_md(Path::new("./markdown/contact.md")).await))
        .route("/linux-journey", get(linux_journey().await));

    let listener = tokio::net::TcpListener::bind(&ENV_VARS.bind_address).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

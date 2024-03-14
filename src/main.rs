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
    markdown::{
        MAUD_VERSION,
        MD_ROOT,
        handle_top_lvl_md_page,
        handle_sub_lvl_md_page,
        md_page_list
    },
    linux_journey::linux_journey
};

struct EnvVars {
    bind_address: String,
    bcdg_json: Option<PathBuf>,
    static_dir: PathBuf
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

        let static_dir_key = "ANNAAURORA_EU_CRANBERRY_STATIC_DIR";
        let static_dir = match env::var(static_dir_key) {
            Ok(p) => Path::new(&p).to_owned(),
            Err(e) => {
                eprintln!("{}, {}, using ./static instead", static_dir_key, e);
                Path::new("./static").to_owned()
            }
        };

        Self { bind_address, bcdg_json, static_dir }
    }
}

lazy_static::lazy_static! {
    static ref ENV_VARS: EnvVars = EnvVars::new();
}

async fn index() -> Markup {
    base(
        None,
        html! { 
            div #portrait {
                img src="/static/portrait-srgb-lossier-downscaled.jpg" alt="A photo of showing Anna Aurora outdoors from the top to her sholders. She is wearing fox ears, a choker and a grey tshirt. She is holding her right hand up to her shoulder in joy. She has her eyes closed and is smiling. The background contains the sky, mossy walls and trees." {}
            }

            ul .bio {
                li { "likes programming in Rust, Javascript" }
                li { "using NixOS and maintainer for nixpkgs" }
                li { "certified kittyfoxgirl​, likes cuddling, will hug a Blåhaj and hide her face with it sometimes" }
                li {
                    "polyamorous "
                    ruby {
                        span .pansexual { "\"gay\"" }
                        rp { "(" }
                        rt { "pansexual" }
                        rp { ")" }
                    }
                    " "
                    span .trans { "trans" }
                    " girl"
                }
                li { "does 3d art and drawing" }
                li { "experimenting with music with LMMS and a MIDI keyboard (need to repair)" }
                li { "interested in converting some of her devices to USB-C and designing her own hardware" }
            }

            div #shark { 
                div { "blå 🦈 🦈 haj" }
            }
        }
    ).await
}

fn comic_neue_bold() -> PathBuf {
    let fc = Fontconfig::new().unwrap();
    let font = fc.find("Comic Neue", Some("Bold")).unwrap();
    eprintln!("{}", font.path.display());
    font.path
}

#[tokio::main]
async fn main() {
    lazy_static::initialize(&ENV_VARS);
    lazy_static::initialize(&MD_ROOT);

    let app = Router::new()
        .route("/", get(index))
        .nest_service("/fonts/ComicNeue-Bold", ServeFile::new(&comic_neue_bold()))
        .nest_service("/static/", ServeDir::new(&ENV_VARS.static_dir))
        .route("/linux-journey/", get(linux_journey))
        .route("/blog/", get(md_page_list("blog", "Blog").await))
        .route("/:md_page/", get(handle_top_lvl_md_page))
        .route("/:md_dir/:md_page/", get(handle_sub_lvl_md_page))

    let listener = tokio::net::TcpListener::bind(&ENV_VARS.bind_address).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

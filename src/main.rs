mod base;
mod header;
mod footer;
mod markdown;
mod linux_journey;
mod etag;

use std::{
    env,
    path::{
        Path,
        PathBuf
    }
};
use maud::{
    DOCTYPE,
    html
};
use tower::builder::ServiceBuilder;
use tower_http::{
    services::{
        ServeDir,
        ServeFile
    },
    catch_panic::CatchPanicLayer,
    compression::CompressionLayer,
    set_header::SetResponseHeaderLayer
};
use fontconfig::Fontconfig;
use axum::{
    body::Body,
    response::{
        AppendHeaders,
        IntoResponse
    },
    routing::get,
    Router,
    middleware
};
use crate::{
    base::{
        base,
        MyFrontmatter
    },
    markdown::{
        MAUD_VERSION,
        MD_ROOT,
        handle_top_lvl_md_page,
        handle_sub_lvl_md_page,
        handle_md_media,
        atom_feed
    },
    linux_journey::linux_journey
};
use http::{
    header::{
        CACHE_CONTROL,
        CONTENT_SECURITY_POLICY,
        CONTENT_TYPE,
        LOCATION,
        X_CONTENT_TYPE_OPTIONS
    },
    HeaderValue,
    Request,
    StatusCode
};
use axum_macros::debug_handler;
use etag::apply_etag;

pub const BASE_URL: &str = "https://annaaurora.eu/";
pub const COMMON_CSP: &str = "default-src 'none'; font-src 'self'; img-src 'self'; media-src 'self'; base-uri 'self'; form-action 'none';";

struct EnvVars {
    bind_address: String,
    bcdg_json: PathBuf,
    web_data_dir: PathBuf
}

impl EnvVars {
    fn new() -> Self {
        let bind_address_key = "AAURA_W3_STRAWB_BIND_ADDRESS";
        let bind_address = match env::var(bind_address_key) {
            Ok(address) => address,
            Err(e) => {
                let address = "localhost:60021".to_string();
                eprintln!("{}: {}, using {} instead", bind_address_key, e, address);
                address
            }
        };
    
        let bcdg_json_key: &str = "AAURA_W3_STRAWB_BCDG_JSON_PATH";
        let bcdg_json: PathBuf = match env::var(bcdg_json_key) {
            Ok(p) => Path::new(&p).to_owned(),
            Err(e) => {
                let path = "/var/cache/fetch-with-etag/cache";
                eprintln!("{}: {}, using {} instead", bcdg_json_key, e, path);
                Path::new(path).to_owned()
            }
        };

        let web_data_dir_key = "AAURA_W3_STRAWB_WEB_DATA_DIR";
        let web_data_dir = match env::var(web_data_dir_key) {
            Ok(p) => Path::new(&p).to_owned(),
            Err(e) => {
                eprintln!("{}, {}, using /var/lib/aaura-w3-strawb instead", web_data_dir_key, e);
                Path::new("/var/lib/aaura-w3-strawb").to_owned()
            }
        };

        Self { bind_address, bcdg_json, web_data_dir }
    }
}

lazy_static::lazy_static! {
    static ref ENV_VARS: EnvVars = EnvVars::new();
}

async fn index() -> impl IntoResponse {
    base(
        Some(MyFrontmatter {
            atom_id_parts: None,
            title: "Index".to_string(),
            date_published: None,
            date_published_time_precision: None,
            description: Some("The start page of Anna Aurora's website describing her person and interests.".to_string()),
            keywords: None
        }),
        html! { 
            div #portrait {
                img src="/static/portrait-srgb-lossier-downscaled.jpg" alt="A photo of showing Anna Aurora outdoors from the top to her sholders. She is wearing fox ears, a choker and a grey tshirt. She is holding her right hand up to her shoulder in joy. She has her eyes closed and is smiling. The background contains the sky, mossy walls and trees." {}
            }

            ul .bio {
                li { "likes programming in Rust, Python, Javascript" }
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
    eprintln!("Using font at this path for Comic Neue: {}", font.path.display());
    font.path
}

#[debug_handler]
async fn redirect_to_dir(req: Request<Body>) -> impl IntoResponse {
    let path = req.uri().path().strip_prefix("/").unwrap();
    let redirect_url = BASE_URL.to_owned() + &path + "/";

    (
        StatusCode::MOVED_PERMANENTLY,
        AppendHeaders([
            (LOCATION, redirect_url.clone())
        ]),
        Body::new(
            html! {
                (DOCTYPE)
                html lang="en" {
                    body {
                        a href=(redirect_url) { "Moved permanently" }
                        "."
                    }
                }
            }.into_string()
        )
    )
}

async fn do_not_ads() -> impl IntoResponse {
    (
        AppendHeaders([
            (CONTENT_TYPE, "text/plain; charset=utf8")
        ]),
        Body::new(
            "placeholder.example.com, placeholder, DIRECT, placeholder"
            .to_string()
        )
    )
}

const SECS_IN_YEAR: usize = 60 * 60 * 24 * 365;

#[tokio::main]
async fn main() {
    lazy_static::initialize(&ENV_VARS);
    lazy_static::initialize(&MAUD_VERSION);
    lazy_static::initialize(&MD_ROOT);

    let app = Router::new()
        .nest_service("/fonts/ComicNeue-Bold", ServeFile::new(&comic_neue_bold()))
        .layer(SetResponseHeaderLayer::if_not_present(
            CACHE_CONTROL,
            HeaderValue::from_str(&format!("max-age={}, public", SECS_IN_YEAR)).unwrap()
        ))
        .route("/", get(index))
        .nest_service("/static/", ServeDir::new(&ENV_VARS.web_data_dir.join("static")))
        .route("/linux-journey/", get(linux_journey))
        .route("/:md_page", get(redirect_to_dir))
        .route("/:md_page/", get(handle_top_lvl_md_page))
        .route("/:md_dir/:md_page", get(redirect_to_dir))
        .route("/:md_dir/:md_page/", get(handle_sub_lvl_md_page))
        .route("/:md_dir/:md_page/:image", get(handle_md_media))
        .route("/atom.xml", get(atom_feed))
        .route("/ads.txt", get(do_not_ads))
        .route("/app-ads.txt", get(do_not_ads))
        .layer(ServiceBuilder::new()
            .layer(CatchPanicLayer::new())
            .layer(middleware::from_fn(apply_etag))
            .layer(CompressionLayer::new()
                .br(true)
                .gzip(true)
                .zstd(true)
            )
            .layer(SetResponseHeaderLayer::if_not_present(
                CONTENT_SECURITY_POLICY,
                HeaderValue::from_str(
                    &format!("{} style-src 'self';", COMMON_CSP)
                ).unwrap()
            ))
            .layer(SetResponseHeaderLayer::if_not_present(
                CACHE_CONTROL,
                HeaderValue::from_static("no-cache, public")
            ))
            .layer(SetResponseHeaderLayer::overriding(
                X_CONTENT_TYPE_OPTIONS,
                HeaderValue::from_static("no-sniff")
            ))
        );

    let listener = tokio::net::TcpListener::bind(&ENV_VARS.bind_address).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

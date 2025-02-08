use maud::{
    html,
    Markup,
};

pub async fn header(nonce: &str) -> Markup {
    html! {
        header {
            img #banner src="/static/banner-text-to-path.svg" alt="The name “Anna Aurora” in the font Comic Neue, a font looking similar to Comic Sans. “Anna” rotated 2° counter-clockwise and is placed over “Aurora” which is rotated by 2° clockwise. Both words are colored in the same gradient, starting with a light purple at the top left and ending in a light pink at the bottom right.";
            nav {
                // Some of these don't have alt text on purpose because there would be no
                // additional context added for what the content of the linked page is.
                a href="/" {
                    picture {
                        source srcset="/static/neofox_home.webp";
                        img src="/static/neofox_home.png" alt="";
                    }
                    "Home"
                }
                a href="/blog/" {
                    "Blog"
                }
                a href="/art/" { 
                    img src="/static/mutant-standard_sparkles.svg" alt="";
                    "Art portfolio"
                }
                a href="/services/" {
                    "Services"
                }
                a href="/contact/" {
                    picture {
                        source srcset="/static/neofox/neofox_wink.avif";
                        img src="/static/neofox/neofox_wink.png" alt="";
                    }
                    "Contact"
                }
                a href="/find-billy/" {
                    picture {
                        source srcset="/static/neobot.webp";
                        img src="/static/neobot.png" alt="a neobot";
                    }
                    "Find Billy!"
                }
                a href="/linux-journey/" {
                    picture {
                        source srcset="/static/xenia_icon.avif";
                        img src="/static/xenia_icon.png" alt="";
                    }
                    "Linux journey"
                }
                a href="https://ring.annaaurora.eu/" {
                    picture {
                        source srcset="/static/ring_0005.webp";
                        img src="/static/ring_0005.png" alt="a smooth, golden, thin toroid";
                    }
                    "ring"
                }
                a href="/license/" {
                    span aria-hidden="true" { "©" }
                    "License"
                }
                a href="https://codeberg.org/annaaurora/aaura-w3-strawb" {
                    "Source code"
                }
                a href="/atom.xml" {
                    "Atom feed"
                }
                a href="/feed.json" {
                    "JSON Feed"
                }
            }
        }
    }
}

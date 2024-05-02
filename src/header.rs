use maud::{
    html,
    Markup,
    PreEscaped
};

struct MenuEntry {
    href: String,
    name: String,
    before: Option<char>
}

struct Menu {
    entries: Vec<MenuEntry>
}

impl Menu {
    fn new() -> Self {
        Menu {
            entries: Vec::new(),
        }
    }

    fn add_entry(&mut self, href: &str, name: &str, before: Option<char>) {
        self.entries.push(
            MenuEntry { href: href.to_string(), name: name.to_string(), before }
        );
    }
}

pub async fn header(nonce: &str) -> Markup {
    let mut menu = Menu::new();
    menu.add_entry("/", "Home", Some('🏠'));
    menu.add_entry("/blog/", "Blog", Some('📜'));
    menu.add_entry("/art/", "Art portfolio", Some('🌠'));
    menu.add_entry("/services/", "Services", None);
    menu.add_entry("/contact/", "Contact", Some('👋'));
    menu.add_entry("/static/find-billy", "Find Billy!", Some('🤖'));
    menu.add_entry("/linux-journey/", "Linux journey", Some('🐧'));
    menu.add_entry("/license/", "License", Some('©'));
    menu.add_entry("https://codeberg.org/annaaurora/aaura-w3-strawb", "Source code", Some('📦'));
    menu.add_entry("/atom.xml", "Atom feed", None);

    html! {
        header {
            img #banner src="/static/banner-text-to-path.svg" alt="The name “Anna Aurora” in the font Comic Neue, a font looking similar to Comic Sans. “Anna” rotated 2° counter-clockwise and is placed over “Aurora” which is rotated by 2° clockwise. Both words are colored in the same gradient, starting with a light purple at the top left and ending in a light pink at the bottom right." {}
            nav {
                @for entry in &menu.entries {
                    a href=(entry.href) { (entry.name) }
                }
                style nonce=(nonce) {
                    @for (i, entry) in menu.entries.iter().enumerate() {
                        @match entry.before {
                            Some(content) => {
                                (PreEscaped(
                                    format!("header > nav a:nth-child({}) {{ --before: '{}'; }} ", i + 1, content)
                                ))
                            },
                            None => {}
                        }
                    }
                }
            }
        }
    }
}

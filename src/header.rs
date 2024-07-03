use maud::{
    html,
    Markup,
    PreEscaped
};

struct Before {
    content: char,
    hidden: bool
}

struct MenuEntry {
    href: String,
    name: String,
    before: Option<Before>
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

    fn add_entry(&mut self, href: &str, name: &str, before: Option<Before>) {
        self.entries.push(
            MenuEntry { href: href.to_string(), name: name.to_string(), before }
        );
    }
}

pub async fn header(nonce: &str) -> Markup {
    let mut menu = Menu::new();
    menu.add_entry("/", "Home", Some(Before { content: '🏠', hidden: true }));
    menu.add_entry("/blog/", "Blog", Some(Before { content: '📜', hidden: true }));
    menu.add_entry("/art/", "Art portfolio", Some(Before { content: '🌠', hidden: true }));
    menu.add_entry("/services/", "Services", None);
    menu.add_entry("/contact/", "Contact", Some(Before { content: '👋', hidden: true }));
    menu.add_entry("/find-billy/", "Find Billy!", Some(Before { content: '🤖', hidden: false }));
    menu.add_entry("/linux-journey/", "Linux journey", Some(Before { content: '🐧', hidden: true }));
    menu.add_entry("/license/", "License", Some(Before { content: '©', hidden: true }));
    menu.add_entry("https://codeberg.org/annaaurora/aaura-w3-strawb", "Source code", Some(Before { content: '📦', hidden: true }));
    menu.add_entry("/atom.xml", "Atom feed", None);
    menu.add_entry("/feed.json", "JSON Feed", None);

    html! {
        header {
            img #banner src="/static/banner-text-to-path.svg" alt="The name “Anna Aurora” in the font Comic Neue, a font looking similar to Comic Sans. “Anna” rotated 2° counter-clockwise and is placed over “Aurora” which is rotated by 2° clockwise. Both words are colored in the same gradient, starting with a light purple at the top left and ending in a light pink at the bottom right.";
            nav {
                @for entry in &menu.entries {
                    a href=(entry.href) {
                        @match &entry.before {
                            Some(b) => {
                                span aria-hidden=(b.hidden) {
                                    (b.content)
                                }
                            }
                            None => {}
                        }
                        (entry.name)
                    }
                }
            }
        }
    }
}

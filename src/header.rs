use maud::{
    html,
    Markup
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

pub async fn header() -> Markup {
    let mut menu = Menu::new();
    menu.add_entry("/", "Home", Some('🏠'));
    menu.add_entry("/blog/", "Blog", Some('📜'));
    menu.add_entry("/art", "Art portfolio", Some('🌠'));*/
    menu.add_entry("/static/find-billy", "Find Billy!", Some('🤖'));
    menu.add_entry("https://kaufkauflist.annaaurora.eu", "kaufkauflist", Some('🛒'));
    menu.add_entry("/contact/", "Contact", Some('👋'));
    menu.add_entry("/linux-journey/", "Linux journey", Some('🐧'));
    menu.add_entry("/license/", "License", Some('©'));
    menu.add_entry("https://codeberg.org/annaaurora/annaaurora.eu-cranberry", "Source code", Some('📦'));
    menu.add_entry("/atom.xml", "Atom feed", None);
    menu.add_entry("/feed.json", "JSON feed", None);*/

    html! {
        header {
            img #banner src="/static/banner-text-to-path.svg" alt="The name “Anna Aurora” in the font Comic Neue, a font looking similar to Comic Sans. “Anna” rotated 2° counter-clockwise and is placed over “Aurora” which is rotated by 2° clockwise. Both words are colored in the same gradient, starting with a light purple at the top left and ending in a light pink at the bottom right." {}
            nav {
                @for entry in menu.entries {
                    @match entry.before {
                        Some(content) => {
                            a href=(entry.href) style=(format!("--before: '{}';", content)) { (entry.name) }
                        },
                        None => {
                            a href=(entry.href) { (entry.name) }
                        }
                    }
                }
            }
        }
    }
}
use std::collections::HashSet;
use once_cell::sync::Lazy;

static ALLOWED_TAGS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut set = HashSet::new();
    set.insert("p");
    set.insert("br");
    set.insert("strong");
    set.insert("em");
    set.insert("a");
    set.insert("ul");
    set.insert("li");
    set
});

pub struct ContentSanitizer {
    strip_comments: bool,
    allow_data_attrs: bool,
}

impl ContentSanitizer {
    pub fn new() -> Self {
        Self {
            strip_comments: true,
            allow_data_attrs: false,
        }
    }

    pub fn sanitize(&self, content: &str) -> String {
        let mut result = super::XssMiddleware::sanitize_html(content);
        
        if self.strip_comments {
            result = result.replace("<!--", "<!--");
        }
        
        if !self.allow_data_attrs {
            result = result.replace("data-", "");
        }
        
        self.strip_disallowed_tags(&result)
    }
    
    fn strip_disallowed_tags(&self, content: &str) -> String {
        let mut result = String::with_capacity(content.len());
        let mut in_script = false;
        let mut chars = content.chars();
        
        while let Some(c) = chars.next() {
            match c {
                '<' => {
                    let tag: String = chars.clone()
                        .take_while(|&c| c != '>')
                        .collect();
                    
                    if ALLOWED_TAGS.contains(tag.trim_start_matches('/')) {
                        result.push('<');
                        result.push_str(&tag);
                        result.push('>');
                        chars.nth(tag.len());
                    } else if tag.starts_with("script") {
                        in_script = true;
                        chars.nth(tag.len());
                    }
                }
                _ if !in_script => result.push(c),
                _ => {}
            }
        }
        result
    }    }  
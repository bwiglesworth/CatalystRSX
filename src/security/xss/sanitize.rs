use std::collections::HashSet;
use once_cell::sync::Lazy;

static ALLOWED_TAGS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut set = HashSet::new();
    set.insert("p");
    set.insert("a");
    set.insert("br");
    set
});

pub struct ContentSanitizer;

impl ContentSanitizer {
    pub fn new() -> Self {
        Self
    }

    pub fn sanitize(&self, content: &str) -> String {
        // First encode HTML entities
        let encoded = content
            .replace("&", "&")
            .replace("<", "<")
            .replace(">", ">")
            .replace("\u{27}", "'")
            .replace("\\\"", "&quote;");

        let mut result = String::new();
        let mut in_script = false;
        let mut chars = encoded.chars().peekable();
        
        while let Some(c) = chars.next() {
            match c {
                '<' => {
                    let tag = chars.clone()
                        .take_while(|&c| c != '>')
                        .collect::<String>();
                    
                    if tag.starts_with("script") {
                        in_script = true;
                        chars.nth(tag.len());
                        continue;
                    }
                    
                    if tag.starts_with("/script") {
                        in_script = false;
                        chars.nth(tag.len());
                        continue;
                    }
                    
                    if let Some(clean_tag) = self.clean_tag(&tag) {
                        result.push('<');
                        result.push_str(&clean_tag);
                        result.push('>');
                    }
                    chars.nth(tag.len());
                }
                _ if !in_script => result.push(c),
                _ => {}
            }
        }
        result
    }

    fn clean_tag(&self, tag: &str) -> Option<String> {
        let parts: Vec<&str> = tag.split_whitespace().collect();
        if parts.is_empty() {
            return None;
        }

        let is_closing = parts[0].starts_with('/');
        let tag_name = parts[0].trim_start_matches('/');
        
        if !ALLOWED_TAGS.contains(tag_name) {
            return None;
        }

        // For anchor tags, preserve closing slash if present
        if tag_name == "a" {
            return Some(format!("{}{}", if is_closing { "/" } else { "" }, "a"));
        }

        // Strip dangerous attributes and URIs for other tags
        let safe_attrs: Vec<&str> = parts.into_iter()
            .filter(|&attr| {
                !attr.starts_with("on") && 
                !attr.contains("data:") &&
                !attr.contains("javascript:") &&
                !attr.contains("vbscript:")
            })
            .collect();

        Some(safe_attrs.join(" "))
    }}
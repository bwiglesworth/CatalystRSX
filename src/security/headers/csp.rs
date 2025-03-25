use std::collections::HashMap;

pub struct CSPBuilder {
    directives: HashMap<String, Vec<String>>
}

impl CSPBuilder {
    pub fn new() -> Self {
        let mut directives = HashMap::new();
        directives.insert("default-src".to_string(), vec!["'self'".to_string()]);
        directives.insert("script-src".to_string(), vec!["'self'".to_string()]);
        directives.insert("style-src".to_string(), vec!["'self'".to_string()]);
        directives.insert("img-src".to_string(), vec!["'self'".to_string()]);
        Self { directives }
    }

    pub fn build(&self) -> String {
        self.directives
            .iter()
            .map(|(key, values)| format!("{} {}", key, values.join(" ")))
            .collect::<Vec<String>>()
            .join("; ")
    }
}